use std::cmp::{max, min};

use futures::{stream, StreamExt, TryStreamExt};
use tempfile::TempDir;
use tokio::fs::{DirEntry, File};
use tokio_stream::wrappers::ReadDirStream;

use crate::{
    csv::TxRecord,
    model::{Transaction, TxId},
    Error, TransactCsv,
};

#[derive(Debug)]
pub struct TxBlockStore {
    temp_dir: TempDir,
}

impl TxBlockStore {
    /// Initializes a new transaction block store.
    pub fn try_new() -> Result<Self, Error> {
        let temp_dir = tempfile::tempdir().map_err(Error::BlockStoreDirCreate)?;
        Ok(Self { temp_dir })
    }

    /// Persists the given block of transactions in this store.
    pub async fn persist_block(&self, transactions: &[Transaction]) -> Result<(), Error> {
        // * find smallest and largest transaction id
        // * TODO (optimization): map to client,tx,amt -- only possible if we can only
        //   reverse one kind of transaction
        // * save as min_max.csv

        let tx_min = transactions
            .first()
            .expect("expected at least one transaction")
            .tx();
        let tx_max = transactions
            .last()
            .expect("expected at least one transaction")
            .tx();

        // Use `{tx_min}_{tx_max}.csv` as file name as transaction IDs are unique, and
        // the only way this can collide is when two blocks have the same tx_min and
        // tx_max values -- e.g. disputed transaction IDs perfectly align at the block
        // boundaries.
        let file_name = format!("{tx_min}_{tx_max}.csv");
        let file_path = self.temp_dir.path().join(&file_name);
        let block_file = File::create(&file_path)
            .await
            .map_err(Error::BlockFileCreate)?;

        let (mut block_writer, tx_min, tx_max) = stream::iter(
            // Only persist deposits and withdrawals, as they're the only transactions that may be
            // disputed
            transactions
                .iter()
                .filter(|transaction| matches!(transaction, Transaction::Deposit(_))),
        )
        .map(Result::<_, Error>::Ok)
        .try_fold(
            (TransactCsv::csv_writer(block_file), tx_min, tx_max),
            |(mut block_writer, tx_min, tx_max), transaction| async move {
                block_writer
                    .serialize(TxRecord::from(transaction.clone()))
                    .await
                    .map_err(Error::BlockTxWrite)?;

                Ok((
                    block_writer,
                    min(tx_min, transaction.tx()),
                    max(tx_max, transaction.tx()),
                ))
            },
        )
        .await?;
        block_writer.flush().await.map_err(Error::BlockFileFlush)?;
        let min_max_file_name = format!("{tx_min}_{tx_max}.csv");
        let min_max_file_path = self.temp_dir.path().join(&min_max_file_name);

        if file_name != min_max_file_name {
            tokio::fs::rename(&file_path, &min_max_file_path)
                .await
                .map_err(|error| Error::BlockFileRename {
                    from: file_name,
                    to: min_max_file_name,
                    error,
                })?;
        }

        Ok(())
    }

    /// Returns the transaction if found in this block store.
    ///
    /// This is a semi expensive operation.
    ///
    /// An optimization is to store disputed transactions and their amounts
    /// separately.
    pub async fn find_transaction(&self, tx: TxId) -> Result<Transaction, Error> {
        let block_transaction_match = tokio::fs::read_dir(self.temp_dir.path())
            .await
            .map(ReadDirStream::new)
            .map_err(Error::BlockStoreDirRead)?
            .map_err(Error::BlockStoreDirRead)
            .and_then(Self::parse_min_max_tx)
            .try_filter_map(|(dir_entry, tx_min, tx_max)| {
                // Checks if the block file may contain the transaction.
                let block_may_have_transaction = tx_min <= tx && tx_max >= tx;
                async move {
                    if block_may_have_transaction {
                        Ok(Some(dir_entry))
                    } else {
                        Ok(None)
                    }
                }
            })
            .and_then(|dir_entry| async move {
                // Stream the transaction block file.
                TransactCsv::stream(&dir_entry.path())
                    .await
                    .map(move |block_transactions| {
                        block_transactions.try_filter(move |transaction| {
                            // Only match deposits, because dispute/resolve/chargeback transactions
                            // don't carry amounts, and from the spec it doesn't appear that
                            // disputes apply to withdrawals.
                            //
                            // If you update here, also update the filter in
                            // `TxBlockStore::persist_block`
                            let tx_matches = matches!(transaction, Transaction::Deposit(_))
                                && transaction.tx() == tx;
                            async move { tx_matches }
                        })
                    })
            })
            .try_flatten();

        Box::pin(block_transaction_match)
            .next()
            .await
            .ok_or(Error::DisputeTxNotFound { tx })?
    }

    /// Returns the min and max transaction IDs associated with a dir entry.
    async fn parse_min_max_tx(dir_entry: DirEntry) -> Result<(DirEntry, TxId, TxId), Error> {
        let file_name = dir_entry.file_name();
        let file_name_lossy = file_name.to_string_lossy();
        let mut plain_name = file_name_lossy.splitn(2, '.');
        let file_name_invalid = || Error::BlockFileNameInvalid {
            file_name: file_name.clone(),
        };
        let file_name_invalid_err = |_| Error::BlockFileNameInvalid {
            file_name: file_name.clone(),
        };
        let mut split = plain_name
            .next()
            .ok_or_else(file_name_invalid)?
            .splitn(2, '_');
        let tx_min = TxId::from(
            split
                .next()
                .ok_or_else(file_name_invalid)?
                .parse::<u32>()
                .map_err(file_name_invalid_err)?,
        );
        let tx_max = TxId::from(
            split
                .next()
                .ok_or_else(file_name_invalid)?
                .parse::<u32>()
                .map_err(file_name_invalid_err)?,
        );

        Ok((dir_entry, tx_min, tx_max))
    }
}

use std::cmp::{max, min};

use futures::{stream, StreamExt, TryStreamExt};
use tempfile::TempDir;
use tokio::fs::File;

use crate::{csv::TxRecord, model::Transaction, Error, TransactCsv};

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
        // find smallest and largest transaction id
        // TODO: optimization: map to tx,amt
        // save as min_max.csv

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

        let (mut block_writer, tx_min, tx_max) = stream::iter(transactions.iter())
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

        tokio::fs::rename(&file_name, &min_max_file_name)
            .await
            .map_err(|error| Error::BlockFileRename {
                from: file_name,
                to: min_max_file_name,
                error,
            })?;

        Ok(())
    }
}
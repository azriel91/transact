#![deny(missing_docs, missing_debug_implementations)]
#![deny(clippy::float_arithmetic)] // prevent arithmetic overflow

//! Toy transaction library

// API

pub mod model;
pub use crate::error::Error;

// impl

use std::path::Path;

use futures::{
    stream::{self, TryChunksError, TryStreamExt},
    StreamExt,
};

use crate::{
    csv::TransactCsv,
    model::{Account, Accounts},
    tx_block_store::TxBlockStore,
    tx_processor::TxProcessor,
};

mod csv;
mod error;
mod tx_block_store;
mod tx_processor;

/// Number of transactions to store per transaction file.
///
/// These will be re-read to discover a transaction's amount when processing
/// disputed transactions.
const TX_BLOCK_SIZE: usize = 10000;

/// Processes transactions and outputs them to the given stream.
pub async fn process<W>(path: &Path, out_stream: W) -> Result<(), Error>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    let tx_block_store = &TxBlockStore::try_new()?;
    let transactions = TransactCsv::stream(path).await?;
    let accounts = transactions
        .try_chunks(TX_BLOCK_SIZE)
        .and_then(|transactions| async move {
            tx_block_store
                .persist_block(&transactions)
                .await
                .map_err(|e| TryChunksError(transactions.clone(), e))?;

            let stream = stream::iter(transactions.into_iter())
                .map(Result::<_, Error>::Ok)
                .map_err(|e| TryChunksError(Vec::new(), e));

            Ok(stream)
        })
        .try_flatten()
        // drop transactions when encountering an error
        .map_err(|TryChunksError(_transactions, e)| e)
        .try_fold(Accounts::new(), |mut accounts, transaction| async move {
            let account = accounts
                .entry(transaction.client())
                .or_insert_with(|| Account::empty(transaction.client()));

            TxProcessor::process(account, transaction)?;

            Ok(accounts)
        })
        .await?;

    let mut writer = stream::iter(accounts.into_values())
        .map(Result::<Account, Error>::Ok)
        .try_fold(
            TransactCsv::csv_writer(out_stream),
            |mut writer, account| async move {
                writer
                    .serialize(account)
                    .await
                    .map_err(Error::OutputWrite)?;

                Ok(writer)
            },
        )
        .await?;

    writer.flush().await.map_err(Error::OutputFlush)?;

    Ok(())
}

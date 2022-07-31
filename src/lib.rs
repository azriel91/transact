#![deny(missing_docs, missing_debug_implementations)]
#![deny(clippy::float_arithmetic)] // prevent arithmetic overflow

//! Toy transaction library

// API

pub mod model;
pub use crate::error::Error;

// impl

use std::path::Path;

use futures::{
    stream::{self, TryStreamExt},
    StreamExt,
};

use crate::{
    csv::TransactCsv,
    model::{Account, Accounts},
    tx_processor::TxProcessor,
};

mod csv;
mod error;
mod tx_processor;

/// Processes transactions and outputs them to the given stream.
pub async fn process<W>(path: &Path, out_stream: W) -> Result<(), Error>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    let transactions = TransactCsv::stream(path).await?;
    let accounts = transactions
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

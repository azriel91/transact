#![deny(missing_docs, missing_debug_implementations)]

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
};

mod csv;
mod error;

/// Processes transactions and outputs them to the given stream.
pub async fn process<W>(path: &Path, out_stream: W) -> Result<(), Error>
where
    W: std::io::Write,
{
    let transactions = TransactCsv::stream(path)?;
    let accounts = transactions
        .try_fold(Accounts::new(), |mut accounts, transaction| async move {
            let _account = accounts.entry(transaction.client()).or_insert_with(|| {
                let client = transaction.client();
                let available = 0.0;
                let held = 0.0;
                let total = 0.0;
                let locked = false;

                Account::new(client, available, held, total, locked)
            });

            Ok(accounts)
        })
        .await?;

    let mut writer = stream::iter(accounts.into_values())
        .map(Result::<Account, Error>::Ok)
        .try_fold(
            TransactCsv::csv_writer(out_stream),
            |mut writer, account| async move {
                writer.serialize(account).map_err(Error::OutputWrite)?;

                Ok(writer)
            },
        )
        .await?;

    writer.flush().map_err(Error::OutputFlush)?;

    Ok(())
}

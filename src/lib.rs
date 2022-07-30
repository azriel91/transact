#![deny(missing_docs, missing_debug_implementations)]

//! Toy transaction library

use std::path::Path;

use crate::model::Account;
use futures::stream::TryStreamExt;

use crate::csv::TransactCsv;

//

pub use crate::error::Error;

pub mod model;

mod csv;
mod error;

/// Processes transactions and outputs them to the given stream.
pub async fn process<W>(path: &Path, out_stream: W) -> Result<(), Error>
where
    W: std::io::Write,
{
    let transactions = TransactCsv::stream(path)?;
    let mut writer = transactions
        .map_ok(|transaction| {
            let client = transaction.client();
            let available = 0.0;
            let held = 0.0;
            let total = 0.0;
            let locked = false;

            Account::new(client, available, held, total, locked)
        })
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

use std::{fs::File, path::Path};

use futures::{stream, TryStream, TryStreamExt};

use crate::{csv::TxRecord, model::Transaction, Error};

/// Abstraction to call [`csv`] functions with suitable parameters.
#[derive(Debug)]
pub struct TransactCsv;

impl TransactCsv {
    /// Returns a [`TryStream`] of [`Transaction`]s.
    ///
    /// # Parameters
    ///
    /// * `path`: Path to the transactions CSV file.
    pub fn stream(path: &Path) -> Result<impl TryStream<Item = Result<Transaction, Error>>, Error> {
        Self::open(path).map(|csv_reader| {
            stream::iter(csv_reader.into_deserialize::<TxRecord>())
                .map_err(Error::TransactionDeserialize)
                .and_then(|tx_record| async { Transaction::try_from(tx_record) })
        })
    }

    /// Returns a [`csv::Reader`] to the transactions CSV.
    ///
    /// # Parameters
    ///
    /// * `path`: Path to the transactions CSV file.
    fn open(path: &Path) -> Result<csv::Reader<File>, Error> {
        csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true) // In case Dispute, Resolve, and Chargeback rows don't contain an empty column
            .from_path(path)
            .map_err(|error| Error::TransactCsvOpen {
                path: path.to_path_buf(),
                error,
            })
    }
}

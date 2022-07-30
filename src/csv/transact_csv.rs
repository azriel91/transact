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
    pub fn stream(path: &Path) -> Result<impl TryStream<Ok = Transaction, Error = Error>, Error> {
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
            .trim(csv::Trim::All)
            .from_path(path)
            .map_err(|error| Error::TransactCsvOpen {
                path: path.to_path_buf(),
                error,
            })
    }

    /// Returns a [`csv::Writer`].
    pub fn csv_writer<'f, W>(out_stream: W) -> csv::Writer<Box<dyn std::io::Write + 'f>>
    where
        W: std::io::Write + 'f,
    {
        csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(Box::new(out_stream))
    }
}

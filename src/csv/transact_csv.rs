use std::{path::Path, pin::Pin};

use futures::{TryStream, TryStreamExt};
use tokio::fs::File;

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
    pub async fn stream(
        path: &Path,
    ) -> Result<impl TryStream<Ok = Transaction, Error = Error>, Error> {
        Self::open(path).await.map(|csv_deserializer| {
            csv_deserializer
                .into_deserialize::<TxRecord>()
                .map_err(Error::TransactionDeserialize)
                .and_then(|tx_record| async { Transaction::try_from(tx_record) })
        })
    }

    /// Returns a [`csv_async::AsyncDeserializer`] to the transactions CSV.
    ///
    /// # Parameters
    ///
    /// * `path`: Path to the transactions CSV file.
    async fn open(path: &Path) -> Result<csv_async::AsyncDeserializer<File>, Error> {
        let file = File::open(path)
            .await
            .map_err(|error| Error::TransactCsvOpen {
                path: path.to_path_buf(),
                error,
            })?;
        let deserializer = csv_async::AsyncReaderBuilder::new()
            .has_headers(true)
            .flexible(true) // In case Dispute, Resolve, and Chargeback rows don't contain an empty column
            .trim(csv_async::Trim::All)
            .create_deserializer(file);

        Ok(deserializer)
    }

    /// Returns a [`csv_async::AsyncWriter`].
    pub fn csv_writer<'f, W>(
        out_stream: W,
    ) -> csv_async::AsyncSerializer<Pin<Box<dyn tokio::io::AsyncWrite + 'f>>>
    where
        W: tokio::io::AsyncWrite + Unpin + 'f,
    {
        csv_async::AsyncWriterBuilder::new()
            .has_headers(true)
            .flexible(true)
            .create_serializer(Box::pin(out_stream))
    }
}

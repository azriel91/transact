use std::{fmt, path::PathBuf};

/// Errors that happen during processing.
#[derive(Debug)]
pub enum Error {
    /// Error opening transactions CSV.
    TransactCsvOpen {
        /// Path to the CSV.
        path: PathBuf,
        /// Underlying CSV error.
        error: csv::Error,
    },
    /// Error deserializing a transaction.
    TransactionDeserialize(csv::Error),
    /// Error writing output.
    OutputWrite(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransactCsvOpen { path, .. } => {
                write!(f, "Error opening transactions CSV: {}", path.display())
            }
            Self::TransactionDeserialize(_) => write!(f, "Error deserializing a transaction."),
            Self::OutputWrite(_) => write!(f, "Error writing output"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TransactCsvOpen { error, .. } => Some(error),
            Self::TransactionDeserialize(error) => Some(error),
            Self::OutputWrite(error) => Some(error),
        }
    }
}

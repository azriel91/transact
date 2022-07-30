use std::{fmt, path::PathBuf};

use crate::model::{ClientId, TxId};

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
    /// Deposit amount not provided in transaction record.
    DepositAmountNotProvided {
        /// Client ID.
        client: ClientId,
        /// Transaction ID.
        tx: TxId,
    },
    /// Withdrawal amount not provided in transaction record.
    WithdrawalAmountNotProvided {
        /// Client ID.
        client: ClientId,
        /// Transaction ID.
        tx: TxId,
    },
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
            Self::DepositAmountNotProvided { client, tx } => write!(
                f,
                "Deposit amount not provided in transaction record for client {client}, transaction {tx}."
            ),
            Self::WithdrawalAmountNotProvided { client, tx } => write!(
                f,
                "Withdrawal amount not provided in transaction record for client {client}, transaction {tx}."
            ),
            Self::OutputWrite(_) => write!(f, "Error writing output"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TransactCsvOpen { error, .. } => Some(error),
            Self::TransactionDeserialize(error) => Some(error),
            Self::DepositAmountNotProvided { .. } => None,
            Self::WithdrawalAmountNotProvided { .. } => None,
            Self::OutputWrite(error) => Some(error),
        }
    }
}

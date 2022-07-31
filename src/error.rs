use std::{
    ffi::OsString,
    fmt,
    path::{Path, PathBuf},
};

use crate::model::{ClientId, TxId};

/// Errors relating to running the application / corrupt data.
#[derive(Debug)]
pub enum Error {
    /// Error creating directory to store transaction block files.
    BlockStoreDirCreate(std::io::Error),
    /// Error reading block store directory to find transaction.
    BlockStoreDirRead(std::io::Error),
    /// Error creating transaction block file.
    BlockFileCreate(std::io::Error),
    /// Error flushing output stream for a block file.
    BlockFileFlush(std::io::Error),
    /// Error flushing output stream for a block file.
    BlockFileRename {
        /// File name to rename from.
        from: String,
        /// File name to rename to.
        to: String,
        /// Underlying error.
        error: std::io::Error,
    },
    /// Block file name not in the format `min_max.csv`.
    BlockFileNameInvalid {
        /// Name of the file in the transaction block store.
        file_name: OsString,
    },
    /// Error writing transaction to a block file.
    BlockTxWrite(csv_async::Error),
    /// Error opening transactions CSV.
    TransactCsvOpen {
        /// Path to the CSV.
        path: PathBuf,
        /// Underlying CSV error.
        error: std::io::Error,
    },
    /// Error deserializing a transaction.
    TransactionDeserialize(csv_async::Error),
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
    OutputWrite(csv_async::Error),
    /// Error flushing output stream.
    OutputFlush(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockStoreDirCreate(_) => write!(
                f,
                "Error creating directory to store transaction block files."
            ),
            Self::BlockStoreDirRead(_) => write!(
                f,
                "Error reading block store directory to find transaction."
            ),
            Self::BlockFileCreate(_) => write!(f, "Error creating transaction block file."),
            Self::BlockFileFlush(_) => write!(f, "Error flushing output stream for a block file."),
            Self::BlockFileRename { from, to, .. } => {
                write!(f, "Error renaming block file from `{from}` to `{to}`.")
            }
            Self::BlockFileNameInvalid { file_name } => write!(
                f,
                "Block file name not in the format `min_max.csv`: {}",
                Path::new(file_name).display()
            ),
            Self::BlockTxWrite(_) => write!(f, "Error writing transaction to a block file."),
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
            Self::OutputFlush(_) => write!(f, "Error flushing output stream"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BlockStoreDirCreate(error) => Some(error),
            Self::BlockStoreDirRead(error) => Some(error),
            Self::BlockFileCreate(error) => Some(error),
            Self::BlockFileFlush(error) => Some(error),
            Self::BlockFileRename { error, .. } => Some(error),
            Self::BlockFileNameInvalid { .. } => None,
            Self::BlockTxWrite(error) => Some(error),
            Self::TransactCsvOpen { error, .. } => Some(error),
            Self::TransactionDeserialize(error) => Some(error),
            Self::DepositAmountNotProvided { .. } => None,
            Self::WithdrawalAmountNotProvided { .. } => None,
            Self::OutputWrite(error) => Some(error),
            Self::OutputFlush(error) => Some(error),
        }
    }
}

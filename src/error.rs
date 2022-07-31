use std::{fmt, path::PathBuf};

use rust_decimal::Decimal;

use crate::model::{ClientId, TxId};

/// Errors that happen during processing.
#[derive(Debug)]
pub enum Error {
    /// Error creating directory to store transaction block files.
    BlockStoreDirCreate(std::io::Error),
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
    /// Deposit transaction amount is negative.
    DepositAmountNegative {
        /// Client ID.
        client: ClientId,
        /// Transaction ID.
        tx: TxId,
        /// Amount in the transaction.
        amount: Decimal,
    },
    /// Deposit transaction would cause an account's available funds to
    /// overflow.
    DepositAvailableOverflow {
        /// Client ID.
        client: ClientId,
        /// Transaction ID.
        tx: TxId,
    },
    /// Deposit transaction would cause an account's total funds to overflow.
    DepositTotalOverflow {
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
    /// Withdrawal transaction amount is negative.
    WithdrawalAmountNegative {
        /// Client ID.
        client: ClientId,
        /// Transaction ID.
        tx: TxId,
        /// Amount in the transaction.
        amount: Decimal,
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
            Self::BlockFileCreate(_) => write!(f, "Error creating transaction block file."),
            Self::BlockFileFlush(_) => write!(f, "Error flushing output stream for a block file."),
            Self::BlockFileRename { from, to, .. } => {
                write!(f, "Error renaming block file from `{from}` to `{to}`.")
            }
            Self::BlockTxWrite(_) => write!(f, "Error writing transaction to a block file."),
            Self::TransactCsvOpen { path, .. } => {
                write!(f, "Error opening transactions CSV: {}", path.display())
            }
            Self::TransactionDeserialize(_) => write!(f, "Error deserializing a transaction."),
            Self::DepositAmountNotProvided { client, tx } => write!(
                f,
                "Deposit amount not provided in transaction record for client {client}, transaction {tx}."
            ),
            Self::DepositAmountNegative { client, tx, amount } => write!(
                f,
                "Deposit transaction amount is negative: client {client}, transaction {tx}, amount: {amount}."
            ),
            Self::DepositAvailableOverflow { client, tx } => write!(
                f,
                "Deposit transaction would cause an account's available funds to overflow: client {client}, transaction {tx}."
            ),
            Self::DepositTotalOverflow { client, tx } => write!(
                f,
                "A deposit transaction would cause an account's total funds to overflow: client {client}, transaction {tx}."
            ),
            Self::WithdrawalAmountNotProvided { client, tx } => write!(
                f,
                "Withdrawal amount not provided in transaction record for client {client}, transaction {tx}."
            ),
            Self::WithdrawalAmountNegative { client, tx, amount } => write!(
                f,
                "Withdrawal transaction amount is negative: client {client}, transaction {tx}, amount: {amount}."
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
            Self::BlockFileCreate(error) => Some(error),
            Self::BlockFileFlush(error) => Some(error),
            Self::BlockFileRename { error, .. } => Some(error),
            Self::BlockTxWrite(error) => Some(error),
            Self::TransactCsvOpen { error, .. } => Some(error),
            Self::TransactionDeserialize(error) => Some(error),
            Self::DepositAmountNotProvided { .. } => None,
            Self::DepositAmountNegative { .. } => None,
            Self::DepositAvailableOverflow { .. } => None,
            Self::DepositTotalOverflow { .. } => None,
            Self::WithdrawalAmountNotProvided { .. } => None,
            Self::WithdrawalAmountNegative { .. } => None,
            Self::OutputWrite(error) => Some(error),
            Self::OutputFlush(error) => Some(error),
        }
    }
}

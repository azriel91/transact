use std::fmt;

use rust_decimal::Decimal;

use crate::model::{ClientId, TxId};

/// Errors relating to invalid transactions.
#[derive(Debug, PartialEq, Eq)]
pub enum TxError {
    /// Account
    AccountLocked {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is not processed.
        tx: TxId,
    },
    /// Dispute transaction client ID does not match client ID of the disputed
    /// transaction.
    DisputeClientMismatch {
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Client ID of the `dispute` transaction.
        dispute_tx_client: ClientId,
        /// Client ID of the transaction that is disputed.
        disputed_tx_client: ClientId,
    },
    /// Dispute transaction not found in transaction block files.
    DisputeTxNotFound {
        /// Transaction ID that is disputed.
        tx: TxId,
    },
    /// Account does not have sufficient funds to hold in a dispute.
    DisputeInsufficientAvailable {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Amount client has available.
        available: Decimal,
        /// Amount that is disputed.
        amount: Decimal,
    },
    /// Account held amount would overflow for dispute.
    DisputeHeldOverflow {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Amount client has held.
        held: Decimal,
        /// Amount that is disputed.
        amount: Decimal,
    },
    /// Resolve transaction client ID does not match client ID of the disputed
    /// transaction.
    ResolveClientMismatch {
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Client ID of the `dispute` transaction.
        resolve_tx_client: ClientId,
        /// Client ID of the transaction that is disputed.
        disputed_tx_client: ClientId,
    },
    /// Account does not have sufficient funds to unhold in a dispute
    /// resolution.
    ResolveInsufficientHeld {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Amount client has held.
        held: Decimal,
        /// Amount that is disputed.
        amount: Decimal,
    },
    /// Account available amount would overflow for dispute resolution.
    ResolveAvailableOverflow {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Amount client has available.
        available: Decimal,
        /// Amount that is disputed.
        amount: Decimal,
    },
    /// Resolve transaction ID is not in dispute.
    ResolveTxNotInDispute {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is disputed.
        tx: TxId,
    },
    /// Chargeback transaction client ID does not match client ID of the
    /// disputed transaction.
    ChargebackClientMismatch {
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Client ID of the `dispute` transaction.
        chargeback_tx_client: ClientId,
        /// Client ID of the transaction that is disputed.
        disputed_tx_client: ClientId,
    },
    /// Account does not have sufficient funds to unhold in a dispute
    /// chargeback.
    ChargebackInsufficientHeld {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is disputed.
        tx: TxId,
        /// Amount client has held.
        held: Decimal,
        /// Amount that is disputed.
        amount: Decimal,
    },
    /// Chargeback transaction ID is not in dispute.
    ChargebackTxNotInDispute {
        /// Client ID.
        client: ClientId,
        /// Transaction ID that is disputed.
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
    /// Withdrawal transaction amount is negative.
    WithdrawalAmountNegative {
        /// Client ID.
        client: ClientId,
        /// Transaction ID.
        tx: TxId,
        /// Amount in the transaction.
        amount: Decimal,
    },
    /// Account does not have sufficient funds to withdraw.
    WithdrawalInsufficientAvailable {
        /// Client ID.
        client: ClientId,
        /// Withdrawal transaction ID.
        tx: TxId,
        /// Amount client has available.
        available: Decimal,
        /// Amount to withdraw.
        amount: Decimal,
    },
}

impl fmt::Display for TxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountLocked { client, tx } => write!(
                f,
                "Client {client} account is locked, not processing transaction {tx}.",
            ),
            Self::DisputeClientMismatch {
                tx,
                dispute_tx_client,
                disputed_tx_client,
            } => write!(
                f,
                "Dispute transaction client ID does not match client ID of the disputed transaction:\n\
                 transaction {tx}, dispute transaction client {dispute_tx_client}, disputed transaction client {disputed_tx_client}.",
            ),
            Self::DisputeTxNotFound { tx } => write!(
                f,
                "Dispute transaction not found in transaction block files: transaction {tx}.",
            ),
            Self::DisputeInsufficientAvailable {
                client,
                tx,
                available,
                amount,
            } => write!(
                f,
                "Account does not have sufficient funds to hold in a dispute:\n\
                 client {client}, transaction {tx}, available {available}, amount {amount}.",
            ),
            Self::DisputeHeldOverflow {
                client,
                tx,
                held,
                amount,
            } => write!(
                f,
                "Account held amount would overflow for dispute:\n\
                 client {client}, transaction {tx}, held {held}, amount {amount}.",
            ),
            Self::ResolveClientMismatch {
                tx,
                resolve_tx_client,
                disputed_tx_client,
            } => write!(
                f,
                "Resolve transaction client ID does not match client ID of the disputed transaction:\n\
                 transaction {tx}, resolve transaction client {resolve_tx_client}, disputed transaction client {disputed_tx_client}.",
            ),
            Self::ResolveInsufficientHeld {
                client,
                tx,
                held,
                amount,
            } => write!(
                f,
                "Account does not have sufficient funds to unhold in a dispute resolution:\n\
                 client {client}, transaction {tx}, held {held}, amount {amount}.",
            ),
            Self::ResolveAvailableOverflow {
                client,
                tx,
                available,
                amount,
            } => write!(
                f,
                "Account available amount would overflow for dispute resolution:\n\
                 client {client}, transaction {tx}, available {available}, amount {amount}.",
            ),
            Self::ResolveTxNotInDispute { client, tx } => write!(
                f,
                "Resolve transaction ID not in dispute: client {client}, transaction {tx}.",
            ),
            Self::ChargebackClientMismatch {
                tx,
                chargeback_tx_client,
                disputed_tx_client,
            } => write!(
                f,
                "Chargeback transaction client ID does not match client ID of the disputed transaction:\n\
                 transaction {tx}, chargeback transaction client {chargeback_tx_client}, disputed transaction client {disputed_tx_client}.",
            ),
            Self::ChargebackInsufficientHeld {
                client,
                tx,
                held,
                amount,
            } => write!(
                f,
                "Account does not have sufficient funds to unhold in a dispute chargeback:\n\
                 client {client}, transaction {tx}, held {held}, amount {amount}.",
            ),
            Self::ChargebackTxNotInDispute { client, tx } => write!(
                f,
                "Chargeback transaction ID not in dispute: client {client}, transaction {tx}.",
            ),
            Self::DepositAmountNegative { client, tx, amount } => write!(
                f,
                "Deposit transaction amount is negative: client {client}, transaction {tx}, amount {amount}."
            ),
            Self::DepositAvailableOverflow { client, tx } => write!(
                f,
                "Deposit transaction would cause an account's available funds to overflow: client {client}, transaction {tx}."
            ),
            Self::DepositTotalOverflow { client, tx } => write!(
                f,
                "A deposit transaction would cause an account's total funds to overflow: client {client}, transaction {tx}."
            ),
            Self::WithdrawalAmountNegative { client, tx, amount } => write!(
                f,
                "Withdrawal transaction amount is negative: client {client}, transaction {tx}, amount {amount}."
            ),
            Self::WithdrawalInsufficientAvailable {
                client,
                tx,
                available,
                amount,
            } => write!(
                f,
                "Account does not have sufficient funds to withdraw:\n\
                 client {client}, transaction {tx}, available {available}, amount {amount}.",
            ),
        }
    }
}

impl std::error::Error for TxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AccountLocked { .. } => None,
            Self::DisputeClientMismatch { .. } => None,
            Self::DisputeTxNotFound { .. } => None,
            Self::DisputeInsufficientAvailable { .. } => None,
            Self::DisputeHeldOverflow { .. } => None,
            Self::ResolveClientMismatch { .. } => None,
            Self::ResolveInsufficientHeld { .. } => None,
            Self::ResolveAvailableOverflow { .. } => None,
            Self::ResolveTxNotInDispute { .. } => None,
            Self::ChargebackClientMismatch { .. } => None,
            Self::ChargebackInsufficientHeld { .. } => None,
            Self::ChargebackTxNotInDispute { .. } => None,
            Self::DepositAmountNegative { .. } => None,
            Self::DepositAvailableOverflow { .. } => None,
            Self::DepositTotalOverflow { .. } => None,
            Self::WithdrawalAmountNegative { .. } => None,
            Self::WithdrawalInsufficientAvailable { .. } => None,
        }
    }
}

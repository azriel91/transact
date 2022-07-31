use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::model::{ClientId, TxId};

/// Types of transactions.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Transaction {
    /// Credit to the client's asset account.
    Deposit(Deposit),

    /// Debit to the client's asset account.
    Withdrawal(Withdrawal),

    /// Client's claim that a transaction was erroneous and should be reversed.
    Dispute(Dispute),

    /// Resolution to a dispute, releasing the associated held funds.
    Resolve(Resolve),

    /// Final state of a dispute and represents the client reversing a
    /// transaction.
    Chargeback(Chargeback),
}

impl Transaction {
    /// Returns this transaction's client ID.
    pub fn client(&self) -> ClientId {
        match self {
            Self::Deposit(deposit) => deposit.client(),
            Self::Withdrawal(withdrawal) => withdrawal.client(),
            Self::Dispute(dispute) => dispute.client(),
            Self::Resolve(resolve) => resolve.client(),
            Self::Chargeback(chargeback) => chargeback.client(),
        }
    }

    /// Returns this transaction's transaction ID.
    pub fn tx(&self) -> TxId {
        match self {
            Self::Deposit(deposit) => deposit.tx(),
            Self::Withdrawal(withdrawal) => withdrawal.tx(),
            Self::Dispute(dispute) => dispute.tx(),
            Self::Resolve(resolve) => resolve.tx(),
            Self::Chargeback(chargeback) => chargeback.tx(),
        }
    }
}

impl From<Deposit> for Transaction {
    fn from(deposit: Deposit) -> Transaction {
        Transaction::Deposit(deposit)
    }
}

impl From<Withdrawal> for Transaction {
    fn from(withdrawal: Withdrawal) -> Transaction {
        Transaction::Withdrawal(withdrawal)
    }
}

impl From<Dispute> for Transaction {
    fn from(dispute: Dispute) -> Transaction {
        Transaction::Dispute(dispute)
    }
}

impl From<Resolve> for Transaction {
    fn from(resolve: Resolve) -> Transaction {
        Transaction::Resolve(resolve)
    }
}

impl From<Chargeback> for Transaction {
    fn from(chargeback: Chargeback) -> Transaction {
        Transaction::Chargeback(chargeback)
    }
}

/// Credit to the client's asset account.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deposit {
    client: ClientId,
    tx: TxId,
    #[serde(with = "rust_decimal::serde::float")]
    amount: Decimal,
}

impl Deposit {
    /// Returns a new `Deposit` transaction.
    pub fn new(client: ClientId, tx: TxId, amount: Decimal) -> Self {
        Self { client, tx, amount }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> ClientId {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> TxId {
        self.tx
    }

    /// Get the transaction's amount.
    pub fn amount(&self) -> Decimal {
        self.amount
    }
}

/// Debit to the client's asset account.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Withdrawal {
    client: ClientId,
    tx: TxId,
    #[serde(with = "rust_decimal::serde::float")]
    amount: Decimal,
}

impl Withdrawal {
    /// Returns a new `Withdrawal` transaction.
    pub fn new(client: ClientId, tx: TxId, amount: Decimal) -> Self {
        Self { client, tx, amount }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> ClientId {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> TxId {
        self.tx
    }

    /// Get the transaction's amount.
    pub fn amount(&self) -> Decimal {
        self.amount
    }
}

/// Client's claim that a transaction was erroneous and should be reversed.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dispute {
    client: ClientId,
    tx: TxId,
}

impl Dispute {
    /// Returns a new `Dispute` transaction.
    pub fn new(client: ClientId, tx: TxId) -> Self {
        Self { client, tx }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> ClientId {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> TxId {
        self.tx
    }
}

/// Resolution to a dispute, releasing the associated held funds.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resolve {
    client: ClientId,
    tx: TxId,
}

impl Resolve {
    /// Returns a new `Resolve` transaction.
    pub fn new(client: ClientId, tx: TxId) -> Self {
        Self { client, tx }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> ClientId {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> TxId {
        self.tx
    }
}

/// Final state of a dispute and represents the client reversing a transaction.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chargeback {
    client: ClientId,
    tx: TxId,
}

impl Chargeback {
    /// Returns a new `Chargeback` transaction.
    pub fn new(client: ClientId, tx: TxId) -> Self {
        Self { client, tx }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> ClientId {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> TxId {
        self.tx
    }
}

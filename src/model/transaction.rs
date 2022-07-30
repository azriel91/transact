use serde::{Deserialize, Serialize};

use crate::model::{ClientId, TxId};

/// Types of transactions.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Deposit {
    client: ClientId,
    tx: TxId,
    amount: f64,
}

impl Deposit {
    /// Returns a new `Deposit` transaction.
    pub fn new(client: ClientId, tx: TxId, amount: f64) -> Self {
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
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

/// Debit to the client's asset account.
#[derive(Debug, Deserialize, Serialize)]
pub struct Withdrawal {
    client: ClientId,
    tx: TxId,
    amount: f64,
}

impl Withdrawal {
    /// Returns a new `Withdrawal` transaction.
    pub fn new(client: ClientId, tx: TxId, amount: f64) -> Self {
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
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

/// Client's claim that a transaction was erroneous and should be reversed.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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

/// Types of transactions.
#[derive(Debug)]
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

/// Credit to the client's asset account.
#[derive(Debug)]
pub struct Deposit {
    client: u16,
    tx: u32,
    amount: f64,
}

impl Deposit {
    /// Returns a new `Deposit` transaction.
    pub fn new(client: u16, tx: u32, amount: f64) -> Self {
        Self { client, tx, amount }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> u16 {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> u32 {
        self.tx
    }

    /// Get the transaction's amount.
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

/// Debit to the client's asset account.
#[derive(Debug)]
pub struct Withdrawal {
    client: u16,
    tx: u32,
    amount: f64,
}

impl Withdrawal {
    /// Returns a new `Withdrawal` transaction.
    pub fn new(client: u16, tx: u32, amount: f64) -> Self {
        Self { client, tx, amount }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> u16 {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> u32 {
        self.tx
    }

    /// Get the transaction's amount.
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

/// Client's claim that a transaction was erroneous and should be reversed.
#[derive(Debug)]
pub struct Dispute {
    client: u16,
    tx: u32,
}

impl Dispute {
    /// Returns a new `Dispute` transaction.
    pub fn new(client: u16, tx: u32) -> Self {
        Self { client, tx }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> u16 {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> u32 {
        self.tx
    }
}

/// Resolution to a dispute, releasing the associated held funds.
#[derive(Debug)]
pub struct Resolve {
    client: u16,
    tx: u32,
}

impl Resolve {
    /// Returns a new `Resolve` transaction.
    pub fn new(client: u16, tx: u32) -> Self {
        Self { client, tx }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> u16 {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> u32 {
        self.tx
    }
}

/// Final state of a dispute and represents the client reversing a transaction.
#[derive(Debug)]
pub struct Chargeback {
    client: u16,
    tx: u32,
}

impl Chargeback {
    /// Returns a new `Chargeback` transaction.
    pub fn new(client: u16, tx: u32) -> Self {
        Self { client, tx }
    }

    /// Get the transaction's client.
    pub fn client(&self) -> u16 {
        self.client
    }

    /// Get the transaction's tx.
    pub fn tx(&self) -> u32 {
        self.tx
    }
}

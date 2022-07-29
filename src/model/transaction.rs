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
pub struct Deposit {}

/// Debit to the client's asset account.
#[derive(Debug)]
pub struct Withdrawal {}

/// Client's claim that a transaction was erroneous and should be reversed.
#[derive(Debug)]
pub struct Dispute {}

/// Resolution to a dispute, releasing the associated held funds.
#[derive(Debug)]
pub struct Resolve {}

/// Final state of a dispute and represents the client reversing a transaction.
#[derive(Debug)]
pub struct Chargeback {}

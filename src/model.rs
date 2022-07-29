//! Contains the data model for transactions.
//!
//! In a larger project, this may be published as a separate crate so consumers
//! may rely on the model API, while keeping the business logic private.

pub use transaction::{Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal};

mod transaction;

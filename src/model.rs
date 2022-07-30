//! Contains the data model for transactions.
//!
//! In a larger project, this may be published as a separate crate so consumers
//! may rely on the model API, while keeping the business logic private.

pub use self::{
    account::Account,
    client_id::ClientId,
    transaction::{Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal},
    tx_id::TxId,
};

mod account;
mod client_id;
mod transaction;
mod tx_id;

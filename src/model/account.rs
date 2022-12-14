use std::collections::HashSet;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::model::{ClientId, TxId};

/// Error when `available` and `held` amounts will overflow when added together.
#[derive(Debug)]
pub struct TotalOverflow;

/// Client account state.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Account {
    client: ClientId,
    #[serde(with = "rust_decimal::serde::float")]
    available: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    held: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    total: Decimal,
    locked: bool,
    #[serde(skip)]
    disputed_txs: HashSet<TxId>,
}

impl Account {
    /// Returns a new `Account` with the provided values.
    pub fn try_new(
        client: ClientId,
        available: Decimal,
        held: Decimal,
        locked: bool,
        disputed_txs: HashSet<TxId>,
    ) -> Result<Self, TotalOverflow> {
        let total = available.checked_add(held).ok_or(TotalOverflow)?;

        Ok(Self {
            client,
            available,
            held,
            total,
            locked,
            disputed_txs,
        })
    }

    /// Returns a new empty `Account`.
    pub fn empty(client: ClientId) -> Self {
        // Should be sensible defaults
        let available = dec!(0.0);
        let held = dec!(0.0);
        let total = dec!(0.0);
        let locked = false;
        let disputed_txs = HashSet::new();

        Self {
            client,
            available,
            held,
            total,
            locked,
            disputed_txs,
        }
    }

    /// Returns the account's client.
    pub fn client(&self) -> ClientId {
        self.client
    }

    /// Returns the available funds in the account.
    pub fn available(&self) -> Decimal {
        self.available
    }

    /// Returns the held funds in the account.
    pub fn held(&self) -> Decimal {
        self.held
    }

    /// Returns the total funds in the account.
    pub fn total(&self) -> Decimal {
        self.total
    }

    /// Returns whether the account is locked.
    pub fn locked(&self) -> bool {
        self.locked
    }

    /// Returns open disputed transactions.
    pub fn disputed_txs(&self) -> &HashSet<TxId> {
        &self.disputed_txs
    }
}

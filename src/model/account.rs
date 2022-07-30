use serde::{Deserialize, Serialize};

use crate::model::ClientId;

/// Client account state.
#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    client: ClientId,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

impl Account {
    /// Returns a new `Account`.
    pub fn new(client: ClientId) -> Self {
        // Should be sensible defaults
        let available = 0.0;
        let held = 0.0;
        let total = 0.0;
        let locked = false;

        Self {
            client,
            available,
            held,
            total,
            locked,
        }
    }

    /// Get the account's client.
    pub fn client(&self) -> ClientId {
        self.client
    }

    /// Get the account's available.
    pub fn available(&self) -> f64 {
        self.available
    }

    /// Get the account's held.
    pub fn held(&self) -> f64 {
        self.held
    }

    /// Get the account's total.
    pub fn total(&self) -> f64 {
        self.total
    }

    /// Get the account's locked.
    pub fn locked(&self) -> bool {
        self.locked
    }
}

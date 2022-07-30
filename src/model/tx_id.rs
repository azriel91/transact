use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

/// Transaction ID. [`u32`] newtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct TxId(u32);

impl TxId {
    /// Returns a new `TxId`.
    pub fn new(tx_id: u32) -> Self {
        Self(tx_id)
    }

    /// Returns the inner [`u32`].
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl Deref for TxId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for TxId {
    fn from(tx_id: u32) -> Self {
        Self(tx_id)
    }
}

impl fmt::Display for TxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

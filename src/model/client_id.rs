use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// Client ID. [`u16`] newtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct ClientId(u16);

impl ClientId {
    /// Returns a new `ClientId`.
    pub fn new(client_id: u16) -> Self {
        Self(client_id)
    }

    /// Returns the inner [`u16`].
    pub fn into_inner(self) -> u16 {
        self.0
    }
}

impl Deref for ClientId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClientId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u16> for ClientId {
    fn from(client_id: u16) -> Self {
        Self(client_id)
    }
}

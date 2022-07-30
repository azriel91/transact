use std::{
    collections::{hash_map::IntoValues, HashMap},
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::model::{Account, ClientId};

/// Working record of all accounts. `HashMap<ClientId, Account>` newtype.
///
/// As long as we only hold up to `u16` accounts, the amount of working memory
/// should be `2^16 * size_of::<Account>()` (27 bytes), which should be about
/// 1.7 MB, plus any memory allocated while processing transactions.
#[derive(Debug, Deserialize, Serialize)]
pub struct Accounts(HashMap<ClientId, Account>);

impl Accounts {
    /// Returns a new `Accounts` list.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns an iterator of accounts.
    pub fn into_values(self) -> IntoValues<ClientId, Account> {
        self.0.into_values()
    }
}

impl Deref for Accounts {
    type Target = HashMap<ClientId, Account>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Accounts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Accounts {
    fn default() -> Self {
        Self::new()
    }
}

// Allows [`futures::stream::StreamExt::collect`]
impl Extend<(ClientId, Account)> for Accounts {
    fn extend<T: IntoIterator<Item = (ClientId, Account)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

pub mod errors;

use axum::Form;
use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::NonemptyStringVisitor;

pub use errors::*;

/// An account here is akin to an account in the double entry bookkeeping model.
/// It may represent your bank account, an "expenses" account, and so on. Its
/// balance is represented in cents.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Account {
    id: Uuid,
    name: AccountName,
    balance_cents: i64,
}

impl Account {
    pub fn new(id: Uuid, name: AccountName, balance_cents: i64) -> Self {
        Self {
            id,
            name,
            balance_cents,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &AccountName {
        &self.name
    }

    pub fn balance_cents(&self) -> i64 {
        self.balance_cents
    }
}

/// A valid account name.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub struct AccountName(String);

impl AccountName {
    pub fn new(raw: &str) -> Result<Self, AccountNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(AccountNameEmptyError)
        } else {
            Ok(Self(raw.to_string()))
        }
    }

    pub fn into_url_encoding(self) -> String {
        Form(self).to_string()
    }
}

impl<'de> Deserialize<'de> for AccountName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = deserializer.deserialize_str(NonemptyStringVisitor)?;

        AccountName::new(&raw)
            .map_err(|_| serde::de::Error::custom("account name must be a nonempty string"))
    }
}

/// The fields required to create an [Account]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateAccountRequest {
    name: AccountName,
}

impl CreateAccountRequest {
    /// Create a new [CreateAccountRequest]. Note that a balance is not present,
    /// because a new account will always have a balance of 0.
    pub fn new(name: AccountName) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &AccountName {
        &self.name
    }
}

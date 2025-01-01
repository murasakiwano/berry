use derive_more::derive::Display;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

/// An account here is akin to an account in the double entry bookkeeping model.
/// It may represent your bank account, an "expenses" account, and so on. Its
/// balance is represented in cents.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
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

#[derive(Clone, Debug, Error)]
#[error("Account name must not be empty")]
pub struct AccountNameEmptyError;

impl AccountName {
    pub fn new(raw: &str) -> Result<Self, AccountNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(AccountNameEmptyError)
        } else {
            Ok(Self(raw.to_string()))
        }
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

/// Specifies errors that may arise from interacting with [Account]s
#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Account name {name} is already taken")]
    Duplicate { name: AccountName },
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from creating an [Account]
#[derive(Debug, Error)]
pub enum CreateAccountError {
    #[error("Account name {name} is already taken")]
    Duplicate { name: AccountName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from getting an [Account]
#[derive(Debug, Error)]
pub enum GetAccountError {
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise when updating an [Account]
#[derive(Debug, Error)]
pub enum UpdateAccountError {
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error("Account with name {name} already exists")]
    Duplicate { name: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise when updating an [Account]
#[derive(Debug, Error)]
pub enum DeleteAccountError {
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

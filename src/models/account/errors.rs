use uuid::Uuid;

use super::AccountName;

#[derive(Clone, Debug, thiserror::Error)]
#[error("Account name must not be empty")]
pub struct AccountNameEmptyError;

/// Specifies errors that may arise from interacting with [Account]s
#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("Account name {name} is already taken")]
    Duplicate { name: AccountName },
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from creating an [Account]
#[derive(Debug, thiserror::Error)]
pub enum CreateAccountError {
    #[error("Account name {name} is already taken")]
    Duplicate { name: AccountName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from getting an [Account]
#[derive(Debug, thiserror::Error)]
pub enum GetAccountError {
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from listing [Account]s
#[derive(Debug, thiserror::Error)]
pub enum ListAccountsError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise when updating an [Account]
#[derive(Debug, thiserror::Error)]
pub enum UpdateAccountError {
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error("Account with name {name} already exists")]
    Duplicate { name: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise when updating an [Account]
#[derive(Debug, thiserror::Error)]
pub enum DeleteAccountError {
    #[error("Account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

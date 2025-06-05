use uuid::Uuid;

use super::AccountName;

#[derive(Clone, Debug, thiserror::Error)]
#[error("Account name must not be empty")]
pub struct AccountNameEmptyError;

/// Specifies errors that may arise from interacting with [Account]s
#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("account name \"{name}\" is already taken")]
    Duplicate { name: AccountName },
    #[error("account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from creating an [Account]
#[derive(Debug, thiserror::Error)]
pub enum CreateAccountError {
    #[error("account name \"{name}\" is already taken")]
    Duplicate { name: AccountName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetOrCreateAccountError {
    #[error("account name \"{name}\" is already taken")]
    Duplicate { name: AccountName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<CreateAccountError> for GetOrCreateAccountError {
    fn from(value: CreateAccountError) -> Self {
        match value {
            CreateAccountError::Duplicate { name } => GetOrCreateAccountError::Duplicate { name },
            CreateAccountError::Unknown(err) => GetOrCreateAccountError::Unknown(err),
        }
    }
}

/// Specifies errors that may arise from getting an [Account]
#[derive(Debug, thiserror::Error)]
pub enum GetAccountError {
    #[error("account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from getting an [Account], filtering by its name
#[derive(Debug, thiserror::Error)]
pub enum GetAccountByNameError {
    #[error("account with name {name} not found")]
    NotFound { name: AccountName },
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
    #[error("account with id {id} not found")]
    NotFound { id: Uuid },
    #[error("account with name \"{name}\" already exists")]
    Duplicate { name: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise when updating the balance of an [Account]
#[derive(Debug, thiserror::Error)]
pub enum UpdateAccountBalanceError {
    #[error("account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise when renaming an [Account]
#[derive(Debug, thiserror::Error)]
pub enum RenameAccountError {
    #[error("account with id {id} not found")]
    NotFound { id: Uuid },
    #[error("account with name \"{name}\" already exists")]
    Duplicate { name: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise when updating an [Account]
#[derive(Debug, thiserror::Error)]
pub enum DeleteAccountError {
    #[error("account with id {id} not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

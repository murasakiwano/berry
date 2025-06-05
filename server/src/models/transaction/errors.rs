use uuid::Uuid;

#[derive(Clone, Debug, thiserror::Error)]
#[error("transaction should have a nonempty title")]
pub struct TransactionTitleEmptyError;

/// Specifies errors that may arise from creating a [Transaction]
#[derive(Debug, thiserror::Error)]
pub enum CreateTransactionError {
    #[error("source account with id {id} was not found")]
    SourceAccountNotFound { id: Uuid },
    #[error("destination account with id {id} was not found")]
    DestinationAccountNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from deleting a [Transaction]
#[derive(Debug, thiserror::Error)]
pub enum DeleteTransactionError {
    #[error("transaction with id {id} was not found")]
    TransactionNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from reading a [Transaction]
#[derive(Debug, thiserror::Error)]
pub enum GetTransactionError {
    #[error("transaction with id {id} was not found")]
    TransactionNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from listing [Transaction]s
#[derive(Debug, thiserror::Error)]
pub enum ListTransactionsError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

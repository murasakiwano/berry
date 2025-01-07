use uuid::Uuid;

#[derive(Clone, Debug, thiserror::Error)]
#[error("Transaction should have a nonempty title")]
pub struct TransactionTitleEmptyError;

/// Specifies errors that may arise from creating a [Transaction]
#[derive(Debug, thiserror::Error)]
pub enum CreateTransactionError {
    #[error("Source account with id {id} was not found")]
    SourceAccountNotFound { id: Uuid },
    #[error("Destination account with id {id} was not found")]
    DestinationAccountNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from deleting a [Transaction]
#[derive(Debug, thiserror::Error)]
pub enum DeleteTransactionError {
    #[error("Transaction with id {id} was not found")]
    TransactionNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from reading a [Transaction]
#[derive(Debug, thiserror::Error)]
pub enum GetTransactionError {
    #[error("Transaction with id {id} was not found")]
    TransactionNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from listing [Transaction]s
#[derive(Debug, thiserror::Error)]
pub enum ListTransactionsError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

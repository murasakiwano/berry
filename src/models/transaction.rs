use chrono::{DateTime, Utc};
use derive_more::derive::Display;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

/// A uniquely identifiable monetary transaction between two [Account]s.
/// All amounts are represented as cents.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Transaction {
    id: Uuid,
    title: TransactionTitle,
    amount_cents: i64,
    source_account_id: Uuid,
    destination_account_id: Uuid,
    category: Option<String>,
    /// The moment the transaction happened.
    posting_date: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        id: Uuid,
        title: TransactionTitle,
        amount_cents: i64,
        source_account_id: Uuid,
        destination_account_id: Uuid,
        category: Option<String>,
        posting_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            amount_cents,
            source_account_id,
            destination_account_id,
            category,
            posting_date,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &TransactionTitle {
        &self.title
    }

    pub fn amount_cents(&self) -> i64 {
        self.amount_cents
    }

    pub fn display_amount(&self) -> String {
        let reais = self.amount_cents / 100;
        let cents = self.amount_cents.abs() % 100;

        format!("{}.{:02} BRL", reais, cents)
    }

    pub fn from_account(&self) -> Uuid {
        self.source_account_id
    }

    pub fn to_account(&self) -> Uuid {
        self.destination_account_id
    }

    pub fn category(&self) -> &Option<String> {
        &self.category
    }
}

#[derive(Clone, Debug, Error)]
#[error("Transaction should have a nonempty title")]
pub struct TransactionTitleEmptyError;

/// A valid transaction title
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub struct TransactionTitle(String);

impl TransactionTitle {
    pub fn new(raw: &str) -> Result<Self, TransactionTitleEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(TransactionTitleEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

/// The fields required to create a [Transaction]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTransactionRequest {
    title: TransactionTitle,
    amount_cents: i64,
    source_account_id: Uuid,
    destination_account_id: Uuid,
    category: Option<String>,
    posting_date: DateTime<Utc>,
}

impl CreateTransactionRequest {
    /// Create a new [CreateTransactionRequest].
    ///
    /// If `date` is [None], it defaults to
    pub fn new(
        title: TransactionTitle,
        amount_cents: i64,
        source_account_id: Uuid,
        destination_account_id: Uuid,
        category: Option<String>,
        posting_date: Option<DateTime<Utc>>,
    ) -> Self {
        let posting_date = posting_date.unwrap_or_else(Utc::now);

        Self {
            title,
            amount_cents,
            source_account_id,
            destination_account_id,
            category,
            posting_date,
        }
    }

    pub fn title(&self) -> &TransactionTitle {
        &self.title
    }

    pub fn amount_cents(&self) -> i64 {
        self.amount_cents
    }

    pub fn source_account_id(&self) -> Uuid {
        self.source_account_id
    }

    pub fn destination_account_id(&self) -> Uuid {
        self.destination_account_id
    }

    pub fn category(&self) -> &Option<String> {
        &self.category
    }

    pub fn posting_date(&self) -> DateTime<Utc> {
        self.posting_date
    }
}

/// Specifies errors that may arise from interacting with [Transaction]s
#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Source account with id {id} was not found")]
    SourceAccountNotFound { id: Uuid },
    #[error("Destination account with id {id} was not found")]
    DestinationAccountNotFound { id: Uuid },
    #[error("Transaction with id {id} was not found")]
    NotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from creating a [Transaction]
#[derive(Debug, Error)]
pub enum CreateTransactionError {
    #[error("Source account with id {id} was not found")]
    SourceAccountNotFound { id: Uuid },
    #[error("Destination account with id {id} was not found")]
    DestinationAccountNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from deleting a [Transaction]
#[derive(Debug, Error)]
pub enum DeleteTransactionError {
    #[error("Transaction with id {id} was not found")]
    TransactionNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Specifies errors that may arise from reading a [Transaction]
#[derive(Debug, Error)]
pub enum GetTransactionError {
    #[error("Transaction with id {id} was not found")]
    TransactionNotFound { id: Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub mod errors;

use axum::Form;
use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::NonemptyStringVisitor;

pub use errors::*;

/// A uniquely identifiable monetary transaction between two [Account]s.
/// All amounts are represented as cents.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

    pub fn into_url_encoded(self) -> String {
        Form(self).to_string()
    }
}

impl<'de> Deserialize<'de> for TransactionTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = deserializer.deserialize_str(NonemptyStringVisitor)?;

        TransactionTitle::new(&raw)
            .map_err(|_| serde::de::Error::custom("transaction title must be a nonempty string"))
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
    posting_date: Option<NaiveDateTime>,
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
        posting_date: Option<NaiveDateTime>,
    ) -> Self {
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

    pub fn posting_date(&self) -> Option<NaiveDateTime> {
        self.posting_date
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::models::transaction::TransactionTitle;

    use super::Transaction;

    #[test]
    fn serde_valid_json() {
        let tx = Transaction::new(
            Uuid::new_v4(),
            TransactionTitle::new("test").unwrap(),
            42,
            Uuid::new_v4(),
            Uuid::new_v4(),
            None,
            Utc::now(),
        );

        let ser = serde_json::to_string(&tx);
        assert!(ser.is_ok());

        let raw = ser.unwrap();
        let result: Result<Transaction, serde_json::Error> = serde_json::from_str(&raw);

        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_json() {
        let raw = "{\"id\":\"b309f1bc-c1b6-4dd4-8404-c8c8627025a9\",\"title\":\"test\",\"amount_cents\":42,\"source_account_id\":\"ef342846-ecea-43e7-a69e-fea1686a2f7c\",\"destination_account_id\":\"0f08a046-0338-40b1-97c6-5feb925b6f84\",\"category\":null,\"posting_date\":\"2025-01-06T14:56:00.760515Z\"}";

        let result: Result<Transaction, serde_json::Error> = serde_json::from_str(raw);

        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_invalid_json_no_title() {
        let raw = "{\"id\":\"b309f1bc-c1b6-4dd4-8404-c8c8627025a9\",\"title\":\"\",\"amount_cents\":42,\"source_account_id\":\"ef342846-ecea-43e7-a69e-fea1686a2f7c\",\"destination_account_id\":\"0f08a046-0338-40b1-97c6-5feb925b6f84\",\"category\":null,\"posting_date\":\"2025-01-06T14:56:00.760515Z\"}";
        let result: Result<Transaction, serde_json::Error> = serde_json::from_str(raw);

        assert!(result.is_err());
    }
}

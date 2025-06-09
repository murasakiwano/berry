use std::str::FromStr as _;

use anyhow::Context as _;
use axum::Json;
use axum::http::StatusCode;
use axum::{Form, extract::State};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

use crate::models::transaction::{
    CreateTransactionError, CreateTransactionRequest, Transaction, TransactionTitle,
};
use crate::server::AppState;

#[derive(Deserialize, Debug)]
pub struct CreateTransactionRequestBody {
    title: String,
    amount: Decimal,
    source_account_id: String,
    destination_account_id: String,
    category: Option<String>,
    posting_date: Option<NaiveDateTime>,
}

impl CreateTransactionRequestBody {
    fn into_domain_model(self) -> anyhow::Result<CreateTransactionRequest> {
        let title = TransactionTitle::new(&self.title)?;
        let source_account_id = Uuid::from_str(&self.source_account_id)
            .context("Could not parse the source account's ID")
            .map_err(|e| {
                tracing::error!("{:?}\n{}", e, e.backtrace());

                CreateTransactionRequestBodyParseError {
                    field: "source_account_id".to_string(),
                }
            })?;
        let destination_account_id = Uuid::from_str(&self.destination_account_id)
            .context("Failed to parse destination account's ID")
            .map_err(|e| {
                tracing::error!("{:?}\n{}", e, e.backtrace());

                CreateTransactionRequestBodyParseError {
                    field: "destination_account_id".to_string(),
                }
            })?;

        Ok(CreateTransactionRequest::new(
            title,
            self.amount,
            source_account_id,
            destination_account_id,
            self.category,
            self.posting_date,
        ))
    }
}

#[derive(Debug, Error)]
#[error("Failed to parse the following body field: {field}")]
pub struct CreateTransactionRequestBodyParseError {
    field: String,
}

pub async fn create_transaction(
    State(state): State<AppState>,
    Form(body): Form<CreateTransactionRequestBody>,
) -> Result<(StatusCode, Json<Transaction>), (StatusCode, String)> {
    let req = body
        .into_domain_model()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let transaction = state
        .service
        .create_transaction(&req)
        .await
        .map_err(|e| match e {
            CreateTransactionError::SourceAccountNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("could not find source account {}", id),
            ),
            CreateTransactionError::DestinationAccountNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("could not find destination account {}", id),
            ),
            CreateTransactionError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        })?;

    Ok((StatusCode::CREATED, Json(transaction)))
}

use std::str::FromStr;

use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    http::AppState,
    models::{
        account::{Account, AccountName, CreateAccountError, GetAccountError, UpdateAccountError},
        transaction::{
            CreateTransactionError, CreateTransactionRequest, GetTransactionError, Transaction,
            TransactionTitle,
        },
    },
};

#[derive(Deserialize, Debug)]
pub struct CreateAccountRequest {
    name: String,
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(body): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<Account>), (StatusCode, String)> {
    let account_name = AccountName::new(&body.name).map_err(|e| {
        tracing::error!(error = ?e);

        (
            StatusCode::BAD_REQUEST,
            "Account name must not be empty".to_string(),
        )
    })?;

    let account = state
        .pool
        .create_account(&account_name)
        .await
        .map_err(|err| match err {
            CreateAccountError::Duplicate { name } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("An account with the name {} already exists", name),
            ),
            CreateAccountError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        })?;

    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Account>>, (StatusCode, &'static str)> {
    let accounts = state.pool.list_accounts().await.map_err(|err| {
        tracing::error!("{:?}\n{}", err, err.backtrace());

        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(Json(accounts))
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Account>), (StatusCode, String)> {
    let account = state
        .pool
        .get_account_by_id(id)
        .await
        .map_err(|e| match e {
            GetAccountError::NotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Account with id {id} not found"),
            ),
            GetAccountError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        })?;

    Ok((StatusCode::OK, Json(account)))
}

pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> anyhow::Result<StatusCode, (StatusCode, &'static str)> {
    state.pool.delete_account(id).await.map_err(|e| {
        tracing::error!("{:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct RenameAccountRequestBody {
    name: String,
}

pub async fn rename_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<RenameAccountRequestBody>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .pool
        .rename_account(id, &body.name)
        .await
        .map_err(|e| match e {
            UpdateAccountError::NotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Account with id {} does not exist", id),
            ),
            UpdateAccountError::Duplicate { name } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Account with name {} already exists", name),
            ),
            UpdateAccountError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        })?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct UpdateAccountBalanceRequestBody {
    new_balance: i64,
}

pub async fn update_account_balance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateAccountBalanceRequestBody>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .pool
        .update_account_balance(id, body.new_balance)
        .await
        .map_err(|e| match e {
            UpdateAccountError::NotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Account with id {} does not exist", id),
            ),
            UpdateAccountError::Duplicate { name } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Account name {} is already taken", name),
            ),
            UpdateAccountError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        })?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct CreateTransactionRequestBody {
    title: String,
    amount_cents: i64,
    source_account_id: String,
    destination_account_id: String,
    category: Option<String>,
    posting_date: Option<DateTime<Utc>>,
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
            self.amount_cents,
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
    Json(body): Json<CreateTransactionRequestBody>,
) -> Result<(StatusCode, Json<Transaction>), (StatusCode, String)> {
    let req = body
        .into_domain_model()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let transaction = state
        .pool
        .create_transaction(&req)
        .await
        .map_err(|e| match e {
            CreateTransactionError::SourceAccountNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Could not find source account {}", id),
            ),
            CreateTransactionError::DestinationAccountNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Could not find destination account {}", id),
            ),
            CreateTransactionError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        })?;

    Ok((StatusCode::CREATED, Json(transaction)))
}

pub async fn list_transactions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let transactions = state.pool.list_transactions().await.map_err(|e| {
        tracing::error!(error = ?e,"An error occurred when listing all transactions" );

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    Ok(Json(transactions))
}

pub async fn get_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Transaction>, (StatusCode, String)> {
    let transaction = state
        .pool
        .get_transaction_by_id(id)
        .await
        .map_err(|e| match e {
            GetTransactionError::TransactionNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Transaction with id {} does not exist", id),
            ),
            GetTransactionError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        })?;

    Ok(Json(transaction))
}

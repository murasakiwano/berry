use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::account::{Account, AccountName, GetAccountByNameError, GetAccountError};
use crate::server::AppState;

#[derive(Deserialize)]
pub struct AccountNameQuery {
    name: String,
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Account>), (StatusCode, String)> {
    let account = state
        .service
        .get_account_by_id(id)
        .await
        .map_err(|e| match e {
            GetAccountError::NotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("account with id {id} not found"),
            ),
            GetAccountError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        })?;

    Ok((StatusCode::OK, Json(account)))
}

pub async fn find_account_by_name(
    State(state): State<AppState>,
    Query(query): Query<AccountNameQuery>,
) -> Result<(StatusCode, Json<Account>), (StatusCode, String)> {
    let account_name = AccountName::new(&query.name).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "account name must not be empty".to_string(),
        )
    })?;
    let account = state
        .service
        .get_account_by_name(&account_name)
        .await
        .map_err(|e| match e {
            GetAccountByNameError::NotFound { name } => (
                StatusCode::NOT_FOUND,
                format!("no account found with name {}", name),
            ),
            GetAccountByNameError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        })?;

    Ok((StatusCode::OK, Json(account)))
}

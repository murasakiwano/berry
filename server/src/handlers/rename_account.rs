use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Form;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::account::{AccountName, UpdateAccountError};
use crate::server::AppState;

#[derive(Deserialize)]
pub struct RenameAccountRequestBody {
    name: String,
}

pub async fn rename_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(body): Form<RenameAccountRequestBody>,
) -> Result<StatusCode, (StatusCode, String)> {
    let account_name = AccountName::new(&body.name).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "account name must not be empty".to_string(),
        )
    })?;

    state
        .service
        .rename_account(id, account_name)
        .await
        .map_err(|e| match e {
            UpdateAccountError::NotFound { id } => (
                StatusCode::UNPROCESSABLE_ENTITY,
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

use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::models::transaction::DeleteTransactionError;
use crate::server::AppState;

pub async fn delete_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> anyhow::Result<StatusCode, (StatusCode, String)> {
    state
        .service
        .delete_transaction(id)
        .await
        .map_err(|e| match e {
            DeleteTransactionError::TransactionNotFound { id } => {
                tracing::error!(txid = ?id, "tried to delete unexisting transaction");
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("transaction with ID {} does not exist", id),
                )
            }
            DeleteTransactionError::Unknown(e) => {
                tracing::error!(error = ?e, "failed to delete transaction");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

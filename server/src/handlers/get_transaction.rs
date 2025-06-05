use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::models::transaction::{GetTransactionError, Transaction};
use crate::server::AppState;

pub async fn get_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Transaction>, (StatusCode, String)> {
    let transaction = state
        .service
        .get_transaction_by_id(id)
        .await
        .map_err(|e| match e {
            GetTransactionError::TransactionNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("transaction with id {} does not exist", id),
            ),
            GetTransactionError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        })?;

    Ok(Json(transaction))
}

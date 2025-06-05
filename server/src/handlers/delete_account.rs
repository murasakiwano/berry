use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::models::account::DeleteAccountError;
use crate::server::AppState;

pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> anyhow::Result<StatusCode, (StatusCode, String)> {
    state
        .service
        .delete_account(id)
        .await
        .map_err(|e| match e {
            DeleteAccountError::NotFound { id } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("account with id {} does not exist.", id),
            ),
            DeleteAccountError::Unknown(e) => {
                tracing::error!("{:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

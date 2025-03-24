use axum::http::StatusCode;
use axum::{extract::State, Json};

use crate::{models::transaction::Transaction, server::AppState};

pub async fn list_transactions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let transactions = state.service.list_transactions().await.map_err(|e| {
        tracing::error!(error = ?e,"An error occurred when listing all transactions" );

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    Ok(Json(transactions))
}

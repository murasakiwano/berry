use axum::extract::Query;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use serde::Deserialize;

use crate::service::PaginationParameters;
use crate::{models::transaction::Transaction, server::AppState};

#[derive(Deserialize, Default)]
pub struct ListTransactionsQuery {
    // Pagination
    /// the `page` is 1-indexed
    page: Option<u32>,
    /// How many items per page
    per_page: Option<u32>,
}

pub async fn list_transactions(
    State(AppState { service }): State<AppState>,
    Query(pagination): Query<ListTransactionsQuery>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);
    let limit = per_page.min(100) as i64;
    let offset = page.saturating_sub(1) as i64 * limit;
    let transactions = service
        .list_transactions(PaginationParameters { limit, offset })
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "An error occurred when listing all transactions");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })?;

    Ok(Json(transactions))
}

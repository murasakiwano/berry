use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Json, extract::State};
use serde::Deserialize;

use crate::service::PaginationParameters;
use crate::{models::transaction::Transaction, server::AppState};

#[derive(Deserialize, Default)]
pub struct ListTransactionsQuery {
    /// The `page` is 1-indexed
    page: Option<u32>,
    /// How many items per page
    per_page: Option<u32>,
}

pub async fn list_transactions(
    State(AppState { service }): State<AppState>,
    Query(pagination): Query<ListTransactionsQuery>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let pagination_parameters = if let ListTransactionsQuery {
        page: None,
        per_page: None,
    } = pagination
    {
        None
    } else {
        let limit = pagination.per_page.unwrap_or(20).min(100) as i64;
        let offset = pagination.page.unwrap_or(1).saturating_sub(1) as i64 * limit;

        Some(PaginationParameters { limit, offset })
    };

    let transactions = service
        .list_transactions(pagination_parameters)
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

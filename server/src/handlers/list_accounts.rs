use axum::{Json, extract::State, http::StatusCode};

use crate::{
    models::account::{Account, ListAccountsError},
    server::AppState,
};

pub async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Account>>, (StatusCode, &'static str)> {
    let accounts = state
        .service
        .list_accounts()
        .await
        .map_err(|err| match err {
            ListAccountsError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        })?;

    Ok(Json(accounts))
}

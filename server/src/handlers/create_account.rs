use axum::{Form, Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::{
    models::account::{Account, AccountName, CreateAccountError},
    server::AppState,
};

#[derive(Deserialize, Debug)]
pub struct CreateAccountRequest {
    name: String,
}

pub async fn create_account(
    State(state): State<AppState>,
    Form(body): Form<CreateAccountRequest>,
) -> Result<(StatusCode, Json<Account>), (StatusCode, String)> {
    let account_name = AccountName::new(&body.name).map_err(|e| {
        tracing::error!(error = ?e);

        (
            StatusCode::BAD_REQUEST,
            "account name must not be empty".to_string(),
        )
    })?;

    let account = state
        .service
        .create_account(&account_name)
        .await
        .map_err(|err| match err {
            CreateAccountError::Duplicate { name } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("an account with the name \"{}\" already exists", name),
            ),
            CreateAccountError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        })?;

    Ok((StatusCode::CREATED, Json(account)))
}

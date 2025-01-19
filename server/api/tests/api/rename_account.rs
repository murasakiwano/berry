use berry::models::account::{Account, AccountName};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn rename_account_with_valid_name() {
    let app = spawn_app().await;
    let new_name = AccountName::new("Second account").unwrap();
    let body = format!("name={}", new_name.clone().into_url_encoding());

    let response = app
        .rename_account(app.test_account.id.to_string(), body.to_string())
        .await;
    let status = response.status().as_u16();

    let account = app
        .get_account(app.test_account.id.to_string())
        .await
        .bytes()
        .await
        .unwrap();
    let account: Account = serde_json::from_slice(&account).unwrap();

    assert_eq!(StatusCode::OK, status);
    assert_eq!(app.test_account.id, account.id());
    assert_eq!(new_name, *account.name());
}

#[tokio::test]
async fn rename_unexisting_account_returns_unprocessable_entity() {
    let app = spawn_app().await;
    let new_name = AccountName::new("Second account").unwrap();
    let body = format!("name={}", new_name.clone().into_url_encoding());

    let response = app
        .rename_account(Uuid::new_v4().to_string(), body.to_string())
        .await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, status);
}

#[tokio::test]
async fn rename_account_with_empty_name_returns_bad_request() {
    let app = spawn_app().await;
    let body = "name=";

    let response = app
        .rename_account(app.test_account.id.to_string(), body.to_string())
        .await;
    let status = response.status().as_u16();
    let body = response.bytes().await;
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::BAD_REQUEST, status);
}

#[tokio::test]
async fn rename_account_with_duplicate_name_returns_unprocessable_entity() {
    let app = spawn_app().await;
    let body = "name=Test%20account";
    app.post_account(body.to_string()).await;

    let response = app
        .rename_account(app.test_account.id.to_string(), body.to_string())
        .await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, status);
}

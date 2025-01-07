use std::str::from_utf8;

use reqwest::StatusCode;
use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn delete_existing_account() {
    let app = spawn_app().await;

    let response = app.delete_account(app.test_account.id.to_string()).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    let body = from_utf8(&body);
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::NO_CONTENT, status);
}

#[tokio::test]
async fn deleting_an_unexisting_account_returns_unprocessable_entity() {
    let app = spawn_app().await;

    let response = app.delete_account(Uuid::new_v4().to_string()).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    let body = from_utf8(&body);
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, status);
}

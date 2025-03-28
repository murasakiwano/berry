use std::str::from_utf8;

use reqwest::StatusCode;

use crate::helpers::spawn_app;

#[tokio::test]
async fn create_account_with_valid_name() {
    let app = spawn_app().await;
    let body = "name=Test%20account";

    let response = app.post_account(body.to_string()).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    let body = from_utf8(&body);
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::CREATED, status);
}

#[tokio::test]
async fn accounts_must_not_have_empty_names() {
    let app = spawn_app().await;
    let body = "name=";

    let response = app.post_account(body.to_string()).await;

    assert_eq!(StatusCode::BAD_REQUEST, response.status().as_u16());
}

#[tokio::test]
async fn accounts_with_duplicate_names_are_invalid() {
    let app = spawn_app().await;
    let req_body = "name=Test%20account";

    let response = app.post_account(req_body.to_string()).await;
    assert_eq!(201, response.status().as_u16());

    let response = app.post_account(req_body.to_string()).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    let body = from_utf8(&body);
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, status);
}

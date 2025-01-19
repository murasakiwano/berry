use std::str::from_utf8;

use berry::models::account::Account;
use reqwest::StatusCode;

use crate::helpers::spawn_app;

#[tokio::test]
async fn list_accounts_returns_empty_vec_if_there_are_no_accounts() {
    let app = spawn_app().await;
    app.delete_account(app.test_account.id.to_string()).await;

    let response = app.list_accounts().await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::OK, status);

    let body = response.bytes().await.unwrap();
    let body = from_utf8(&body).unwrap();
    tracing::info!(response_body = ?body);

    let accounts: Vec<Account> = serde_json::from_str(body).unwrap();

    assert!(accounts.is_empty());
}

#[tokio::test]
async fn list_accounts_returns_all_accounts_in_the_database() {
    let app = spawn_app().await;

    let body = "name=Test%20account";
    app.post_account(body.to_string()).await;

    let response = app.list_accounts().await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::OK, status);

    let body = response.bytes().await.unwrap();
    let accounts: Vec<Account> = serde_json::from_slice(&body).unwrap();

    assert_eq!(accounts.len(), 2);
}

use berry::models::account::Account;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn get_existing_account() {
    let app = spawn_app().await;

    let response = app.get_account(app.test_account.id.to_string()).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    let account: Account = serde_json::from_slice(&body).unwrap();

    assert_eq!(StatusCode::OK, status);
    assert_eq!(app.test_account.id, account.id());
}

#[tokio::test]
async fn get_unexisting_account_returns_not_found() {
    let app = spawn_app().await;

    let response = app.get_account(Uuid::new_v4().to_string()).await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::NOT_FOUND, status);
}

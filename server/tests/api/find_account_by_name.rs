use berry::models::account::Account;
use reqwest::StatusCode;

use crate::helpers::spawn_app;

#[tokio::test]
async fn find_existing_account() {
    let app = spawn_app().await;

    let response = app
        .find_account_by_name(app.test_account.name.to_string())
        .await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    let account: Account = serde_json::from_slice(&body).unwrap();

    assert_eq!(StatusCode::OK, status);
    assert_eq!(app.test_account.id, account.id());
    assert_eq!(app.test_account.name, *account.name());
}

#[tokio::test]
async fn find_unexisting_account_returns_not_found() {
    let app = spawn_app().await;

    let response = app.find_account_by_name("Nonexistent".to_string()).await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[tokio::test]
async fn empty_name_should_give_bad_request() {
    let app = spawn_app().await;

    let response = app.find_account_by_name("".to_string()).await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::BAD_REQUEST, status);
}

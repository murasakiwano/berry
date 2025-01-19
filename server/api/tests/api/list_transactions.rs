use std::str::from_utf8;

use berry::models::{account::Account, transaction::Transaction};
use reqwest::StatusCode;

use crate::helpers::{spawn_app, TestAccount};

#[tokio::test]
async fn list_transactions_returns_empty_vec_if_there_are_no_transactions() {
    let app = spawn_app().await;

    let response = app.list_transactions().await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::OK, status);

    let body = response.bytes().await.unwrap();
    let body = from_utf8(&body).unwrap();
    tracing::info!(response_body = ?body);

    let transactions: Vec<Transaction> = serde_json::from_str(body).unwrap();

    assert!(transactions.is_empty());
}

#[tokio::test]
async fn list_transactions_returns_all_transactions_in_the_database() {
    let app = spawn_app().await;

    let source_account = app.test_account();
    let destination_account = TestAccount::generate();
    let destination_account = app
        .post_account(format!(
            "name={}",
            destination_account.name.into_url_encoding()
        ))
        .await
        .bytes()
        .await
        .unwrap();
    let destination_account: Account = serde_json::from_slice(&destination_account).unwrap();
    let amount = 42;

    let body = format!(
        "title=Test%20transaction&amount={}&source_account_id={}&destination_account_id={}",
        amount,
        source_account.id,
        destination_account.id()
    );
    tracing::info!(request_body = ?body);

    app.post_transaction(body.clone()).await;
    app.post_transaction(body).await;

    let response = app.list_transactions().await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::OK, status);

    let body = response.bytes().await.unwrap();
    let transactions: Vec<Transaction> = serde_json::from_slice(&body).unwrap();

    assert_eq!(transactions.len(), 2);
}

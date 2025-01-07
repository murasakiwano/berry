use berry::models::{account::Account, transaction::Transaction};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::helpers::{spawn_app, TestAccount};

#[tokio::test]
async fn get_existing_transaction() {
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
        "title=Test%20transaction&amount_cents={}&source_account_id={}&destination_account_id={}",
        amount,
        source_account.id,
        destination_account.id()
    );
    tracing::info!(request_body = ?body);

    let response = app.post_transaction(body).await.bytes().await.unwrap();
    let transaction: Transaction = serde_json::from_slice(&response).unwrap();
    let expected_id = transaction.id();

    let response = app.get_transaction(transaction.id().to_string()).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    let transaction: Transaction = serde_json::from_slice(&body).unwrap();

    assert_eq!(StatusCode::OK, status);
    assert_eq!(expected_id, transaction.id());
}

#[tokio::test]
async fn get_unexisting_transaction_returns_not_found() {
    let app = spawn_app().await;

    let response = app.get_transaction(Uuid::new_v4().to_string()).await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::NOT_FOUND, status);
}

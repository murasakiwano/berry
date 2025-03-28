use berry::models::{account::Account, transaction::Transaction};
use reqwest::StatusCode;
use rust_decimal_macros::dec;

use crate::helpers::{spawn_app, TestAccount};

#[tokio::test]
async fn delete_existing_transaction_updates_account_balances() {
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

    let response = app.post_transaction(body).await.bytes().await.unwrap();
    let transaction: Transaction = serde_json::from_slice(&response).unwrap();

    let response = app.delete_transaction(transaction.id().to_string()).await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::NO_CONTENT, status);

    let source_account = app
        .get_account(source_account.id.to_string())
        .await
        .bytes()
        .await
        .unwrap();
    let source_account: Account = serde_json::from_slice(&source_account).unwrap();
    let destination_account = app
        .get_account(destination_account.id().to_string())
        .await
        .bytes()
        .await
        .unwrap();
    let destination_account: Account = serde_json::from_slice(&destination_account).unwrap();

    assert_eq!(dec!(0), source_account.balance());
    assert_eq!(dec!(0), destination_account.balance());
}

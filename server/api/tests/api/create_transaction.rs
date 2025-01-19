use berry::models::account::Account;
use reqwest::StatusCode;
use rust_decimal_macros::dec;

use crate::helpers::{spawn_app, TestAccount};

#[tokio::test]
async fn accounts_must_have_their_balances_updated() {
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

    let amount = dec!(42);

    let body = format!(
        "title=Test%20transaction&amount={}&source_account_id={}&destination_account_id={}",
        amount,
        source_account.id,
        destination_account.id()
    );
    tracing::info!(request_body = ?body);

    let response = app.post_transaction(body).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::CREATED, status);

    // Check if account balances were updated
    let source_account = app
        .get_account(source_account.id.to_string())
        .await
        .bytes()
        .await
        .expect("body is not utf8");
    let source_account: Account = serde_json::from_slice(&source_account).unwrap();
    assert_eq!(dec!(-42), source_account.balance());

    let destination_account = app
        .get_account(destination_account.id().to_string())
        .await
        .bytes()
        .await
        .expect("body is not utf8");
    let destination_account: Account = serde_json::from_slice(&destination_account).unwrap();
    assert_eq!(dec!(42), destination_account.balance());
}

#[tokio::test]
async fn source_account_must_exist() {
    let app = spawn_app().await;
    let destination_account = app.test_account();
    let amount = dec!(42);

    let body = format!(
        "title=Test%20transaction&amount={}&destination_account_id={}",
        amount, destination_account.id
    );
    tracing::info!(request_body = ?body);
    let response = app.post_transaction(body).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, status);
}

#[tokio::test]
async fn destination_account_must_exist() {
    let app = spawn_app().await;
    let source_account = app.test_account();
    let amount = dec!(42);

    let body = format!(
        "title=Test%transaction&amount={}&source_account_id={}",
        amount, source_account.id
    );
    tracing::info!(request_body = ?body);
    let response = app.post_transaction(body).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, status);
}

#[tokio::test]
async fn transaction_title_is_required() {
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

    let amount = dec!(42);

    let body = format!(
        "amount={}&source_account_id={}&destination_account_id={}",
        amount,
        source_account.id,
        destination_account.id()
    );
    tracing::info!(request_body = ?body);

    let response = app.post_transaction(body).await;
    let status = response.status().as_u16();
    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, status);
}

#[tokio::test]
async fn posting_date_is_allowed() {
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

    let amount = dec!(42);

    let body = format!(
        "title=Test%20transaction&amount={}&source_account_id={}&destination_account_id={}&posting-date=2025-01-01T18%3A00%3A00",
        amount,
        source_account.id,
        destination_account.id()
    );
    tracing::info!(request_body = ?body);

    let response = app.post_transaction(body).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::CREATED, status);
}

#[tokio::test]
async fn category_is_allowed() {
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
    tracing::debug!(destination_account=?destination_account);
    let destination_account: Account = serde_json::from_slice(&destination_account).unwrap();

    let amount = dec!(42);

    let body = format!(
        "title=Test%20transaction&amount={}&source_account_id={}&destination_account_id={}&category=expenses",
        amount,
        source_account.id,
        destination_account.id()
    );
    tracing::info!(request_body = ?body);

    let response = app.post_transaction(body).await;
    let status = response.status().as_u16();
    let body = response.bytes().await.unwrap();
    tracing::info!(response_body = ?body);

    assert_eq!(StatusCode::CREATED, status);
}

#[tokio::test]
async fn creating_multiple_transactions_correctly_updates_the_balances() {
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

    let amount = dec!(42);

    let body = format!(
        "title=Test%20transaction&amount={}&source_account_id={}&destination_account_id={}",
        amount,
        source_account.id,
        destination_account.id()
    );
    tracing::info!(request_body = ?body);

    app.post_transaction(body).await;

    let amount = dec!(45);

    let body = format!(
        "title=Test%20transaction&amount={}&source_account_id={}&destination_account_id={}",
        amount,
        destination_account.id(),
        source_account.id,
    );
    tracing::info!(request_body = ?body);

    app.post_transaction(body).await;

    let amount = dec!(50);

    let body = format!(
        "title=Test%20transaction&amount={}&source_account_id={}&destination_account_id={}",
        amount,
        source_account.id,
        destination_account.id(),
    );
    tracing::info!(request_body = ?body);

    app.post_transaction(body).await;

    // Check if account balances were updated
    let source_account = app
        .get_account(source_account.id.to_string())
        .await
        .bytes()
        .await
        .expect("body is not utf8");
    let source_account: Account = serde_json::from_slice(&source_account).unwrap();
    assert_eq!(dec!(-47), source_account.balance());

    let destination_account = app
        .get_account(destination_account.id().to_string())
        .await
        .bytes()
        .await
        .expect("body is not utf8");
    let destination_account: Account = serde_json::from_slice(&destination_account).unwrap();
    assert_eq!(dec!(47), destination_account.balance());
}

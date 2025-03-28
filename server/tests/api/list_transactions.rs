use std::str::from_utf8;

use berry::{models::transaction::Transaction, service::PaginationParameters};
use reqwest::StatusCode;

use crate::helpers::{generate_fake_transaction, spawn_app};

#[tokio::test]
async fn list_transactions_returns_empty_vec_if_there_are_no_transactions() {
    let app = spawn_app().await;

    let response = app.list_transactions(None).await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::OK, status);

    let body = response.bytes().await.unwrap();
    let body = from_utf8(&body).unwrap();
    tracing::info!(response_body = ?body);

    let transactions: Vec<Transaction> = serde_json::from_str(body).unwrap();

    assert!(transactions.is_empty());
}

#[tokio::test]
async fn list_transactions_returns_paginated_transactions_from_the_database() {
    let app = spawn_app().await;

    for _ in 0..30 {
        generate_fake_transaction(&app).await;
    }

    let response = app
        .list_transactions(Some(PaginationParameters {
            limit: 20,
            offset: 1,
        }))
        .await;
    let status = response.status().as_u16();

    assert_eq!(StatusCode::OK, status);

    let body = response.bytes().await.unwrap();
    let transactions: Vec<Transaction> = serde_json::from_slice(&body).unwrap();

    assert_eq!(transactions.len(), 20);

    let response = app
        .list_transactions(Some(PaginationParameters {
            limit: 20,
            offset: 2,
        }))
        .await;

    let status = response.status().as_u16();

    assert_eq!(StatusCode::OK, status);

    let body = response.bytes().await.unwrap();
    let transactions: Vec<Transaction> = serde_json::from_slice(&body).unwrap();

    assert_eq!(transactions.len(), 10);
}

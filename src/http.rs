use std::sync::Arc;

use anyhow::Context;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    config::Settings,
    handlers::{
        create_account, create_transaction, delete_account, get_account, get_transaction,
        list_accounts, list_transactions, rename_account, update_account_balance,
    },
    sqlite::Sqlite,
};

/// Global state shared by all request handlers
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Arc<Sqlite>,
}

/// The app's HTTP server
pub struct Server {
    router: axum::Router,
    listener: tokio::net::TcpListener,
}

impl Server {
    /// Returns a new Server bound to the port specified in `config`
    pub async fn new(config: Settings) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let pool = Sqlite::new(&config.database_url).await?;

        let state = AppState {
            pool: Arc::new(pool),
        };

        let router = axum::Router::new()
            .nest("/api", api_routes())
            .layer(trace_layer)
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            config.application.host, config.application.port
        ))
        .await
        .with_context(|| format!("failed to listen on {}", config.application.port))?;

        Ok(Self { router, listener })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/accounts", post(create_account))
        .route("/accounts", get(list_accounts))
        .route("/accounts/{id}", get(get_account))
        .route("/accounts/{id}", delete(delete_account))
        .route("/accounts/{id}/name", patch(rename_account))
        .route("/accounts/{id}/balance", patch(update_account_balance))
        .route("/transactions", post(create_transaction))
        .route("/transactions", get(list_transactions))
        .route("/transactions/{id}", get(get_transaction))
}

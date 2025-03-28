use std::sync::Arc;

use anyhow::Context;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{configuration::Settings, handlers, service::BerryService};

/// Global state shared by all request handlers
#[derive(Debug, Clone)]
pub struct AppState {
    pub service: Arc<BerryService>,
}

/// The app's HTTP server
pub struct Server {
    router: axum::Router,
    listener: tokio::net::TcpListener,
    port: u16,
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

        let pool = BerryService::new(&config.database).await?;

        let state = AppState {
            service: Arc::new(pool),
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
        let port = listener
            .local_addr()
            .with_context(|| "failed to get listener's local_addr")?
            .port();
        tracing::info!(port = ?port, "listening on host");

        Ok(Self {
            router,
            listener,
            port,
        })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/accounts", post(handlers::create_account))
        .route("/accounts", get(handlers::list_accounts))
        .route(
            "/accounts/find-by-name",
            get(handlers::find_account_by_name),
        )
        .route("/accounts/{id}", get(handlers::get_account))
        .route("/accounts/{id}", delete(handlers::delete_account))
        .route("/accounts/{id}/name", patch(handlers::rename_account))
        .route("/transactions", post(handlers::create_transaction))
        .route("/transactions", get(handlers::list_transactions))
        .route("/transactions/{id}", get(handlers::get_transaction))
        .route("/transactions/{id}", delete(handlers::delete_transaction))
}

use anyhow::Context;
use configuration::get_configuration;
use server::Server;

pub mod cli;
pub mod configuration;
pub mod handlers;
pub mod models;
pub mod server;
pub mod service;
pub mod telemetry;
mod utils;

/// Spin up the server -- to be reused whenever I need it
pub async fn setup_server() -> anyhow::Result<Server> {
    dotenvy::dotenv().ok();
    let configuration = get_configuration(None).with_context(|| "Failed to read configuration")?;
    let subscriber =
        telemetry::get_subscriber("berry".to_string(), "info".to_string(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let server = Server::new(configuration).await?;

    Ok(server)
}

use anyhow::Context;
use berry::{configuration::get_configuration, server::Server, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let configuration = get_configuration().with_context(|| "Failed to read configuration")?;
    let subscriber =
        telemetry::get_subscriber("berry".to_string(), "info".to_string(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let server = Server::new(configuration).await?;

    server.run().await
}

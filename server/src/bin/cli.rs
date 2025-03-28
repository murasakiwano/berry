use std::path::PathBuf;

use berry::{cli::Cli, configuration::get_configuration, service::BerryService, telemetry};
use clap::Parser;
use color_eyre::eyre::eyre;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let subscriber =
        telemetry::get_subscriber("cli".to_string(), "debug".to_string(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config = get_configuration(Some(PathBuf::from("../../")))?;
    let service = BerryService::new(&config.database)
        .await
        .map_err(|e| eyre!("failed to create berry service: {:?}", e))?;

    let cli = Cli::parse();

    cli.run(service).await
}

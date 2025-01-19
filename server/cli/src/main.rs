use std::path::PathBuf;

use berry::{configuration::get_configuration, telemetry};
use clap::Parser;
use cli::Cli;
use color_eyre::eyre::eyre;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    println!("{:?}", std::env::var("RUST_LOG"));

    let subscriber =
        telemetry::get_subscriber("cli".to_string(), "debug".to_string(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config = get_configuration(Some(PathBuf::from("../server")))?;
    let service = berry::service::BerryService::new(&config.database)
        .await
        .map_err(|e| eyre!("failed to create berry service: {:?}", e))?;

    // const CREDIT_CARD_FIELDS: &[&str] = &["date", "title", "amount"];
    // const BANK_STATEMENT_FIELDS: &[&str] = &["Data", "Valor", "Identificador", "Descrição"];

    let cli = Cli::parse();

    cli.run(service).await
}

use std::path::PathBuf;

use chrono::NaiveDate;
use clap::Parser;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    models::{
        account::AccountName,
        transaction::{CreateTransactionRequest, TransactionTitle},
    },
    service::BerryService,
};

#[derive(Deserialize)]
struct CreditCardTransaction {
    #[serde(alias = "Data", deserialize_with = "deserialize_date")]
    date: NaiveDate,
    #[serde(alias = "Descrição")]
    title: String,
    #[serde(alias = "Valor")]
    amount: Decimal,
    #[serde(default)]
    category: Option<String>,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.contains("/") {
        NaiveDate::parse_from_str(&s, "%d/%m/%Y").map_err(serde::de::Error::custom)
    } else {
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    /// The input file containing financial data.
    #[arg(short, long)]
    file: PathBuf,

    /// Name of the source account
    #[arg(short, long)]
    source_account: String,
}

impl Cli {
    pub async fn run(&self, service: BerryService) -> color_eyre::Result<()> {
        let mut rdr = csv::Reader::from_path(&self.file)?;
        let credit_card_account_name = AccountName::new(&self.source_account)?;
        let credit_card_account = service
            .get_or_create_account(&credit_card_account_name)
            .await?;

        for result in rdr.deserialize() {
            let transaction: CreditCardTransaction = result?;

            let destination_account_name = AccountName::new(&transaction.title)?;
            let destination_account = service
                .get_or_create_account(&destination_account_name)
                .await?;
            let title = TransactionTitle::new(&transaction.title)?;
            let req = CreateTransactionRequest::new(
                title,
                transaction.amount,
                credit_card_account.id(),
                destination_account.id(),
                transaction.category,
                Some(transaction.date.into()),
            );
            let response = service.create_transaction(&req).await;
            match response {
                Ok(tx) => tracing::info!(transaction = ?tx, "Transaction successfully created"),
                Err(err) => tracing::error!(error = %err, "Failed to create transaction"),
            }
        }

        Ok(())
    }
}

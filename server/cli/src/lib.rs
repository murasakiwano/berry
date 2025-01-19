use std::{
    fs::File,
    io::{self, Read as _},
    path::PathBuf,
    str::Lines,
};

use berry::{
    models::{
        account::{Account, AccountName},
        transaction::{CreateTransactionRequest, TransactionTitle},
    },
    service::BerryService,
};
use chrono::{NaiveDate, NaiveTime};
use clap::Parser;
use color_eyre::eyre::{bail, Context as _};
use rust_decimal::Decimal;

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
        let path_string = self.file.clone().to_string_lossy().to_string();
        let mut file = match File::open(self.file.clone()) {
            Ok(f) => f,
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => {
                    bail!("file {:?} does not exist", path_string)
                }
                _ => {
                    bail!("failed to open file {}: {:?}", path_string, error)
                }
            },
        };

        let mut csv = String::new();
        let size = file.read_to_string(&mut csv)?;

        if size == 0 {
            bail!("file {} is empty", path_string);
        }

        parse_credit_card_file(csv.lines(), service, &self.source_account).await
    }
}

async fn parse_credit_card_file(
    mut csv_lines: Lines<'_>,
    service: BerryService,
    source_account_name: &str,
) -> color_eyre::Result<()> {
    let header = csv_lines
        .next()
        .ok_or(color_eyre::eyre::eyre!("File is empty!"))?;
    let header_without_category = "date,title,amount";
    let header_with_category = "date,category,title,amount";

    let credit_card_account_name = AccountName::new(source_account_name)
        .context("account name should have been parsed correctly")?;
    let credit_card_account = service
        .get_or_create_account(&credit_card_account_name)
        .await?;

    let parse_category = if header == header_without_category {
        false
    } else if header == header_with_category {
        true
    } else {
        bail!(
            "the header of the file must be either \"{}\" or \"{}\"",
            header_without_category,
            header_with_category
        )
    };

    for l in csv_lines {
        parse_credit_card_line(l, &service, &credit_card_account, parse_category).await?
    }

    Ok(())
}

/// Nubank started to send the credit card bill without the category of each expense
async fn parse_credit_card_line(
    l: &str,
    service: &BerryService,
    credit_card_account: &Account,
    parse_category: bool,
) -> color_eyre::Result<()> {
    let mut fields = l.split(',');
    let Some(date) = fields.next() else {
        bail!("line must not be empty")
    };
    let category = if parse_category {
        match fields.next() {
            Some(c) => Some(c.to_string()),
            None => bail!("line does not contain the expected number of fields: {}", l),
        }
    } else {
        None
    };
    let Some(title) = fields.next() else {
        bail!("line does not contain the expected number of fields: {}", l)
    };
    let Some(amount) = fields.next() else {
        bail!("line does not contain the expected number of fields: {}", l)
    };

    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .context(format!("failed to parse date: {}", date))?
        .and_time(NaiveTime::default());

    // Here, `title` will become an account
    let account_name =
        AccountName::new(title).context("failed to parse title into an AccountName")?;
    let destination_account = service
        .get_or_create_account(&account_name)
        .await
        .context("failed to get or create the destination account of the transaction")?;

    let amount = Decimal::from_str_exact(amount).context("failed to parse amount into decimal")?;

    let txreq = CreateTransactionRequest::new(
        TransactionTitle::new(title)?,
        amount,
        credit_card_account.id(),
        destination_account.id(),
        category,
        Some(date),
    );

    let transaction = service
        .create_transaction(&txreq)
        .await
        .context("failed to create transaction")?;
    tracing::info!(transaction = ?transaction, "created transaction");

    Ok(())
}

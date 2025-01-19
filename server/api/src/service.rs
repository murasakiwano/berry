use anyhow::{anyhow, Context};
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use sqlx::PgPool;
use uuid::Uuid;

use crate::configuration::DatabaseSettings;
use crate::models::account::GetAccountByNameError;
use crate::models::account::GetOrCreateAccountError;
use crate::models::account::ListAccountsError;
use crate::models::account::{Account, AccountName, CreateAccountError, GetAccountError};
use crate::models::account::{DeleteAccountError, UpdateAccountError};
use crate::models::transaction::ListTransactionsError;
use crate::models::transaction::{
    CreateTransactionError, CreateTransactionRequest, DeleteTransactionError, GetTransactionError,
    Transaction, TransactionTitle,
};

#[derive(Debug, Clone)]
pub struct BerryService {
    pub pool: PgPool,
}

impl BerryService {
    pub async fn new(config: &DatabaseSettings) -> Result<BerryService, anyhow::Error> {
        Ok(BerryService {
            pool: get_connection_pool(config),
        })
    }

    /// Store an [Account] in the database
    async fn save_account(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        name: &AccountName,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let name = &name.to_string();
        let query = sqlx::query!("INSERT INTO accounts (id, name) VALUES ($1, $2)", id, name,);
        tx.execute(query).await?;
        Ok(id)
    }

    /// Store a [Transaction] in the database
    async fn save_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        req: &CreateTransactionRequest,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let title = &req.title().to_string();
        let category = req.category().as_ref().map(|c| c.to_string());
        let amount = req.amount();
        let posting_date = Utc::now();
        let query = sqlx::query!(
            "INSERT INTO postings (
id, title, amount, source_account_id, destination_account_id, category, posting_date
) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            id,
            title,
            amount,
            req.source_account_id(),
            req.destination_account_id(),
            category,
            posting_date
        );
        tx.execute(query).await?;

        Ok(id)
    }

    /// Checks if both [Account]s with the given ids exist
    async fn check_if_accounts_exist(
        &self,
        source_account_id: Uuid,
        destination_account_id: Uuid,
    ) -> Result<(), CreateTransactionError> {
        self.get_account_by_id(source_account_id)
            .await
            .map_err(|e| match e {
                GetAccountError::NotFound { id } => {
                    CreateTransactionError::SourceAccountNotFound { id }
                }
                GetAccountError::Unknown(e) => CreateTransactionError::Unknown(e),
            })?;
        self.get_account_by_id(destination_account_id)
            .await
            .map_err(|e| match e {
                GetAccountError::NotFound { id } => {
                    CreateTransactionError::DestinationAccountNotFound { id }
                }
                GetAccountError::Unknown(e) => CreateTransactionError::Unknown(e),
            })?;

        Ok(())
    }

    /// Persists the [Account] balance change to the database
    ///
    /// # Errors
    ///
    /// - [UpdateAccountError::NotFound] if no [Account] exists for the given id
    /// - [UpdateAccountError::Unknown] in case any other error occurred
    async fn add_balance_to_account(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        id: Uuid,
        new_balance: Decimal,
    ) -> Result<Account, UpdateAccountError> {
        let row = sqlx::query!(
            "UPDATE accounts SET balance = balance + $1 WHERE id = $2 RETURNING *",
            new_balance,
            id
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UpdateAccountError::NotFound { id },
            _ => UpdateAccountError::Unknown(e.into()),
        })?;

        Ok(Account::new(
            row.id,
            AccountName::new(&row.name).unwrap(),
            row.balance,
        ))
    }

    /// Helper to start a PostgreSQL transaction and avoid boilerplate
    async fn start_psql_transaction(
        &self,
    ) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, anyhow::Error> {
        self.pool
            .begin()
            .await
            .context("Failed to start PostgreSQL transaction")
    }

    /// Persists an [Account] to the database
    ///
    /// # Errors
    ///
    /// - [CreateAccountError::Duplicate] if an [Account] with the given name already exists
    /// - [CreateAccountError::Unknown] if another kind of error occurred
    pub async fn create_account(&self, req: &AccountName) -> Result<Account, CreateAccountError> {
        let mut tx = self.start_psql_transaction().await?;

        let account_id = self.save_account(&mut tx, req).await.map_err(|e| {
            if is_unique_constraing_violation(&e) {
                CreateAccountError::Duplicate { name: req.clone() }
            } else {
                anyhow!(e)
                    .context(format!("failed to save account with name {:?}", req))
                    .into()
            }
        })?;

        tx.commit()
            .await
            .context("failed to commit PostgreSQL transaction")?;

        tracing::info!(?account_id, "Successfully created account");
        Ok(Account::new(account_id, req.clone(), dec!(0)))
    }

    /// List all accounts in the database.
    ///
    /// This does not support filters, **yet**. It will return an empty [Vec] if there are no
    /// accounts in the database.
    pub async fn list_accounts(&self) -> Result<Vec<Account>, ListAccountsError> {
        let rows = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ListAccountsError::Unknown(e.into()))?;

        Ok(rows
            .iter()
            .map(|r| {
                // Account names in the database are, by design, valid
                let account_name = AccountName::new(&r.name).unwrap();
                // Account ids in the database are, by design, valid UUIDs
                tracing::debug!(id = ?r.id, account_name = ?account_name);
                Account::new(r.id, account_name, r.balance)
            })
            .collect())
    }

    /// Fetch an [Account] by its id.
    ///
    /// # Errors
    ///
    /// - [GetAccountError::NotFound] if no [Account] with the given id exists
    /// - [GetAccountError::Unknown] in case any other error occurred
    pub async fn get_account_by_id(&self, id: Uuid) -> Result<Account, GetAccountError> {
        let row = sqlx::query!("SELECT * FROM accounts WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => GetAccountError::NotFound { id },
                err => GetAccountError::Unknown(err.into()),
            })?;

        let account_name = AccountName::new(&row.name)
            .map_err(|e| GetAccountError::Unknown(e.into()))
            .context(format!("Failed to create account name from {}", row.name))?;
        let account = Account::new(row.id, account_name, row.balance);

        tracing::debug!(?account, "Found account");

        Ok(account)
    }

    /// Fetch an [Account] by its name.
    ///
    /// # Errors
    ///
    /// - [GetAccountByNameError::NotFound] if an [Account] with the given name does not exist
    /// - [GetAccountByNameError::Unknown] in case any other error occurred
    pub async fn get_account_by_name(
        &self,
        name: &AccountName,
    ) -> Result<Account, GetAccountByNameError> {
        let row = sqlx::query!("SELECT * FROM accounts WHERE name = $1", name.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => GetAccountByNameError::NotFound { name: name.clone() },
                err => GetAccountByNameError::Unknown(err.into()),
            })?;

        let account_name = AccountName::new(&row.name)
            .map_err(|e| GetAccountByNameError::Unknown(e.into()))
            .context(format!("Failed to create account name from {}", row.name))?;
        let account = Account::new(row.id, account_name, row.balance);

        tracing::debug!(?account, "Found account");

        Ok(account)
    }

    /// Fetch an [Account] by its name. If it does not exist, create it.
    ///
    /// # Errors
    ///
    /// - [GetOrCreateAccountError::Duplicate] in case there is another [Account] in the database with
    /// - [GetOrCreateAccountError::Unknown] if any other error occurred
    pub async fn get_or_create_account(
        &self,
        name: &AccountName,
    ) -> Result<Account, GetOrCreateAccountError> {
        match self.get_account_by_name(name).await {
            Ok(account) => Ok(account),
            Err(err) => match err {
                GetAccountByNameError::NotFound { name } => self
                    .create_account(&name)
                    .await
                    .map_err(GetOrCreateAccountError::from),
                GetAccountByNameError::Unknown(err) => Err(GetOrCreateAccountError::Unknown(err)),
            },
        }
    }

    /// Rename an [Account].
    ///
    /// # Errors
    ///
    /// - [UpdateAccountError::NotFound] if no [Account] with the given id exists
    /// - [UpdateAccountError::Duplicate] in case there is another [Account] in the database with
    ///   the new name
    /// - [UpdateAccountError::Unknown] in case any other error occurred
    pub async fn rename_account(
        &self,
        id: Uuid,
        new_name: AccountName,
    ) -> Result<(), UpdateAccountError> {
        let result = sqlx::query!(
            "
UPDATE accounts
SET name = $1
WHERE id = $2
",
            new_name.to_string(),
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if is_unique_constraing_violation(&e) {
                UpdateAccountError::Duplicate {
                    name: new_name.to_string(),
                }
            } else {
                UpdateAccountError::Unknown(e.into())
            }
        })?;

        if result.rows_affected() == 0 {
            Err(UpdateAccountError::NotFound { id })
        } else {
            tracing::info!(new_name = new_name.to_string(), account_id = ?id, "Successfully renamed account");
            Ok(())
        }
    }

    /// Update an [Account]'s balance.
    ///
    /// This adds the given `balance_to_add` to an [Account]'s balance, be it positive or negative.
    ///
    /// # Errors
    ///
    /// - [UpdateAccountError::NotFound] if no [Account] with the given id exists
    /// - [UpdateAccountError::Unknown] if any other kind of error occurred
    pub async fn update_account_balance(
        &self,
        id: Uuid,
        balance_to_add: Decimal,
    ) -> Result<Account, UpdateAccountError> {
        let mut tx = self.start_psql_transaction().await?;
        let account = self
            .add_balance_to_account(&mut tx, id, balance_to_add)
            .await?;
        tx.commit()
            .await
            .context("Failed to commit PostgreSQL transaction")?;

        Ok(account)
    }

    /// Delete an [Account].
    ///
    /// # Errors
    ///
    /// - [DeleteAccountError::NotFound] if no [Account] with the given id exists
    /// - [DeleteAccountError::Unknown] in case any other kind of error occurred
    pub async fn delete_account(&self, id: Uuid) -> Result<(), DeleteAccountError> {
        let result = sqlx::query!("DELETE FROM accounts WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DeleteAccountError::Unknown(e.into()))?;

        if result.rows_affected() == 0 {
            Err(DeleteAccountError::NotFound { id })
        } else {
            tracing::info!(?id, "Successfully deleted account");
            Ok(())
        }
    }

    /// Create a [Transaction].
    ///
    /// This also updates the balances of the [Account]s involved (add to the destination account,
    /// subtract from the source account).
    ///
    /// # Errors
    ///
    /// - [CreateTransactionError::SourceAccountNotFound] if the source account does not exist
    /// - [CreateTransactionError::DestinationAccountNotFound] if the destination account does not
    ///   exist
    /// - [CreateTransactionError::Unknown] if any other kind of error occurred
    pub async fn create_transaction(
        &self,
        req: &CreateTransactionRequest,
    ) -> Result<Transaction, CreateTransactionError> {
        self.check_if_accounts_exist(req.source_account_id(), req.destination_account_id())
            .await?;
        let posting_date = req
            .posting_date()
            .map(|d| d.and_utc())
            .unwrap_or_else(Utc::now);

        let mut tx = self.start_psql_transaction().await?;

        let transaction_id = self.save_transaction(&mut tx, req).await.map_err(|e| {
            anyhow!(e).context(format!(
                "failed to save transaction with title {:?}",
                req.title()
            ))
        })?;

        tracing::debug!("created transaction, updating account balances...");

        let source_account = self
            .add_balance_to_account(&mut tx, req.source_account_id(), -req.amount())
            .await
            .context("failed to reset source account balance")?;
        let destination_account = self
            .add_balance_to_account(&mut tx, req.destination_account_id(), req.amount())
            .await
            .context("failed to reset destination account balance")?;

        tracing::debug!(source_balance = ?source_account.balance(), destination_balance = ?destination_account.balance(), "updated account balances");

        tx.commit()
            .await
            .context("failed to commit PostgreSQL transaction")?;

        tracing::info!(?transaction_id, "Successfully created transaction");

        Ok(Transaction::new(
            transaction_id,
            req.title().clone(),
            req.amount(),
            req.source_account_id(),
            req.destination_account_id(),
            req.category().clone(),
            posting_date,
        ))
    }

    /// Delete a [Transaction].
    ///
    /// This is the opposite operation of creating a transaction, so it also subtracts the amount
    /// from the destination account's balance and adds the amount to the source account's balance.
    ///
    /// # Errors
    ///
    /// - [DeleteTransactionError::TransactionNotFound] if no [Transaction] with the given id
    ///   exists
    /// - [DeleteTransactionError::Unknown] in casy any other kind of error occurred
    pub async fn delete_transaction(&self, id: Uuid) -> Result<(), DeleteTransactionError> {
        let mut tx = self.start_psql_transaction().await?;
        let row = sqlx::query!(
            "
         DELETE FROM \"postings\"
         WHERE id = $1
         RETURNING source_account_id, destination_account_id, amount
         ",
            id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DeleteTransactionError::TransactionNotFound { id },
            e => DeleteTransactionError::Unknown(e.into()),
        })?;

        let (source_account_id, destination_acount_id) =
            (row.source_account_id, row.destination_account_id);

        self.add_balance_to_account(&mut tx, source_account_id, row.amount)
            .await
            .context("failed to reset source account balance")?;
        self.add_balance_to_account(&mut tx, destination_acount_id, -row.amount)
            .await
            .context("failed to reset destination account balance")?;

        tx.commit()
            .await
            .context("failed to commit PostgreSQL transaction")?;

        Ok(())
    }

    /// Fetch a [Transaction] by its id.
    ///
    /// # Errors
    ///
    /// - [GetTransactionError::TransactionNotFound] if a transaction with the given id does not exist
    /// - [GetTransactionError::Unknown] if any other kind of error occurred
    pub async fn get_transaction_by_id(
        &self,
        id: Uuid,
    ) -> Result<Transaction, GetTransactionError> {
        let row = sqlx::query!("SELECT * FROM postings WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => GetTransactionError::TransactionNotFound { id },
                _ => GetTransactionError::Unknown(e.into()),
            })?;

        let transaction_title = TransactionTitle::new(&row.title)
            .map_err(|e| GetTransactionError::Unknown(e.into()))?;

        let transaction = Transaction::new(
            id,
            transaction_title,
            row.amount,
            row.source_account_id,
            row.destination_account_id,
            row.category,
            row.posting_date,
        );
        tracing::info!(
            ?id,
            ?row.source_account_id,
            ?row.destination_account_id,
            "Successfully created transaction"
        );

        Ok(transaction)
    }

    /// List all transactions in the database.
    ///
    /// This does not support filters, **yet**. It will return an empty [Vec] if there are no
    /// transactions in the database.
    pub async fn list_transactions(&self) -> Result<Vec<Transaction>, ListTransactionsError> {
        let rows = sqlx::query!("SELECT * FROM \"postings\"")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ListTransactionsError::Unknown(e.into()))?;

        tracing::debug!(?rows, "Got rows from the database");

        rows.iter()
            .map(|r| {
                let transaction_title = TransactionTitle::new(&r.title)
                    .map_err(|e| ListTransactionsError::Unknown(e.into()))?;

                let transaction = Transaction::new(
                    r.id,
                    transaction_title,
                    r.amount,
                    r.source_account_id,
                    r.destination_account_id,
                    r.category.clone(),
                    r.posting_date,
                );
                tracing::info!(
                    ?r.id,
                    ?r.source_account_id,
                    ?r.destination_account_id,
                    "Successfully retrieved transaction"
                );

                Ok(transaction)
            })
            .collect()
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "23505";

/// Check if an error happened due to a unique constraint violation.
///
/// This means that the record had a duplicate.
fn is_unique_constraing_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            return code == UNIQUE_CONSTRAINT_VIOLATION_CODE;
        }
    }

    false
}

/// Util to initialize a [PgPool]
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

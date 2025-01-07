use anyhow::{anyhow, Context};
use chrono::Utc;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use sqlx::PgPool;
use uuid::Uuid;

use crate::configuration::DatabaseSettings;
use crate::models::account::ListAccountsError;
use crate::models::account::{Account, AccountName, CreateAccountError, GetAccountError};
use crate::models::account::{DeleteAccountError, UpdateAccountError};
use crate::models::transaction::ListTransactionsError;
use crate::models::transaction::{
    CreateTransactionError, CreateTransactionRequest, DeleteTransactionError, GetTransactionError,
    Transaction, TransactionTitle,
};

#[derive(Debug, Clone)]
pub struct BerryRepo {
    pub pool: PgPool,
}

impl BerryRepo {
    pub async fn new(config: &DatabaseSettings) -> Result<BerryRepo, anyhow::Error> {
        Ok(BerryRepo {
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
        let amount_cents = req.amount_cents();
        let posting_date = Utc::now();
        let query = sqlx::query!(
            "INSERT INTO postings (
id, title, amount_cents, source_account_id, destination_account_id, category, posting_date
) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            id,
            title,
            amount_cents,
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
        new_balance: i64,
    ) -> Result<Account, UpdateAccountError> {
        let row = sqlx::query!(
            "UPDATE accounts SET balance_cents = balance_cents + $1 WHERE id = $2 RETURNING *",
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
            row.balance_cents,
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
        Ok(Account::new(account_id, req.clone(), 0))
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
                Account::new(r.id, account_name, r.balance_cents)
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

        tracing::debug!(row = ?row, ctx = "get_account_by_id", "Got row from database");

        let account_name = AccountName::new(&row.name)
            .map_err(|e| GetAccountError::Unknown(e.into()))
            .context(format!("Failed to create account name from {}", row.name))?;
        let account = Account::new(row.id, account_name, row.balance_cents);

        tracing::debug!(?account, "Found account");

        Ok(account)
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
        balance_to_add: i64,
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
            .add_balance_to_account(&mut tx, req.source_account_id(), -req.amount_cents())
            .await
            .context("failed to reset source account balance")?;
        let destination_account = self
            .add_balance_to_account(&mut tx, req.destination_account_id(), req.amount_cents())
            .await
            .context("failed to reset destination account balance")?;

        tracing::debug!(source_balance = ?source_account.balance_cents(), destination_balance = ?destination_account.balance_cents(), "updated account balances");

        tx.commit()
            .await
            .context("failed to commit PostgreSQL transaction")?;

        tracing::info!(?transaction_id, "Successfully created transaction");

        Ok(Transaction::new(
            transaction_id,
            req.title().clone(),
            req.amount_cents(),
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
         RETURNING source_account_id, destination_account_id, amount_cents
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

        self.add_balance_to_account(&mut tx, source_account_id, row.amount_cents)
            .await
            .context("failed to reset source account balance")?;
        self.add_balance_to_account(&mut tx, destination_acount_id, -row.amount_cents)
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
            row.amount_cents,
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
                    r.amount_cents,
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

#[cfg(test)]
mod tests {
    use secrecy::SecretString;
    use sqlx::Executor;
    use sqlx::{Connection, PgConnection};
    use uuid::Uuid;

    use crate::{
        configuration::DatabaseSettings,
        models::{
            account::AccountName,
            transaction::{CreateTransactionRequest, TransactionTitle},
        },
        repository::BerryRepo,
    };

    async fn setup_db() -> BerryRepo {
        tracing_subscriber::fmt()
            .with_test_writer() // Ensures logs go to stdout in tests
            .with_env_filter("debug") // Adjust the log level as needed
            .init();

        let database_name = Uuid::new_v4().to_string();
        let db_settings = DatabaseSettings {
            username: "postgres".to_string(),
            password: SecretString::from("password"),
            host: "localhost".to_string(),
            require_ssl: false,
            port: 5432,
            database_name,
        };
        let maintenance_settings = DatabaseSettings {
            database_name: "postgres".to_string(),
            username: "postgres".to_string(),
            password: SecretString::from("password"),
            ..db_settings.clone()
        };
        let mut connection = PgConnection::connect_with(&maintenance_settings.connect_options())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, db_settings.database_name).as_str())
            .await
            .expect("Failed to create database.");
        tracing::info!("Connecting to the database...");
        tracing::debug!(conn_opts = ?&db_settings.connect_options(), "Got database connect options");
        let mut conn = PgConnection::connect_with(&db_settings.connect_options())
            .await
            .unwrap();
        tracing::info!("Running migrations in the database...");
        sqlx::migrate!("./migrations").run(&mut conn).await.unwrap();
        tracing::info!("Migrations ran Successfully");

        BerryRepo::new(&db_settings).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_account() {
        let req = AccountName::new("Test account").unwrap();
        let db = setup_db().await;

        assert!(db.create_account(&req).await.is_ok());
    }

    #[tokio::test]
    async fn test_create_account_duplicate() {
        let req = AccountName::new("Test account").unwrap();
        let db = setup_db().await;

        assert!(db.create_account(&req).await.is_ok());
        assert!(db.create_account(&req).await.is_err());
    }

    #[tokio::test]
    async fn test_create_transaction() {
        let source_req = AccountName::new("Source account").unwrap();
        let destination_req = AccountName::new("Destination account").unwrap();
        let db = setup_db().await;

        let source_account = db.create_account(&source_req).await.unwrap();
        let destination_id = db.create_account(&destination_req).await.unwrap();

        let tx_req = CreateTransactionRequest::new(
            TransactionTitle::new("Test tx").unwrap(),
            42,
            source_account.id(),
            destination_id.id(),
            Some("Test category".to_string()),
            None,
        );

        let result = db.create_transaction(&tx_req).await;
        tracing::debug!(result = ?result, "result of trying to insert a tranasction to postgres");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_transaction_allows_duplicate_content() {
        let source_req = AccountName::new("Source account").unwrap();
        let destination_req = AccountName::new("Destination account").unwrap();
        let db = setup_db().await;

        let source_account = db.create_account(&source_req).await.unwrap();
        let destination_id = db.create_account(&destination_req).await.unwrap();

        let tx_req = CreateTransactionRequest::new(
            TransactionTitle::new("Test tx").unwrap(),
            42,
            source_account.id(),
            destination_id.id(),
            Some("Test category".to_string()),
            None,
        );

        assert!(db.create_transaction(&tx_req).await.is_ok());
        assert!(db.create_transaction(&tx_req).await.is_ok());
    }

    #[tokio::test]
    async fn test_update_account_balance() {
        let db = setup_db().await;
        let req = AccountName::new("Test account").unwrap();
        let account = db.create_account(&req).await.unwrap();

        assert!(db
            .update_account_balance(account.id(), 2)
            .await
            .is_ok_and(|a| a.balance_cents() == 2));
    }

    #[tokio::test]
    async fn test_update_nonexisting_account() {
        let db = setup_db().await;

        assert!(db.update_account_balance(Uuid::new_v4(), 2).await.is_err());
    }

    #[tokio::test]
    async fn test_rename_account() {
        let db = setup_db().await;
        let req = AccountName::new("Test account").unwrap();
        let account = db.create_account(&req).await.unwrap();

        assert!(db
            .rename_account(account.id(), AccountName::new("New account name").unwrap())
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_rename_unexisting_account() {
        let db = setup_db().await;

        assert!(db
            .rename_account(Uuid::new_v4(), AccountName::new("Dummy name").unwrap())
            .await
            .is_err())
    }

    #[tokio::test]
    async fn test_list_accounts_no_accounts() {
        let db = setup_db().await;

        let result = db.list_accounts().await;
        assert!(result.is_ok());

        let accounts = result.unwrap();
        assert!(
            accounts.is_empty(),
            "Expected account list to be empty, but it's not: {:?}",
            accounts
        );
    }

    #[tokio::test]
    async fn test_list_accounts_with_multiple_accounts() {
        let db = setup_db().await;

        let req = AccountName::new("First test account").unwrap();
        db.create_account(&req).await.unwrap();
        let req = AccountName::new("Second test account").unwrap();
        db.create_account(&req).await.unwrap();
        let req = AccountName::new("Third test account").unwrap();
        db.create_account(&req).await.unwrap();

        let result = db.list_accounts().await;
        assert!(result.is_ok());

        let accounts = result.unwrap();
        assert_eq!(
            accounts.len(),
            3,
            "Expected accounts to have length 3, but it has length {}",
            accounts.len()
        );
    }

    #[tokio::test]
    async fn test_get_transaction_by_id() {
        let source_req = AccountName::new("Source account").unwrap();
        let destination_req = AccountName::new("Destination account").unwrap();
        let db = setup_db().await;

        let source_account = db.create_account(&source_req).await.unwrap();
        let destination_id = db.create_account(&destination_req).await.unwrap();

        let tx_req = CreateTransactionRequest::new(
            TransactionTitle::new("Test tx").unwrap(),
            42,
            source_account.id(),
            destination_id.id(),
            Some("Test category".to_string()),
            None,
        );
        let transaction = db.create_transaction(&tx_req).await.unwrap();

        let found_transaction = db.get_transaction_by_id(transaction.id()).await;
        assert!(
            found_transaction.is_ok(),
            "Expected read operation to succeed, but it failed: {:?}",
            found_transaction
        );

        let found_transaction = found_transaction.unwrap();
        assert_eq!(
            found_transaction.id(),
            transaction.id(),
            "Expected found_transaction.id() to be {}, but it is {}",
            transaction.id(),
            found_transaction.id()
        );
    }

    #[tokio::test]
    async fn test_list_transactions_no_transaction() {
        let db = setup_db().await;
        let transactions = db.list_transactions().await;

        assert!(transactions.is_ok());

        let txs = transactions.unwrap();

        assert!(txs.is_empty(), "Expected txs to be empty, but it's not");
    }

    #[tokio::test]
    async fn test_list_transactions_multiple_transactions() {
        let db = setup_db().await;
        let source_req = AccountName::new("Source account").unwrap();
        let destination_req = AccountName::new("Destination account").unwrap();

        let source_account = db.create_account(&source_req).await.unwrap();
        let destination_id = db.create_account(&destination_req).await.unwrap();

        let tx_req = CreateTransactionRequest::new(
            TransactionTitle::new("Test tx").unwrap(),
            42,
            source_account.id(),
            destination_id.id(),
            Some("Test category".to_string()),
            None,
        );
        db.create_transaction(&tx_req).await.unwrap();
        let tx_req = CreateTransactionRequest::new(
            TransactionTitle::new("Test tx").unwrap(),
            45,
            destination_id.id(),
            source_account.id(),
            Some("Test category".to_string()),
            None,
        );
        db.create_transaction(&tx_req).await.unwrap();
        let tx_req = CreateTransactionRequest::new(
            TransactionTitle::new("Test tx").unwrap(),
            54,
            source_account.id(),
            destination_id.id(),
            Some("Test category".to_string()),
            None,
        );
        db.create_transaction(&tx_req).await.unwrap();

        let transactions = db.list_transactions().await;

        assert!(transactions.is_ok());
        let txs = transactions.unwrap();

        assert_eq!(
            txs.len(),
            3,
            "Expected txs length to be 3, got {}",
            txs.len()
        );
    }
}

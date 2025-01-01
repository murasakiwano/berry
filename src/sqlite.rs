use std::str::FromStr;

use anyhow::{anyhow, Context};
use chrono::{DateTime, Utc};
use sqlx::Executor;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use uuid::Uuid;

use crate::models::account::{Account, AccountName, CreateAccountError, GetAccountError};
use crate::models::account::{DeleteAccountError, UpdateAccountError};
use crate::models::transaction::{
    CreateTransactionError, CreateTransactionRequest, DeleteTransactionError, GetTransactionError,
    Transaction, TransactionTitle,
};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> Result<Sqlite, anyhow::Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }

    async fn save_account(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        name: &AccountName,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_as_string = id.to_string();
        let name = &name.to_string();
        let query = sqlx::query!(
            "INSERT INTO accounts (id, name) VALUES (?, ?)",
            id_as_string,
            name,
        );
        tx.execute(query).await?;
        Ok(id)
    }

    async fn save_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        req: &CreateTransactionRequest,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_as_string = id.to_string();
        let source_account_id = req.source_account_id().to_string();
        let destination_account_id = req.destination_account_id().to_string();
        let title = &req.title().to_string();
        let category = req.category().as_ref().map(|c| c.to_string());
        let amount_cents = req.amount_cents();
        let posting_date = Utc::now().timestamp();
        let query = sqlx::query!(
            "INSERT INTO \"postings\" (
id, title, amount_cents, source_account_id, destination_account_id, category, posting_date
) VALUES (?, ?, ?, ?, ?, ?, ?)",
            id_as_string,
            title,
            amount_cents,
            source_account_id,
            destination_account_id,
            category,
            posting_date
        );
        tx.execute(query).await?;

        Ok(id)
    }

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

    pub async fn create_account(&self, req: &AccountName) -> Result<Account, CreateAccountError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start SQLite transaction")?;

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
            .context("failed to commit SQLite transaction")?;

        tracing::info!(?account_id, "Successfully created account");
        Ok(Account::new(account_id, req.clone(), 0))
    }

    pub async fn list_accounts(&self) -> Result<Vec<Account>, anyhow::Error> {
        let rows = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|r| {
                // Account names in the database are, by design, valid
                let account_name = AccountName::new(&r.name).unwrap();
                // Account ids in the database are, by design, valid UUIDs
                let id = Uuid::from_str(&r.id).unwrap();
                tracing::debug!(id = ?id, account_name = ?account_name);
                Account::new(id, account_name, r.balance_cents)
            })
            .collect())
    }

    pub async fn get_account_by_id(&self, id: Uuid) -> Result<Account, GetAccountError> {
        let id_as_string = id.to_string();
        let row = sqlx::query!("SELECT * FROM accounts WHERE id = ?", id_as_string)
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
        let id = Uuid::from_str(&row.id).map_err(|e| GetAccountError::Unknown(e.into()))?;
        let account = Account::new(id, account_name, row.balance_cents);

        tracing::debug!(?account, "Found account");

        Ok(account)
    }

    pub async fn rename_account(&self, id: Uuid, new_name: &str) -> Result<(), UpdateAccountError> {
        let id_as_string = id.to_string();
        let result = sqlx::query!(
            "
UPDATE accounts
SET name = ?
WHERE id = ?
",
            new_name,
            id_as_string
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
            tracing::info!(new_name, account_id = ?id, "Successfully renamed account");
            Ok(())
        }
    }

    pub async fn update_account_balance(
        &self,
        id: Uuid,
        updated_balance: i64,
    ) -> Result<Account, UpdateAccountError> {
        let id_as_string = id.to_string();
        let row = sqlx::query!(
            "
UPDATE accounts
SET balance_cents = ?
WHERE id = ?
RETURNING *
",
            updated_balance,
            id_as_string
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UpdateAccountError::Unknown(e.into()))?;

        match row {
            None => Err(UpdateAccountError::NotFound { id }),
            Some(r) => {
                let uuid =
                    Uuid::from_str(&r.id).map_err(|e| UpdateAccountError::Unknown(e.into()))?;
                let name =
                    AccountName::new(&r.name).map_err(|e| UpdateAccountError::Unknown(e.into()))?;
                tracing::info!(
                    new_balance = r.balance_cents,
                    "Successfully updated the account's balance"
                );
                Ok(Account::new(uuid, name, r.balance_cents))
            }
        }
    }

    pub async fn delete_account(&self, id: Uuid) -> Result<(), DeleteAccountError> {
        let id_as_string = id.to_string();
        let result = sqlx::query!("DELETE FROM accounts WHERE id = ?", id_as_string)
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

    pub async fn create_transaction(
        &self,
        req: &CreateTransactionRequest,
    ) -> Result<Transaction, CreateTransactionError> {
        self.check_if_accounts_exist(req.source_account_id(), req.destination_account_id())
            .await?;

        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start SQLite transaction")?;

        let transaction_id = self.save_transaction(&mut tx, req).await.map_err(|e| {
            anyhow!(e).context(format!(
                "failed to save transaction with title {:?}",
                req.title()
            ))
        })?;
        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        tracing::info!(?transaction_id, "Successfully created transaction");

        Ok(Transaction::new(
            transaction_id,
            req.title().clone(),
            req.amount_cents(),
            req.source_account_id(),
            req.destination_account_id(),
            req.category().clone(),
            req.posting_date(),
        ))
    }

    pub async fn delete_transaction(&self, id: Uuid) -> Result<(), DeleteTransactionError> {
        let id_as_string = id.to_string();
        let result = sqlx::query!(
            "
         DELETE FROM \"postings\"
         WHERE id = ?
         ",
            id_as_string
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DeleteTransactionError::Unknown(e.into()))?;

        if result.rows_affected() == 0 {
            Err(DeleteTransactionError::TransactionNotFound { id })
        } else {
            tracing::info!(?id, "Successfully deleted transaction");
            Ok(())
        }
    }

    pub async fn get_transaction_by_id(
        &self,
        id: Uuid,
    ) -> Result<Transaction, GetTransactionError> {
        let id_as_string = id.to_string();
        let row = sqlx::query!("SELECT * FROM postings WHERE id = ?", id_as_string)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => GetTransactionError::TransactionNotFound { id },
                _ => GetTransactionError::Unknown(e.into()),
            })?;

        let transaction_title = TransactionTitle::new(&row.title)
            .map_err(|e| GetTransactionError::Unknown(e.into()))?;
        let id = Uuid::from_str(&row.id).map_err(|e| GetTransactionError::Unknown(e.into()))?;
        let source_account_id = Uuid::from_str(&row.source_account_id)
            .map_err(|e| GetTransactionError::Unknown(e.into()))?;
        let destination_account_id = Uuid::from_str(&row.destination_account_id)
            .map_err(|e| GetTransactionError::Unknown(e.into()))?;
        let posting_date = match DateTime::from_timestamp(row.posting_date, 0) {
            Some(p) => p,
            None => {
                return Err(GetTransactionError::Unknown(anyhow!(
                    "Invalid posting_date timestamp retrieved from the database: {}",
                    row.posting_date
                )))
            }
        };

        let transaction = Transaction::new(
            id,
            transaction_title,
            row.amount_cents,
            source_account_id,
            destination_account_id,
            row.category,
            posting_date,
        );
        tracing::info!(
            ?id,
            ?source_account_id,
            ?destination_account_id,
            "Successfully created transaction"
        );

        Ok(transaction)
    }

    pub async fn list_transactions(&self) -> Result<Vec<Transaction>, GetTransactionError> {
        let rows = sqlx::query!("SELECT * FROM \"postings\"")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| GetTransactionError::Unknown(e.into()))?;

        tracing::debug!(?rows, "Got rows from the database");

        rows.iter()
            .map(|r| {
                let transaction_title = TransactionTitle::new(&r.title)
                    .map_err(|e| GetTransactionError::Unknown(e.into()))?;
                let id =
                    Uuid::from_str(&r.id).map_err(|e| GetTransactionError::Unknown(e.into()))?;
                let source_account_id = Uuid::from_str(&r.source_account_id)
                    .map_err(|e| GetTransactionError::Unknown(e.into()))?;
                let destination_account_id = Uuid::from_str(&r.destination_account_id)
                    .map_err(|e| GetTransactionError::Unknown(e.into()))?;
                let posting_date = match DateTime::from_timestamp(r.posting_date, 0) {
                    Some(p) => p,
                    None => {
                        return Err(GetTransactionError::Unknown(anyhow!(
                            "Invalid posting_date timestamp retrieved from the database: {}",
                            r.posting_date
                        )))
                    }
                };

                let transaction = Transaction::new(
                    id,
                    transaction_title,
                    r.amount_cents,
                    source_account_id,
                    destination_account_id,
                    r.category.clone(),
                    posting_date,
                );
                tracing::info!(
                    ?id,
                    ?source_account_id,
                    ?destination_account_id,
                    "Successfully retrieved transaction"
                );

                Ok(transaction)
            })
            .collect()
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraing_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            return code == UNIQUE_CONSTRAINT_VIOLATION_CODE;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        models::{
            account::AccountName,
            transaction::{CreateTransactionRequest, TransactionTitle},
        },
        sqlite::Sqlite,
    };

    async fn setup_db() -> Sqlite {
        tracing_subscriber::fmt()
            .with_test_writer() // Ensures logs go to stdout in tests
            .with_env_filter("debug") // Adjust the log level as needed
            .init();

        let db = Sqlite::new(":memory:").await.unwrap();
        sqlx::migrate!().run(&db.pool).await.unwrap();

        db
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

        assert!(db.create_transaction(&tx_req).await.is_ok());
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
            .rename_account(account.id(), "New account name")
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_rename_unexisting_account() {
        let db = setup_db().await;

        assert!(db
            .rename_account(Uuid::new_v4(), "Dummy name")
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

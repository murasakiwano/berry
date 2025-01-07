use std::sync::LazyLock;

use berry::{
    configuration::{get_configuration, DatabaseSettings},
    models::account::AccountName,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};
use reqwest::header::CONTENT_TYPE;
use secrecy::SecretString;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

// NOTE: this code is from the book "Zero to Production In Rust".
// Check it out at https://www.zero2prod.com/

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "test".to_string();
    let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub api_client: reqwest::Client,
    pub test_account: TestAccount,
}

impl TestApp {
    pub async fn post_account(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(format!("{}/accounts", &self.address))
            .body(body)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn list_accounts(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/accounts", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_account(&self, id: String) -> reqwest::Response {
        self.api_client
            .get(format!("{}/accounts/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_account(&self, id: String) -> reqwest::Response {
        self.api_client
            .delete(format!("{}/accounts/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn rename_account(&self, id: String, body: String) -> reqwest::Response {
        self.api_client
            .patch(format!("{}/accounts/{}/name", &self.address, id))
            .body(body)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_transaction(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(format!("{}/transactions", &self.address))
            .body(body)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn list_transactions(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/transactions", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_transaction(&self, id: String) -> reqwest::Response {
        self.api_client
            .get(format!("{}/transactions/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_transaction(&self, id: String) -> reqwest::Response {
        self.api_client
            .delete(format!("{}/transactions/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub fn test_account(&self) -> &TestAccount {
        &self.test_account
    }
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c
    };

    // Create and migrate the database
    let pool = configure_database(&configuration.database).await;

    // Launch the application as a background task
    let application = Server::new(configuration.clone())
        .await
        .expect("Failed to initialize app.");
    let application_port = application.port();
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(application.run());

    let client = reqwest::Client::new();

    let test_app = TestApp {
        address: format!("http://localhost:{}/api", application_port),
        api_client: client,
        test_account: TestAccount::generate(),
    };

    test_app.test_account.store(&pool).await;

    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let maintenance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: SecretString::from("password"),
        ..config.clone()
    };
    let mut connection = PgConnection::connect_with(&maintenance_settings.connect_options())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.connect_options())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");
    connection_pool
}

pub struct TestAccount {
    pub id: Uuid,
    pub name: AccountName,
}

impl TestAccount {
    pub fn generate() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: AccountName::new(&Uuid::new_v4().to_string()).unwrap(),
        }
    }

    pub async fn store(&self, pool: &PgPool) {
        sqlx::query!(
            r#"INSERT INTO accounts (id, name) VALUES ($1, $2)"#,
            self.id,
            &self.name.to_string()
        )
        .execute(pool)
        .await
        .expect("Failed to store test account");
    }
}

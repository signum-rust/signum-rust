use once_cell::sync::Lazy;
use signum_node_rs::{
    configuration::{get_configuration, DatabaseSettings},
    srs_api::SrsApiApplication,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::SqlitePool;

// Ensure that `tracing` stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // Randomize config to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("failed to read configuration");
        c.database.filename = "sqlite::memory:".to_string();
        c.srs_api.listen_port = 0;
        c
    };

    // Configure and migrate the database
    let db_pool = configure_database(&configuration.database)
        .await
        .expect("unable to get dbpool");

    // Launch the application as a background task
    let application = SrsApiApplication::build(configuration.clone())
        .await
        .expect("failed to build application");
    let application_port = application.port();
    // #[allow("clippy::let_underscore_future")]
    tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        db_pool,
        api_client: client,
    }
}

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: SqlitePool,
    pub api_client: reqwest::Client,
}

impl TestApp {}

async fn configure_database(configuration: &DatabaseSettings) -> Result<SqlitePool, anyhow::Error> {
    // Create in-memory database and migrate it
    let connection_pool = SqlitePool::connect_with(configuration.get_writable_db()?)
        .await
        .expect("failed to connect to in-memory database");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate the in-memory database");

    Ok(connection_pool)
}

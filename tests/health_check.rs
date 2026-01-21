use std::net::TcpListener;

use news_letter_backend::configuration::{self, DatabaseSettings, get_configuration};
use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let address = app.address;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscription_return_200_for_valid_form_data() {
    let app = spawn_app().await;
    let address = app.address;
    let db = app.db_pool;

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

   


    let response = client
        .post(&format!("{}/subscription", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name from subscriptions",)
        .fetch_one(&db)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

    

}

#[tokio::test]
async fn subscription_return_400_for_when_data_is_missing() {
    let app = spawn_app().await;
    let address = app.address;

    let client = reqwest::Client::new();

    let missing_data = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_msg) in missing_data {
        let response = client
            .post(&format!("{}/subscription", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        );
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to localhost");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("failed to get configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&configuration.database).await;

    let server = news_letter_backend::startup::run(listener, db_pool.clone())
        .expect("failed to bind to localhost");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool{
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let pg_pool = PgPool::connect(&config.connection_string()).await.expect("failed to connect to postgres");

    sqlx::migrate!("./migrations").run(&pg_pool).await.expect("failed to migrate");

    pg_pool
}


async fn drop_db(db: Pool<Postgres>) {
    let get_db = sqlx::query!("SELECT current_database()",).fetch_one(&db).await.expect("failed to fetch current database").current_database.unwrap();
    let drop_query = format!("DROP DATABASE {}", get_db);
    sqlx::query(&drop_query)
        .execute(&db)
        .await
        .expect("failed to drop database");
}

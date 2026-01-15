use std::net::TcpListener;

use news_letter_backend::configuration::get_configuration;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();


    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("failed to execute request");
    println!("{:?}", response);
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscription_return_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

        let configuration = get_configuration().expect("failed to get configuration");

    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("failed to connect to postgres");

    let saved = sqlx::query!("SELECT email, name from subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    let response = client
        .post(&format!("{}/subscription", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscription_return_400_for_when_data_is_missing() {
    let address = spawn_app();

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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to localhost");

    let port = listener.local_addr().unwrap().port();

    let server = news_letter_backend::startup::run(listener).expect("failed to bind to localhost");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

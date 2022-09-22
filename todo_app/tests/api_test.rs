mod common;

use crate::common::app::run_app;
use crate::common::postgres_container::PostgresDatabase;
use reqwest;
use std::collections::HashMap;
use testcontainers::clients::Cli;

#[tokio::test]
async fn test_create_todo() {
    let cli = Cli::docker();
    let db = PostgresDatabase::new(&cli);
    let app = run_app(db.get_url().as_str()).await;
    let client = reqwest::Client::new();
    let mut request_body = HashMap::new();
    request_body.insert("title", "This is my todo");
    let response = client
        .post(format!("http://localhost:{}/todos", app.port))
        .json(&request_body)
        .send()
        .await
        .expect("Cannot create todo");
    assert!(response.status().is_success());
}

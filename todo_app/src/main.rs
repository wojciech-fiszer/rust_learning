mod domain;
mod infrastructure;
use crate::infrastructure::api::run_api;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;
use tokio;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind random port");
    let db_url = "postgres://todo_app:todo_app@127.0.0.1:5432/todo_app";
    let db_pool = PgPool::connect(db_url)
        .await
        .expect("Cannot connect to database");
    println!(
        "Starting application on port: {}",
        listener.local_addr().unwrap().port()
    );
    run_api(listener, db_pool)?.await
}

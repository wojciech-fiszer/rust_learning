use sqlx::PgPool;
use std::net::TcpListener;
use todo_app::infrastructure::api::run_api;

pub struct TestApp {
    pub port: u16,
}

pub async fn run_app(db_url: &str) -> TestApp {
    let db_pool = PgPool::connect(db_url)
        .await
        .expect("Cannot connect to database");
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Cannot migrate database");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run_api(listener, db_pool).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { port }
}

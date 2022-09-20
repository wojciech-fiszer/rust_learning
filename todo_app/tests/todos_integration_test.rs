use reqwest;
use sqlx::{PgPool, Pool, Postgres};
use std::collections::HashMap;
use std::net::TcpListener;
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::Image;
use todo_app::infrastructure::api::run_api;

#[tokio::test]
async fn test_create_todo() {
    let cli = Cli::default();
    let postgres_container = cli.run(PostgresImage::new(
        "todo_app".to_string(),
        "todo_app".to_string(),
        "todo_app".to_string(),
    ));
    let db_port = postgres_container
        .ports()
        .map_to_host_port_ipv4(5432)
        .expect("Postgres container has not exposed 5432 port");

    let db_pool = PgPool::connect(
        format!(
            "postgres://todo_app:todo_app@localhost:{}/todo_app",
            db_port
        )
        .as_str(),
    )
    .await
    .expect("Cannot connect to database");

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Cannot migrate database");

    let app = run_app(db_pool).await;
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

struct TestApp {
    pub port: u16,
}

async fn run_app(db_pool: Pool<Postgres>) -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run_api(listener, db_pool).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { port }
}

struct PostgresImage {
    env_vars: HashMap<String, String>,
}

impl PostgresImage {
    fn new(db: String, user: String, password: String) -> PostgresImage {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_string(), db);
        env_vars.insert("POSTGRES_USER".to_string(), user);
        env_vars.insert("POSTGRES_PASSWORD".to_string(), password);
        PostgresImage { env_vars }
    }
}

impl Image for PostgresImage {
    type Args = ();

    fn name(&self) -> String {
        "postgres".to_owned()
    }

    fn tag(&self) -> String {
        "14".to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}

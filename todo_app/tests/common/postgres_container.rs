use std::collections::HashMap;
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::{Container, Image};

struct PostgresImage {
    env_vars: HashMap<String, String>,
}

impl PostgresImage {
    fn new(db: &str, user: &str, password: &str) -> PostgresImage {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_string(), db.to_string());
        env_vars.insert("POSTGRES_USER".to_string(), user.to_string());
        env_vars.insert("POSTGRES_PASSWORD".to_string(), password.to_string());
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

pub struct PostgresDatabase<'a> {
    container: Container<'a, PostgresImage>,
    user: String,
    password: String,
    host: String,
    db: String,
}

impl<'a> PostgresDatabase<'a> {
    pub fn new(cli: &'a Cli) -> PostgresDatabase<'a> {
        let user = "todo_app";
        let password = "todo_app";
        let db = "todo_app";
        PostgresDatabase {
            container: cli.run(PostgresImage::new(db, user, password)),
            user: user.to_string(),
            password: password.to_string(),
            host: "localhost".to_string(),
            db: db.to_string(),
        }
    }

    pub fn get_url(&self) -> String {
        let port = self
            .container
            .ports()
            .map_to_host_port_ipv4(5432)
            .expect("Postgres container has not exposed 5432 port");
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, port, self.db
        )
    }
}

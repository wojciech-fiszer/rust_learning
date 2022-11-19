use crate::domain::todo::{Todo, TodoRepository};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::{query_as, Error, FromRow, Pool, Postgres, Row};

pub struct PostgresTodoRepository<'a> {
    pool: &'a Pool<Postgres>,
}

impl FromRow<'_, PgRow> for Todo {
    fn from_row(row: &PgRow) -> std::result::Result<Self, Error> {
        let id = row.try_get::<Uuid, &str>("id")?.to_string();
        let title = row.try_get("title")?;
        let done = row.try_get("done")?;
        Ok(Todo::new(id, title, done))
    }
}

#[async_trait]
impl<'a> TodoRepository for PostgresTodoRepository<'a> {
    async fn insert(&self, title: &str, done: bool) -> Result<Todo> {
        match query_as("INSERT INTO todos (title, done) VALUES ($1, $2) RETURNING id, title, done")
            .bind(title)
            .bind(done)
            .fetch_one(self.pool)
            .await
        {
            Ok(todo) => Ok(todo),
            Err(err) => Err(anyhow!(err)),
        }
    }

    async fn find_all(&self) -> Result<Vec<Todo>> {
        match query_as("SELECT id, title, done FROM todos")
            .fetch_all(self.pool)
            .await
        {
            Ok(todos) => Ok(todos),
            Err(err) => Err(anyhow!(err)),
        }
    }
}

impl<'a> PostgresTodoRepository<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

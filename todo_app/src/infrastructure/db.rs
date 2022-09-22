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
        Ok(Todo {
            id: Some(row.try_get::<Uuid, &str>("id")?.to_string()),
            title: row.try_get("title")?,
            done: row.try_get("done")?,
        })
    }
}

#[async_trait]
impl<'a> TodoRepository for PostgresTodoRepository<'a> {
    async fn save(&self, todo: Todo) -> Result<Todo> {
        let title = todo.title;
        let done = todo.done;
        match todo.id {
            Some(id) => self.update(id, title, done).await,
            None => self.create(title, done).await,
        }
    }

    async fn get_all(&self) -> Result<Vec<Todo>> {
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
    async fn update(&self, id: String, title: String, done: bool) -> Result<Todo> {
        match query_as(
            "UPDATE todos SET (title, done) = ($1, $2) WHERE id::text = $3 RETURNING id, title, done",
        )
        .bind(title)
        .bind(done)
        .bind(id)
        .fetch_one(self.pool)
        .await
        {
            Ok(todo) => Ok(todo),
            Err(err) => Err(anyhow!(err)),
        }
    }

    async fn create(&self, title: String, done: bool) -> Result<Todo> {
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
}

impl<'a> PostgresTodoRepository<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

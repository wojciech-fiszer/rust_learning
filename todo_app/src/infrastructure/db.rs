use crate::domain::todo::{Todo, TodoRepository};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct PostgresTodoRepository<'a> {
    pool: &'a Pool<Postgres>,
}

#[async_trait]
impl<'a> TodoRepository for PostgresTodoRepository<'a> {
    async fn save(&self, todo: Todo) -> Result<Todo> {
        match todo.id {
            Some(id) => match sqlx::query_as!(
            Todo,
            "UPDATE todos SET (title, done) = ($1, $2) WHERE id::text = $3 RETURNING id::text, title, done",
            todo.title,
            todo.done,id
        ).fetch_one(self.pool).await {
                Ok(todo) => Ok(todo),
                Err(err) => Err(anyhow!(err))
            },
            None => match sqlx::query_as!(
            Todo,
            "INSERT INTO todos (title, done) VALUES ($1, $2) RETURNING id::text, title, done",
            todo.title,
            todo.done,
        ).fetch_one(self.pool).await {
                Ok(todo) => Ok(todo),
                Err(err) => Err(anyhow!(err))
            }
        }
    }

    async fn get_all(&self) -> Result<Vec<Todo>> {
        todo!()
    }
}

impl<'a> PostgresTodoRepository<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

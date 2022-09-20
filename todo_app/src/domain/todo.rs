use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository {
    async fn save(&self, todo: Todo) -> Result<Todo>;
    async fn get_all(&self) -> Result<Vec<Todo>>;
}

pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn id(&self) -> &Option<String> {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn done(&self) -> bool {
        self.done
    }
}

pub async fn create_todo<T>(repository: &T, title: &str) -> Result<Todo>
where
    T: TodoRepository,
{
    repository
        .save(Todo {
            id: None,
            title: title.to_string(),
            done: false,
        })
        .await
}

pub async fn get_all_todos<T>(repository: &T) -> Result<Vec<Todo>>
where
    T: TodoRepository,
{
    repository.get_all().await
}

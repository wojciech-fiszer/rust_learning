use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository {
    async fn insert(&self, title: &str, done: bool) -> Result<Todo>;
    async fn find_all(&self) -> Result<Vec<Todo>>;
}

pub struct Todo {
    id: String,
    title: String,
    done: bool,
}

impl Todo {
    pub fn new(id: String, title: String, done: bool) -> Self {
        Self { id, title, done }
    }
}

impl Todo {
    pub fn id(&self) -> &str {
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
    repository.insert(title, false).await
}

pub async fn get_all_todos<T>(repository: &T) -> Result<Vec<Todo>>
where
    T: TodoRepository,
{
    repository.find_all().await
}

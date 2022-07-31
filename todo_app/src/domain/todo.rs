use repository::TodoRepository;

use crate::domain::todo::error::{TodoCreateError, TodoGetAllError};

pub mod repository {
    use crate::domain::todo::repository::error::{
        TodoRepositoryGetAllError, TodoRepositorySaveError,
    };
    use crate::domain::todo::Todo;

    pub trait TodoRepository {
        fn save(&self, todo: Todo) -> Result<Todo, TodoRepositorySaveError>;
        fn get_all(&self) -> Result<Vec<Todo>, TodoRepositoryGetAllError>;
    }

    pub mod error {
        use std::error::Error;
        use std::fmt::{Display, Formatter};

        #[derive(Debug)]
        pub enum TodoRepositorySaveError {
            GeneticError(Box<dyn Error>),
        }

        impl Display for TodoRepositorySaveError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    TodoRepositorySaveError::GeneticError(_) => {
                        write!(f, "Unexpected error while saving the todo")
                    }
                }
            }
        }

        impl Error for TodoRepositorySaveError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match self {
                    TodoRepositorySaveError::GeneticError(e) => Some(e.as_ref()),
                }
            }
        }

        #[derive(Debug)]
        pub enum TodoRepositoryGetAllError {
            GeneticError(Box<dyn Error>),
        }

        impl Display for TodoRepositoryGetAllError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    TodoRepositoryGetAllError::GeneticError(_) => {
                        write!(f, "Unexpected error while getting all todos")
                    }
                }
            }
        }

        impl Error for TodoRepositoryGetAllError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match self {
                    TodoRepositoryGetAllError::GeneticError(e) => Some(e.as_ref()),
                }
            }
        }
    }
}

mod error {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum TodoCreateError {
        GeneticError(Box<dyn Error>),
    }

    impl Display for TodoCreateError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TodoCreateError::GeneticError(_) => {
                    write!(f, "Unexpected error while creating the todo")
                }
            }
        }
    }

    impl Error for TodoCreateError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                TodoCreateError::GeneticError(e) => Some(e.as_ref()),
            }
        }
    }

    #[derive(Debug)]
    pub enum TodoGetAllError {
        GeneticError(Box<dyn Error>),
    }

    impl Display for TodoGetAllError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TodoGetAllError::GeneticError(_) => {
                    write!(f, "Unexpected error while creating the todo")
                }
            }
        }
    }

    impl Error for TodoGetAllError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                TodoGetAllError::GeneticError(e) => Some(e.as_ref()),
            }
        }
    }
}

pub struct Todo {
    id: Option<u32>,
    title: String,
    done: bool,
}

fn create_todo<R>(repository: &R, title: &str) -> Result<Todo, TodoCreateError>
where
    R: TodoRepository,
{
    repository
        .save(Todo {
            id: None,
            title: title.to_string(),
            done: false,
        })
        .map_err(|e| TodoCreateError::GeneticError(Box::new(e)))
}

fn get_all_todos<R>(repository: &R) -> Result<Vec<Todo>, TodoGetAllError>
where
    R: TodoRepository,
{
    repository
        .get_all()
        .map_err(|e| TodoGetAllError::GeneticError(Box::new(e)))
}

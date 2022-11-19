use crate::domain::todo::{create_todo, get_all_todos, Todo};
use crate::infrastructure::db::PostgresTodoRepository;
use actix_web::dev::Server;
use actix_web::web::{get, post, Data, Json};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

pub fn run_api(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/todos", post().to(handle_post_todos))
            .route("/todos", get().to(handle_get_todos))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
}

#[derive(Serialize)]
struct TodoDto {
    id: String,
    title: String,
    done: bool,
}

impl TryFrom<&Todo> for TodoDto {
    type Error = String;

    fn try_from(value: &Todo) -> Result<Self, Self::Error> {
        Ok(TodoDto {
            id: value.id().to_string(),
            title: value.title().to_string(),
            done: value.done(),
        })
    }
}

async fn handle_post_todos(
    pg_pool: Data<PgPool>,
    request: Json<CreateTodoRequest>,
) -> impl Responder {
    let repository = PostgresTodoRepository::new(pg_pool.get_ref());
    match create_todo(&repository, &request.title).await {
        Ok(todo) => match TodoDto::try_from(&todo) {
            Ok(dto) => HttpResponse::Created().json(dto),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn handle_get_todos(pg_pool: Data<PgPool>) -> impl Responder {
    let repository = PostgresTodoRepository::new(pg_pool.get_ref());
    match get_all_todos(&repository).await {
        Ok(todos) => match todos
            .iter()
            .map(move |todo| TodoDto::try_from(todo))
            .collect::<Result<Vec<TodoDto>, _>>()
        {
            Ok(dtos) => HttpResponse::Ok().json(dtos),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

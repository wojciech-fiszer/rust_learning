use std::{io, sync::Mutex, vec::Vec};

use actix_web::{
    App, get,
    HttpResponse,
    HttpServer, post, Responder, web::{Data, Json},
};

#[post("/todos")]
async fn create_todo(todos: Data<Mutex<Vec<String>>>, req_body: String) -> impl Responder {
    todos.lock().unwrap().push(req_body);
    HttpResponse::Created()
}

#[get("/todos")]
async fn get_todos(todos: Data<Mutex<Vec<String>>>) -> impl Responder {
    Json(todos.lock().unwrap().clone())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let todos: Data<Mutex<Vec<String>>> = Data::new(Mutex::new(Vec::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(todos.clone())
            .service(create_todo)
            .service(get_todos)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

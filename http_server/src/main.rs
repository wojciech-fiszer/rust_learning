use crate::website_handler::WebsiteHandler;
use server::HttpServer;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = HttpServer::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

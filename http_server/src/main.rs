use crate::website_handler::WebsiteHandler;
use server::HttpServer;

mod http;
mod server;
mod website_handler;

fn main() {
    let server = HttpServer::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}

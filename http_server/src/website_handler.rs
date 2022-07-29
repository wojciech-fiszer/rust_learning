use crate::http::status_code::StatusCode;
use crate::http::{Request, Response};
use crate::server::RequestHandler;

pub struct WebsiteHandler;

impl RequestHandler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(
            StatusCode::Ok,
            Some("<h1>Hello from my HTTP server</h2>".to_string()),
        )
    }
}

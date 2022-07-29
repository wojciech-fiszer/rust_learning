use crate::http::status_code::StatusCode;
use crate::http::{Request, Response};
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

pub struct HttpServer {
    addr: String,
}
impl HttpServer {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>Hello from my HTTP server</h2>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Failed to read the stream: {}", e);
                        }
                    };
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            };
        }
    }
}

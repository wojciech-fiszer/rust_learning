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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response = Response::new(
                                        StatusCode::NotFound,
                                        Some("<h1>Hello from my HTTP server</h2>".to_string()),
                                    );
                                    match write!(stream, "{}", response) {
                                        Ok(_) => println!("Responded successfully"),
                                        Err(e) => {
                                            println!("Could not write the response: {}", e);
                                            continue;
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Could not read request: {}", e);
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to read the stream: {}", e);
                            continue;
                        }
                    };
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                    continue;
                }
            };
        }
    }
}

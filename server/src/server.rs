use std::io::{Write, Read};
use std::net::TcpListener;
use crate::http::{Response, Request, StatusCode, response, ParseError};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(host_port: &str) -> Self {
        Self {
            addr: host_port.to_string(),
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read: {}", e)
                    }
                },
                Err(e) => println!("failed to establish the connection: {}", e),
            };
        }
    }
}
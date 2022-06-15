use std::fs;

use super::server::Handler;
use crate::http::{HTTPMethod, Request, Response, StatusCode};
pub struct WebsiteHandler {
    pubic_path: String,
}

impl WebsiteHandler {
    pub fn new(pub_path: &str) -> Self {
        Self {
            pubic_path: pub_path.to_string(),
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.pubic_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.pubic_path) {
                    return fs::read_to_string(path).ok()
                } else {
                    // someone can put ../../ in file path using netcat to read other files
                    // echo "/../Cargo.toml" | nc 127.0.0.1 8080 will print out Cargo.toml
                    println!("Directory traversal attack attempted {}", file_path);
                    None
                }
            },
            Err(e) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            HTTPMethod::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)), // for reading style.css file request
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

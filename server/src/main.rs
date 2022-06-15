#![allow(dead_code, unused_imports, unused_variables)]
mod server;
mod http;
mod website_handler;

use server::Server;
use http::{HTTPMethod, Request};
use website_handler::WebsiteHandler;
use std::env;

fn main() {
    let default_path = env!("CARGO_MANIFEST_DIR"); // directory of cargo.toml file
    let default_path = format!("{}/public", default_path);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080");
    server.run(WebsiteHandler::new(&public_path));
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADER \r\n
BODY
 */


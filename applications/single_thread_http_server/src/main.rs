#![allow(dead_code)]

mod http;
mod server;
mod website_handler;

// use http::Method;
// use http::Request;

use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_owned());
    server.run(WebsiteHandler);
}

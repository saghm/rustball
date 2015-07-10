#[macro_use]
extern crate bson;
extern crate mongodb;
extern crate rustc_serialize;

#[macro_use]
extern crate rustful;

mod handler;
mod server;

use std::net::Ipv4Addr;

use handler::RequestHandler;
use mongodb::{Client, ThreadedClient};
use rustful::{Server, TreeRouter};

fn main() {
    let client = Client::connect("localhost", 27017).unwrap();
    let find_handler = RequestHandler::new(client.clone(), server::handle_find);

    let server = Server {
        host: (Ipv4Addr::new(127, 0, 0, 1), 3000).into(),
        content_type: content_type!(Application / Json; Charset = Utf8),
        handlers: insert_routes! {
            TreeRouter::new() => {
                "/find/" => Get: find_handler
            }
        },

        ..Server::default()
    };

    match server.run() {
        Ok(_) => {},
        Err(e) => panic!("Couldn't start server: {}", e)
    }
}

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

    macro_rules! route {
        ($func:ident) => {
            RequestHandler::new(client.clone(), server::$func);
        };
    }

    let server = Server {
        host: (Ipv4Addr::new(127, 0, 0, 1), 3000).into(),
        content_type: content_type!(Application / Json; Charset = Utf8),
        fallback_handler: None,
        handlers: insert_routes! {
            TreeRouter::new() => {
                "/averages/high" => Get: route!(highest_averages),
                "/averages/low" => Get: route!(lowest_averages),
                "/teams" => Get: route!(teams),
                "/teams/batters" => Get: route!(team_batters),
                "/tags/player/:id" => Get: route!(player_tags),
                "/tags/player/:id/add" => Post: route!(add_tag),
                "/teams/:team" => Get: route!(team_roster)
            }
        },

        ..Server::default()
    };

    match server.run() {
        Ok(_) => {},
        Err(e) => panic!("Couldn't start server: {}", e)
    }
}

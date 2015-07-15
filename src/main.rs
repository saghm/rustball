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

    macro_rules! request_route {
        ($func:ident) => {
            RequestHandler::new(client.clone(), server::rest_api::$func);
        };
    }

    let server = Server {
        host: (Ipv4Addr::new(127, 0, 0, 1), 3000).into(),
        content_type: content_type!(Application / Json; Charset = Utf8),
        fallback_handler: None,
        handlers: insert_routes! {
            TreeRouter::new() => {
                "/rest_api/averages/high" => Get: request_route!(highest_averages),
                "/rest_api/averages/low" => Get: request_route!(lowest_averages),
                "/rest_api/players/tag/:tag" => Get: request_route!(tagged_players),
                "/rest_api/teams" => Get: request_route!(teams),
                "/rest_api/teams/batters" => Get: request_route!(team_batters),
                "/rest_api/tags/player/:id" => Get: request_route!(player_tags),
                "/rest_api/tags/player/:id/add" => Post: request_route!(add_tag),
                "/rest_api/teams/:team" => Get: request_route!(team_roster),
            }
        },

        ..Server::default()
    };

    match server.run() {
        Ok(_) => {},
        Err(e) => panic!("Couldn't start server: {}", e)
    }
}

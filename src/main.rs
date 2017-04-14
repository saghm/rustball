#[macro_use]
extern crate bson;
extern crate mongodb;
extern crate serde_json;

#[macro_use]
extern crate rustful;

mod handler;

#[macro_use]
mod macros;
mod server;

use std::net::Ipv4Addr;

use handler::AppHandler;
use mongodb::{Client, ThreadedClient};
use rustful::{Server, TreeRouter};

fn main() {
    let client = Client::connect("localhost", 27017).unwrap();

    macro_rules! page_route {
        ($file:ident) => {
            AppHandler::Page(format!("views/{}.html", stringify!($file)))
        };
    }

    macro_rules! request_route {
        ($func:ident) => {
            AppHandler::Request { client: client.clone(), handler: server::rest_api::$func }
        };
    }

    let server = Server {
        host: (Ipv4Addr::new(127, 0, 0, 1), 3000).into(),
        content_type: content_type!(Application / Json; Charset = Utf8),
        fallback_handler: Some(AppHandler::NotFound),
        handlers: insert_routes! {
            TreeRouter::new() => {
                // Pages
                "/averages/:rank" => Get: page_route!(averages),
                "/batters" => Get: page_route!(batters),
                "/player/:id" => Get: page_route!(player),
                "/league" => Get: page_route!(league),
                "/tags" => Get: page_route!(tags),
                "/tags/:tag" => Get: page_route!(tags),
                "/team/:team" => Get: page_route!(team),

                // REST API
                "/rest_api/averages/high" => Get: request_route!(highest_averages),
                "/rest_api/averages/low" => Get: request_route!(lowest_averages),
                "/rest_api/batters" => Get: request_route!(team_batters),
                "/rest_api/league" => Get: request_route!(teams),
                "/rest_api/player/:id" => Get: request_route!(player_tags),
                "/rest_api/player/:id/add_tag" => Post: request_route!(add_tag),
                "/rest_api/tags/:tag" => Get: request_route!(tagged_players),
                "/rest_api/team/:team" => Get: request_route!(team_roster),
                "/rest_api/team/:team/name" => Get: request_route!(team_name),

                // Static
                "/static/css/:file" => Get: AppHandler::Css
            }
        },

        ..Server::default()
    };

    match server.run() {
        Ok(_) => {},
        Err(e) => panic!("Couldn't start server: {}", e)
    }
}

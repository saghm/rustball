use std::sync::Arc;

use hyper::Client as HttpClient;
use mongodb::Client;
use rustful::{Context, Response, Handler};
use server::page;

pub enum AppHandler {
    Default(fn(Context, Response)),
    NotFound,
    Page(String),
    Request {
        client: Client,
        handler: fn(Client, Context, Response),
    },
    SignIn {
        mongo: Client,
        http: Arc<HttpClient>,
        handler: fn(Client, Arc<HttpClient>, Context, Response),
    }
}

impl Handler for AppHandler {
    fn handle_request(&self, context: Context, response: Response) {
        match self {
            &AppHandler::Default(func) => func(context, response),
            &AppHandler::NotFound => page::not_found(response),
            &AppHandler::Page(ref file) => page::render_html(file, response),
            &AppHandler::Request { ref client, handler } => handler(client.clone(), context, response),
            &AppHandler::SignIn { ref mongo, ref http, handler } =>
                handler(mongo.clone(), http.clone(), context, response)
        }
    }
}

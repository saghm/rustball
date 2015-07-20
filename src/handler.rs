use mongodb::Client;
use rustful::{Context, Response, Handler};
use server::page;

pub enum AppHandler {
    Css,
    NotFound,
    Page(String),
    Request {
        client: Client,
        handler: fn(Client, Context, Response),
    },
}

impl Handler for AppHandler {
    fn handle_request(&self, context: Context, response: Response) {
        match self {
            &AppHandler::Css => page::render_css(context, response),
            &AppHandler::NotFound => page::not_found(response),
            &AppHandler::Page(ref file) => page::render_html(file, response),
            &AppHandler::Request { ref client, handler } => handler(client.clone(), context, response)
        }
    }
}

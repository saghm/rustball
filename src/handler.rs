use mongodb::Client;
use rustful::{Context, Response, Handler};

pub enum AppHandler {
    Page(fn(Context, Response)),
    Request {
        client: Client,
        handler: fn(Client, Context, Response),
    },
}

impl Handler for AppHandler {
    fn handle_request(&self, context: Context, response: Response) {
        match self {
            &AppHandler::Page(handler) => handler(context, response),
            &AppHandler::Request { ref client, handler } => handler(client.clone(), context, response)
        }

    }
}

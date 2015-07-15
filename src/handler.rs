use mongodb::Client;
use rustful::{Context, Response, Handler};

pub struct PageHandler(fn(Context, Response));

pub struct RequestHandler {
    client: Client,
    handler: fn(Client, Context, Response),
}

impl RequestHandler {
    pub fn new(client: Client,
               handler: fn(Client, Context, Response)) -> RequestHandler {
        RequestHandler { client: client, handler: handler }
    }
}


impl Handler for PageHandler {
    fn handle_request(&self, context: Context, response: Response) {
        self.0(context, response);
    }
}

impl Handler for RequestHandler {
    fn handle_request(&self, context: Context, response: Response) {
        let handler = self.handler;
        handler(self.client.clone(), context, response);
    }
}

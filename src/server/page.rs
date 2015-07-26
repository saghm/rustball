use rustful::{Context, Response, StatusCode};
use rustful::header::ContentType;

use std::fs::File;
use std::io::Read;

macro_rules! render_file {
    ($page:expr, $response:expr) => {{
        let mut file = File::open($page).unwrap();
        let mut html = String::new();

        file.read_to_string(&mut html).unwrap();
        $response.send(format!("{}", html))
    }};
}

pub fn render_html(file: &str, mut response: Response) {
    set_content_type!(response, Text / Html);
    render_file!(file, response)
}

pub fn render_css(context: Context, mut response: Response) {
    match context.variables.get("file") {
        Some(file) => {
            set_content_type!(response, Text / Css);
            render_file!(format!("static/{}", file), response)
        },
        None => {
            set_content_type!(response, Text / Plain);
            response.send("No file specified")
        }
    }
}

pub fn not_found(mut response: Response) {
    response.set_status(StatusCode::NotFound);
    set_content_type!(response, Text / Html);
    response.send("<img src=\"http://cf.chucklesnetwork.com/items/3/8/5/8/3/original/i-dont-always-crash-error-404-but-when-i-do-not-found.jpg\">")
}

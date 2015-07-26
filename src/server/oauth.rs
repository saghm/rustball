use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

use credentials::{CONSUMER_KEY, CONSUMER_SECRET};
use hyper::Client as HttpClient;
use hyper::header::Authorization;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use oauthcli::{self, SignatureMethod};
use rustful::{Context, Response, StatusCode};
use rustful::header::{Cookie, Location};
use url::Url;

macro_rules! check_for_session {
    ($cookies:expr, $response:expr) => {
        if $cookies.iter().any(|c| c.name.eq("userid")) {
                $response.set_status(StatusCode::Found);
                $response.headers_mut().set(Location("http://localhost:3000/league/".to_owned()));
                return $response.send("");
        }
    };
}

pub fn sign_in(mongo: Client, http: Arc<HttpClient>, mut context: Context,
               mut response: Response) {
    match context.headers.get_mut() {
        Some(&mut Cookie(ref mut cookies)) => check_for_session!(cookies, response),
        None => ()
    };

    let url = "https://twitter.com/oauth/request_token";
    let auth = oauthcli::authorization_header(
        "POST",
        Url::parse(url).unwrap(),
        None,
        CONSUMER_KEY,
        CONSUMER_SECRET,
        None,
        None,
        SignatureMethod::HmacSha1,
        &oauthcli::timestamp()[..],
        &oauthcli::nonce()[..],
        Some("http://127.0.0.1:3000/persist_access/"),
        None,
        vec![].into_iter()
    );

    let mut token_response = match http.post(url).header(Authorization(auth)).send() {
        Ok(response) => response,
        Err(e) => return response.send(format!("Error sending initial oauth request: {}", e))
    };

    let mut string = String::new();
    match token_response.read_to_string(&mut string) {
        Ok(_) => (),
        Err(e) =>
        return response.send(format!("Error reading response to initial oauth request: {}", e))
    };

    let body : Vec<_> = string.split("&").map(|s| s.split("=")).collect();
    let mut map = HashMap::new();

    for mut field in body {
        let key = match field.next() {
            Some(k) => k,
            None => continue
        };

        let value = match field.next() {
            Some(k) => k,
            None => continue
        };

        map.insert(key, value);
    }

    let token = match map.get("oauth_token") {
        Some(string) => string,
        None => return response.send("No `oauth_token` in response from Twitter")
    };

    let secret = match map.get("oauth_token_secret") {
        Some(string) => string,
        None => return response.send("No `oauth_token_secret` in response from Twitter")
    };

    let doc = doc! {
        &token[..] => (secret.to_owned())
    };

    let db = mongo.db("mlb");
    let coll = db.collection("oauth");

    match coll.insert_one(doc, None) {
        Ok(_) => (),
        Err(e) => return response.send(format!("Error saving response in database: {}", e))
    };


    let url = format!("https://twitter.com/oauth/authenticate?oauth_token={}", token);
    response.set_status(StatusCode::Found);
    response.headers_mut().set(Location(url));
    response.send("")
}

pub fn persist_access() {
}

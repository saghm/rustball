use std::error::Error;

use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::coll::options::FindOptions;
use mongodb::cursor::Cursor;
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;
use rustful::{Context, Response};

macro_rules! err_as_string {
    ($err:expr) => {
        Err($err.description().to_owned())
    };
}

macro_rules! get_string_or_err {
    ($doc:expr, $key:expr) => {
        match $doc.get($key) {
            Some(&Bson::String(ref s)) => s,
            Some(ref bson) => return Err(format!("Invalid value: {:?}", bson)),
            None => return Err(format!("Key not present: {}", $key))
        }
    };
}

macro_rules! err_as_json_string {
    ($err:expr) => {
        Err(format!("{{\"error\":\"{}\"}}", $err.description()))
    }
}

fn get_team(client: Client, team: &str) -> Result<Cursor, String> {
    let db = client.db("mlb");
    let coll = db.collection("players");

    let filter = Some(doc! { "team" => (team) });

    let mut options = FindOptions::new();
    options.projection = Some(doc! {
        "_id" => (0),
        "first_name" => (1),
        "last_name" => (1),
        "position" => (1)
    });

    match coll.find(filter, Some(options)) {
        Ok(cursor) => Ok(cursor),
        Err(e) => err_as_string!(e)
    }
}

fn json_string_from_doc_result(result: MongoResult<Document>) -> Result<String, String> {
    match result {
        Ok(doc) => Ok(format!("{}", Bson::Document(doc).to_json())),
        Err(e) => err_as_json_string!(e)
    }
}

fn get_json_string(client: Client, team: &str) -> String {
    match get_team(client, team) {
        Ok(cursor) => {
            let mut string = "[".to_owned();

            for (i, doc_result) in cursor.enumerate() {
                match json_string_from_doc_result(doc_result) {
                    Ok(json_string) => {
                        let new_string = if i == 0 {
                            json_string
                        } else {
                            format!(",{}", json_string)
                        };

                        string.push_str(&new_string);
                    },
                    Err(e) => return e
                }
            }

            string.push_str("]");
            string
        },
        Err(e) => e
    }
}

pub fn handle_team(client: Client, context: Context, response: Response) {
    let team = match context.variables.get("team") {
        Some(team_name) => &team_name[..],
        None => "BOS"
    };

    let string = get_json_string(client, team);

    response.into_writer().send(string);
}

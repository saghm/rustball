use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::cursor::Cursor;
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::FindOptions;
use mongodb::error::Result as MongoResult;
use rustful::{Context, Response};

macro_rules! json_error_string {
    ($string:expr) => {
        format!("{{\"error\":\"{}\"}}", $string)
    }
}

macro_rules! respond {
    ($response:expr, $string:expr) => {
        $response.into_writer().send($string)
    }
}

macro_rules! respond_with_json_err {
    ($response:expr, $err:expr) => {
        respond!($response, format!("{{\"error\":\"{}\"}}", $err))
    };
}

macro_rules! json_string_from_doc {
    ($doc:expr) => {
        format!("{}", Bson::Document($doc).to_json())
    };
}

fn json_string_from_doc_result(result: MongoResult<Document>) -> Result<String, String> {
    match result {
        Ok(doc) => Ok(json_string_from_doc!(doc)),
        Err(e) => Err(json_error_string!(e))
    }
}

fn get_json_string(result: MongoResult<Cursor>) -> String {
    match result {
        Ok(mut cursor) => {
            match cursor.has_next() {
                Ok(true) => (),
                Ok(false) => return json_error_string!("Invalid team"),
                Err(e) => return json_error_string!(e)
            }

            let mut string = "{\"result\":[".to_owned();

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

            string.push_str("]}");
            string
        },
        Err(e) => json_error_string!(e)
    }
}

fn get_team(client: Client, team: &str) -> String {
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

    let result = coll.find(filter, Some(options));
    get_json_string(result)
}

pub fn team(client: Client, context: Context, response: Response) {
    let team = match context.variables.get("team") {
        Some(team_name) => &team_name[..],
        None => return respond_with_json_err!(response, "No team specified")
    };

    let string = get_team(client, team);
    respond!(response, string);
}

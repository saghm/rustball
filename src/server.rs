use std::error::Error;

use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::cursor::Cursor;
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;
use rustc_serialize::json::Json;
use rustful::{Context, Response};

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
        format!("{{\"error\":\"{}\"}}", $err.description())
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
        Err(e) => Err(err_as_json_string!(e))
    }
}

fn get_json_string(result: MongoResult<Cursor>) -> String {
    match result {
        Ok(cursor) => {
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
        Err(e) => err_as_json_string!(e)
    }
}

pub fn find(client: Client, context: Context, response: Response) {
    let filter = match context.query.get("filter") {
        Some(json_string) => &json_string[..],
        None => "{}"
    };

    let json = match Json::from_str(filter) {
        Ok(val) => val,
        Err(e) => {
            return respond_with_json_err!(response, e)
        }
    };

    let bson = Bson::from_json(&json);
    let doc = match bson {
        Bson::Document(doc) => doc,
        _ => return respond_with_json_err!(response, "JSON value should be object")
    };

    let db = client.db("mlb");
    let coll = db.collection("players");

    let result = coll.find(Some(doc), None);

    respond!(response, get_json_string(result))
}

pub fn find_one(client: Client, context: Context, response: Response) {
    let filter = match context.query.get("filter") {
        Some(json_string) => &json_string[..],
        None => "{}"
    };

    let json = match Json::from_str(filter) {
        Ok(val) => val,
        Err(e) => {
            return respond_with_json_err!(response, e)
        }
    };

    let bson = Bson::from_json(&json);
    let doc = match bson {
        Bson::Document(doc) => doc,
        _ => return respond_with_json_err!(response, "JSON value should be object")
    };

    let db = client.db("mlb");
    let coll = db.collection("players");

    let result = coll.find_one(Some(doc), None);
    let doc_opt = match result {
        Ok(option) => option,
        Err(e) => return respond_with_json_err!(response, e)
    };

    let string = match doc_opt {
        Some(doc) => json_string_from_doc!(doc),
        None => "{}".to_owned()
    };

    respond!(response, format!("{{ \"result\": {}}}", string))
}

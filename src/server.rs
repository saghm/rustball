use bson::{Bson, Document};
use bson::oid::ObjectId;
use mongodb::{Client, ThreadedClient};
use mongodb::cursor::Cursor;
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::FindOptions;
use mongodb::error::Result as MongoResult;
use rustful::{Context, Response};
use rustful::context::ExtQueryBody;

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

macro_rules! find {
    ($client:expr, $filter:expr, $options:expr, $response:expr) => {{
        let db = $client.db("mlb");
        let coll = db.collection("players");
        let result = coll.find($filter, Some($options));
        let string = get_json_string(result);
        respond!($response, string)
    }};
}

macro_rules! aggregate {
    ($client:expr, $pipeline:expr, $response:expr) => {{
        let db = $client.db("mlb");
        let coll = db.collection("players");
        let result = coll.aggregate($pipeline, None);

        let string = get_json_string(result);
        respond!($response, string)
    }};
}

macro_rules! get_id {
    ($context:expr, $response:expr) => {{
        let id_str = match $context.variables.get("id") {
            Some(name) => &name[..],
            None => return respond_with_json_err!($response, "No id specified")
        };

        let id = match ObjectId::with_string(id_str) {
            Ok(oid) => oid,
            Err(e) => return respond_with_json_err!($response, e)
        };

        id
    }}
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

fn averages(high: bool, client: Client, response: Response) {
    let filter = Some(doc! {
        "avg" => { "$ne" => (Bson::Null) }
    });

    let mut options = FindOptions::new();
    options.limit = 20;

    options.projection = Some(doc! {
        "first_name" => 1,
        "last_name" => 1,
        "team" => 1,
        "avg" => 1
    });

    let avg_sort = if high { -1 } else { 1 };

    options.sort = Some(doc! {
        "avg" => avg_sort,
        "team" => 1,
        "last_name" => 1,
        "first_name" => 1
    });

    find!(client, filter, options, response)
}

pub fn highest_averages(client: Client, _context: Context, response: Response) {
    averages(true, client, response)
}

pub fn lowest_averages(client: Client, _context: Context, response: Response) {
    averages(false, client, response)
}

pub fn teams(client: Client, _context: Context, response: Response) {
    let pipeline = vec![
        doc! {
            "$group" => { "_id" => 1, "teams" => { "$addToSet" => "$team" } }
        },
        doc! {
            "$project" => { "_id" => 0, "teams" => 1 }
        }
    ];

    aggregate!(client, pipeline, response)
}

pub fn team_batters(client: Client, _context: Context, response: Response) {
    let pipeline = vec![
        doc! { "$match" => { "position" => { "$ne" => "P" } } },
        doc! {
            "$project" => {
                "team" => 1,
                "bats" => 1,
                "player" => {
                    "_id" => "$_id",
                    "first_name" => "$first_name",
                    "last_name" => "$last_name"
                }
            }
        },
        doc! {
            "$group" => {
                "_id" => "$team",
                "L" => {
                    "$addToSet" => {
                        "$cond" => [
                            { "$eq" => [ "$bats", "L" ] },
                            "$player",
                            (Bson ::Null)
                        ]
                    }
                },
                "R" => {
                    "$addToSet" => {
                        "$cond" => [ { "$eq" => [ "$bats", "R" ] }, "$player", (Bson::Null)]
                    }
                }
            }
        },
        doc! {
            "$project" => {
                "L" => { "$setDifference" => ["$L", [Bson::Null]] },
                "R" => { "$setDifference" => ["$R", [Bson::Null]] }
            }
        }
    ];

    let db = client.db("mlb");
    let coll = db.collection("players");
    let result = coll.aggregate(pipeline, None);

    let string = get_json_string(result);
    respond!(response, string)
}

pub fn player_tags(client: Client, context: Context, response: Response) {
    let id = get_id!(context, response);
    let filter = Some(doc! { "_id" => id });

    let mut options = FindOptions::new();
    options.projection = Some(doc!{
        "first_name" => 1,
        "last_name" => 1,
        "position" => 1,
        "team" => 1,
        "tags" => 1
    });

    let db = client.db("mlb");
    let coll = db.collection("players");
    let result = coll.find_one(filter, Some(options));
    let doc_opt = match result {
        Ok(opt) => opt,
        Err(e) => return respond!(response, json_error_string!(e))
    };

    let string = match doc_opt {
        Some(doc) => format!("{{\"result\":{}}}", Bson::Document(doc).to_json()),
        None => "{}".to_owned()
    };
    respond!(response, string)
}

pub fn add_tag(client: Client, mut context: Context, response: Response) {
    let id = get_id!(context, response);

    let body = match context.body.read_query_body() {
        Ok(body) => body,
        Err(e) => return respond_with_json_err!(response, e)
    };

    let tag = match body.get("tag") {
        Some(string) => string.as_ref(),
        None => return respond_with_json_err!(response, "No tag specified")
    };

    let filter = doc! { "_id" => id };
    let update = doc! { "$addToSet" => { "tags" => tag } };

    let db = client.db("mlb");
    let coll = db.collection("players");
    match coll.update_one(filter, update, false, None) {
        Ok(_) => respond!(response, "{\"result\":\"ok\"}"),
        Err(e) => respond_with_json_err!(response, e)
    }
}

pub fn team_roster(client: Client, context: Context, response: Response) {
    let team = match context.variables.get("team") {
        Some(team_name) => &team_name[..],
        None => return respond_with_json_err!(response, "No team specified")
    };

    let filter = Some(doc! {
        "team" => team
    });

    let mut options = FindOptions::new();

    options.projection = Some(doc! {
        "first_name" => 1,
        "last_name" => 1,
        "position" => 1
    });

    options.sort = Some(doc! {
        "position" => 1,
        "last_name" => 1,
        "first_name" => 1
    });

    find!(client, filter, options, response)
}

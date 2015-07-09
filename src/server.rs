use std::error::Error;

use bson::{Bson, Document};
use mongodb::cursor::Cursor;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::FindOptions;
use rustful::{Context, Response};
use team;

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

fn get_player_string(player: Document) -> Result<(String, String), String> {
    let name = match (player.get("first_name"), player.get("last_name")) {
        (Some(&Bson::String(ref first)), Some(&Bson::String(ref last))) =>
            format!("{} {}", first, last),
        _ => return Err("Unable to parse name from document".to_owned())
    };

    let pos_abbrev = get_string_or_err!(player, "position");

    let position = match pos_abbrev.as_ref() {
        "P" => "Pitcher",
        "C" => "Catcher",
        "IF" => "Infielder",
        "OF" => "Outfielder",
        _ => return Err(format!("Invalid position: {}", pos_abbrev).to_owned())
    };

    Ok((name, position.to_owned()))
}

fn get_team_string(team: &str, cursor: Cursor) -> Result<String, String> {
    let team_name = match team::get_full_team_name(team) {
        Some(name) => name,
        None => return Err(format!("Invalid team: {}", team).to_owned())
    };

    let mut string = format!("<h2>{}</h2><h3><a href=\"/\">Go back</a><h3><table>", team_name);

    for player_result in cursor {
        let player = match player_result {
            Ok(doc) => doc,
            Err(e) => return err_as_string!(e)
        };

        let (name, position) = match get_player_string(player) {
            Ok((name, pos)) => (name, pos),
            Err(_error) => continue
        };

        string.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", name, position));
    }

    Ok(string + "</table>")
}

pub fn handle_team(client: Client, context: Context, response: Response) {
    let team = match context.variables.get("team") {
        Some(team_name) => &team_name[..],
        None => "BOS"
    };

    let string = match get_team(client, team) {
        Ok(cursor) => match get_team_string(team, cursor) {
            Ok(s) => s,
            Err(s) => s
        },
        Err(e) => e
    };

    response.into_writer().send(string);
}

pub fn select_team(_client: Client, _context: Context, response: Response) {
    let mut string = "<select id=\"team\">\n".to_owned();

    for team in team::ALL_TEAMS.iter() {
        let full_name = match team::get_full_team_name(team) {
            Some(name) => name,
            None => continue
        };

        string.push_str(&format!("  <option value=\"{}\">{}</option>\n", team, full_name));
    }

    string.push_str("  </select>\n<br>\n<button onClick=\"\n  \
                       (function() {\n    \
                           var e = document.getElementById('team');\n    \
                           var team = '/' + e.options[e.selectedIndex].value;\n    \
                           window.location.href=team;\n  \
                       })()\">Select Team</button>");

    response.into_writer().send(string);
}

pub fn get_high_average_counts(client: Client) -> Result<Cursor, String> {
    /*
    { $match: { position: { $ne: "P" }, avg: { $gte: 0.3 } } },
    { $group: { _id: "$team" , count: { $sum: 1 } } },
    { $sort: { count: -1, _id: -1 } }
    */

    let db = client.db("mlb");
    let coll = db.collection("players");
    let pipeline = vec![
        doc! {
            "position" => { "$ne" => "P" },
            "avg" => { "$gte" => 0.3 }
        },
        doc! {
            "$group" => { "_id" => "$team", "count" => { "$sum" => 1 } }
        },
        doc! {
            "$sort" => { "count" => -1, "_id" => -1 }
        }
    ];

    let
}

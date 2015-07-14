rustball
========

Simple REST API to demo the MongoDB Rust driver (https://github.com/mongodbinc-interns/mongo-rust-driver-prototype).

Setup
-----

1.	Start `mongod`

2.	Run `mongoimport -d mlb -c players --file mlb_players.json` in the repo root to import the data

3.	Run `cargo build` to compile the app

4.	Run `cargo run` to start the server

Usage
-----

### API endpoints

-	`/averages/high`
	-	Type: GET
	-	Response: list of top 20 hitters by average
-	`/averages/low`
	-	Type: GET
	-	Response: list of bottom 20 hitters by average
-	`/tags/player/:id`
	-	Type: GET
	-	Response: Single document with the player's info and tags
-	`/tags/player/:id/add`
	-	Type: POST
	-	JSON data: "tag" => tag to add to player
	-	Response: Object specifying success or error
-	`/teams/batters`
	-	Type: GET
	-	Response: list of teams with left-handed and right-handed batters grouped
-	`/teams/:team`
	-	Type: GET
	-	Response: list of players on the team
	-	Examples
		-	`/team/BOS`
		-	`/team/PHI`
		-	`/team/KC`

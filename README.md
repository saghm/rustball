rustball
========

Simple webapp to demo the MongoDB Rust driver (https://github.com/mongodbinc-interns/mongo-rust-driver-prototype).

Setup
-----

1.	Start `mongod`

2.	Run `mongoimport -d mlb -c players --file mlb_players.json` in the repo root to import the data

3.	Run `cargo build` to compile the app

4.	Run `cargo run` to start the server

Usage
-----

1.	Load the page `localhost:3000/league` (or any of the other defined pages) in your browser and start using the app!

REST API
--------

-	`/averages/high`
	-	Type: GET
	-	Response: list of top 20 hitters by average
-	`/averages/low`
	-	Type: GET
	-	Response: list of bottom 20 hitters by average
-	`/batters`
	-	Type: GET
	-	Response: list of teams with left-handed and right-handed batters grouped
-	`/league`
	-	Type: GET
	-	Response: list of all teams
-	`/player/:id`
	-	Type: GET
	-	Response: single document with the player's info and tags
-	`/player/:id/add_tag`
	-	Type: POST
	-	JSON data: "tag" => tag to add to player
	-	Response: object specifying success or error
-	`/tags/:tag`
	-	Type: Get
	-	Response: list of players who have the given tag
-	`/team/:team`
	-	Type: GET
	-	Response: list of players on the team
	-	Examples
		-	`/team/BOS`
		-	`/team/PHI`
		-	`/team/KC`
-	`/team/:team/name`
	-	Type: GET
	-	Response: Full name of the team

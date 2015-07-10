rustball
========

Simple webapp to demo the MongoDB Rust driver (https://github.com/mongodbinc-interns/mongo-rust-driver-prototype).

Setup
-----

1.	Import the database

2.	Start `mongod`

3.	Run `mongoimport -d mlb -c players --file mlb_players.json` in the repo root to import the data

4.	Run `cargo build` to compile the app

5.	Run `cargo run` to start the server

Usage
-----

### API endpoints

-	`/team/:team`
	-	Type: GET
	-	Response: list of players on the team
	-	Examples
		-	`/team/BOS`
		-	`/team/PHI`
		-	`/team/KC`
-	`/averages/high`
	-	Type: GET
	-	Response: list of top 20 hitters by average
-	`/averages/low`
	-	Type: GET
	-	Response: list of bottom 20 hitters by average
-	`/teams/bats`
	-	Type: GET
	-	Response: list of teams with left-handed and right-handed batters grouped

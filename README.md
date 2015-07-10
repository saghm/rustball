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

-	`/find/`
	-	GET
	-	Params
		-	`filter`: JSON document
	-	Responds with the results of a `find` query using `filter`

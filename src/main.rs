//Server time (thank you axum examples)
use axum::{
	routing::{get,post},
	http::StatusCode,
	response::IntoResponse,
	Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
//this is for CLI support
use std::io;
//
use rusqlite::{Connection, Result};
use chrono::prelude::*;


// not sure about the structure setup for the main function - pretty sure this is debugging oriented. Will need to update. Delete comment when done.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let serve = Router::new()
		.route("/",get(root))
		.route("/posts",get(all_posts));
	let addr = SocketAddr::from(([127,0,0,1],3000));
	axum::Server::bind(&addr)
		.serve(serve.into_make_service())
		.await
		.unwrap();


	// let conn = Connection::open("post.db")?;
	// conn.execute(
	
	// //Should create the atabase if needed. 
	// 	"create table if not exists notes (
	// 		id integer primary key,
	// 		title text not null unique,
	// 		body text not null unique,
	// 		time_stamp text not null unique
	// 	)",
	// 	[],
	// )?;
	// let mut running = true;
	// while running == true {
	// 	//this CLI stuff needs to be replaced with Server logic later
	// 	let mut buffer = String::new();
	// 	io::stdin().read_line(&mut buffer)?;
	// 	let trimmed_body = buffer.trim();
	// 	//eventually this needs to take the HTTP header (POST, etc) then validate and do thing or return invalid		

	// 	if trimmed_body == "" {
	// 		println!("K. Bye");
	// 		//this terminates the loop
	// 		running = false;
	// 	} else if  trimmed_body == "/add" {
	// 		// this needs to work with JSON I think. We should figure this out first. And pass JSON to this as a standard method (api time) and then just use this logic for adding/updating posts. for now this is a place holder
	// 		println!("Whats the title?");
	// 		let mut title = String::new();
	// 		io::stdin().read_line(&mut title);
	// 		println!("Whats the post body content?");
	// 		let mut body = String::new();
	// 		io::stdin().read_line(&mut body);
	// 		//grab the timestamp and convert to string to put into db
	// 		let local: DateTime<Local> = Local::now();
	// 		let local = &local.to_string();
			
	// 		//CREATE query for new post
			
	// 		conn.execute("INSERT INTO notes (title,body,time_stamp) values (?1, ?2, ?3)",
	// 		[&title,&body,&local])?;
	// 		println!("Row added. Type /list to see all rows");
	// 	} else if trimmed_body == "/list" {
	// 		let mut stmt = conn.prepare("SELECT * from notes")?;
	// 		let mut rows = stmt.query(rusqlite::params![])?;
	// 		while let Some(row) = rows.next()? {
	// 			let id: i32 = row.get(0)?;
	// 			let title: String = row.get(1)?;
	// 			let body: String = row.get(2)?;
	// 			let time_stamp: String = row.get(3)?;
	// 			println!("id: {}, title: {}, body: {}, timestamp: {}",id,title.to_string(),body.to_string(),time_stamp.to_string());
    // 		}
	// 	}
	// 	else {
	// 		println!("You gotat say /add or nothing. If you say nothing I'm leaving though.");
	// 	}
	// }
	Ok(())
}

//under heavy construction currently just flushing out server logic - this should be base url
async fn root() -> &'static str {
	"This is working at least"
}
// address/all_posts should yield this: need to refactor for standard DB access
// this is not working at all. I probably just need to redo this. 
async fn all_posts () -> String {
	let conn = Connection::open("post.db");
	let mut stmt = conn.expect("Unable to connect to database").prepare("SELECT * from notes");
	let mut rows = stmt.expect("Unable to access database or execute query").query(rusqlite::params![]);
	let mut output = String::new();
	while let Ok(Some(row)) = rows.expect("There were no rows found.").next() {
		let id: i32 = row.get(0).unwrap();
		let title: String = row.get(1).unwrap();
		let body: String = row.get(2).unwrap();
		let time_stamp: String = row.get(3).unwrap();
		output += &id.to_string()
		//+ &title.to_string() + &body.to_string() + &time_stamp.to_string();
	}
	output
}

/*
To do:
	Start working on this project like its a server side applicaton!
	Implement REST API server
		Configure Server 
		Establish functionalities
			GET 
			POST
			DELETE
			UPDATE
		Map CRUD to GPPU (
			Create = POST
			Read = GET
			UPDATE = PUT
			Delete = DELETE
		)
	Decide on login system. 
		Likely add separate db table for logins
		Needs to hash password and can update password etc
	Connect a simple front end for testing in browser
	Decide on how to handle markup - WYSIWYG editor markup?
*/
use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

mod database;
mod models;

#[macro_use]
extern crate simple_log;

use dotenv::dotenv;
use tokio::net::TcpListener; 
use simple_log::LogConfigBuilder;
use sqlx::SqlitePool;
use std::{env, error::Error};

#[tokio::main]
async fn main() {
    let config = LogConfigBuilder::builder()
        .path("./log/rust_micro_cms.log")
        .size(1 * 1024 * 1024) // 1 MB
        .roll_count(5)
        .level("info")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config);

    dotenv().ok();
    let db_path = &env::var("DATABASE_URL").expect("DATABASE_URL Must be set in .env file");
    let pool = SqlitePool::connect(&db_path).await;

    info!("Rust Micro CMS started");
    let app = Router::new()
        .route("/", get(StatusCode::OK.with_body("under construction")));

    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Failed to bind");
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

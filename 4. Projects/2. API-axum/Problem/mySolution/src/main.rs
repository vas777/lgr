
use std::env;

use axum::{
    Error, Router,
    routing::{delete, get, post},
};

mod handlers;
mod models;

use handlers::*;
use log::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    dotenvy::dotenv().expect("where is my dotenv?");

    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set ?");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url).await.unwrap();

    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .unwrap();


    info!("********* Question Records *********");
    info!("{:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Start serving ...");
    axum::serve(listener, app).await.unwrap();
}

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    dotenv().expect("where is my dotenv?");

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set ?");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone())); // create a new instance of QuestionsDaoImpl passing in `pool` (use the clone method)
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool)); // create a new instance of AnswersDaoImpl passing in `pool`

    let app_state = AppState{
        questions_dao: questions_dao,
        answers_dao: answers_dao,
    }; // create a new instance of AppState


    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
            // The with_state method allows us to add state to the state managed by this instance of Axum. Then we can use this state in the handlers.
        .with_state(app_state); // pass in `app_state` as application state.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Start serving ...");
    axum::serve(listener, app).await.unwrap();
}

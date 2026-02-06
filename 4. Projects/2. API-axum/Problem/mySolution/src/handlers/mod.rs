use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::*, AppState};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> Result<impl IntoResponse,handlers_inner::HandlerError> {
    handlers_inner::create_question(question, questions_dao.as_ref()).await.map(Json)  
}

pub async fn read_questions(// TODO: add questions_dao from app state as an argument
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {

    handlers_inner::read_questions(questions_dao.as_ref()).await.map(Json)
}

pub async fn delete_question(
    // TODO: add questions_dao from app state as an argument
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) {
    handlers_inner::delete_question(question_uuid,questions_dao.as_ref()).await;
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    // Example of how to add state to a route
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    handlers_inner::create_answer(answer, answers_dao.as_ref()).await.map(Json)
}

pub async fn read_answers(
    // TODO: add answers_dao from app state as an argument
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    handlers_inner::read_answers(question_uuid, answers_dao.as_ref()).await.map(Json)
}

pub async fn delete_answer(
    // TODO: add answers_dao from app state as an argument
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) {
    handlers_inner::delete_answer(answer_uuid, answers_dao.as_ref()).await;
}

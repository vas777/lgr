use crate::models::*;
use axum::{response::IntoResponse, Json};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    println!("Creating question");
    let q: Question = question.into();

    dbg!(&q);

    Json(QuestionDetail {
        question_uuid: "uuid".to_owned(),
        title: q.title.clone(),
        description: q.description.clone(),
        created_at: "today".to_owned(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    println!("Reading questions");
    Json(vec![QuestionDetail{
        question_uuid: "uuid".to_owned(),
        title: "titel".to_owned(),
        description: "q.description".to_owned(),
        created_at: "today".to_owned(),
    }])
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    println!("Deliting question");
    let q : QuestionId =  question_uuid.into();
    dbg!(q);
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above
pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    println!("Creating question");
    let a: Answer = answer.into();

    dbg!(&a);

    Json(Answer{
        question_uuid: "quetion_uuid".to_owned(),
        content: a.content.clone(),
    })
}


// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
pub async fn read_answers() -> impl IntoResponse {
    Json(vec![
        AnswerDetail{
            answer_uuid: "auuid".to_owned(),
            question_uuid: "quuid".to_owned(),
            content: "content".to_owned(),
            created_at: "today".to_owned(),
            
        }
    ])
}


// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>){
    println!("Deliting answer");
    let a : AnswerId =  answer_uuid.into();
    dbg!(a);
}
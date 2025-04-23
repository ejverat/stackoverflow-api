use crate::models::*;
use axum::{Json, http::StatusCode, response::IntoResponse};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    let question_created = QuestionDetail {
        question_uuid: "invalid uuid".to_owned(),
        title: question.title,
        description: question.description,
        created_at: "invalid date".to_owned(),
    };
    Json(question_created)
}

pub async fn read_questions() -> impl IntoResponse {
    let mut questions = vec![];

    let question_dummy = QuestionDetail {
        question_uuid: "invalid uuid".to_owned(),
        title: "Test title".to_owned(),
        description: "Test description".to_owned(),
        created_at: "invalid date".to_owned(),
    };

    questions.push(question_dummy);

    Json(questions)
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    //
}

// ---- CRUD for Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let created_answer = AnswerDetail {
        answer_uuid: "invalid uuid".to_owned(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "invalid date".to_owned(),
    };

    Json(created_answer)
}

pub async fn read_answers(Json(question_id): Json<QuestionId>) -> impl IntoResponse {
    let mut answers = vec![];

    let created_answer = AnswerDetail {
        answer_uuid: "invalid uuid".to_owned(),
        question_uuid: question_id.question_uuid,
        content: "Test answer content".to_owned(),
        created_at: "invalid date".to_owned(),
    };

    answers.push(created_answer);

    Json(answers)
}

pub async fn delete_answer(Json(answer_id): Json<AnswerId>) {
    //
}

use crate::application::use_cases::index_use_case::IndexUseCase;
use actix_web::{HttpResponse, Responder};

pub async fn index_controller() -> impl Responder {
    let greeting = IndexUseCase::execute();
    HttpResponse::Ok().body(greeting.message)
}

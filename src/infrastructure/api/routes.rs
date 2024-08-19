use crate::infrastructure::api::controllers::index_controller;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index_controller::index_controller)));
}

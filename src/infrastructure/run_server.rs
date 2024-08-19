use crate::infrastructure::api::routes::configure;
use actix_web::{App, HttpServer};

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

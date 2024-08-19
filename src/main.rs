mod application;
mod core;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infrastructure::run_server::run_server().await
}

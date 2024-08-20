use crate::core::models::index_model;
use crate::core::services::index_service;
pub struct IndexUseCase;
impl IndexUseCase {
    pub async fn execute() -> index_model::Response {
        index_service::IndexService::send_hello_world().await
    }
}

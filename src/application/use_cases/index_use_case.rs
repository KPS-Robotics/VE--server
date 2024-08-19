use crate::core::models::index_model;
use crate::core::services::index_service;
pub struct IndexUseCase;
impl IndexUseCase {
    pub fn execute() -> index_model::Response {
        index_service::IndexServices::send_hello_world()
    }
}

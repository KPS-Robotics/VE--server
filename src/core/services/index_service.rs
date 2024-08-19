use crate::core::models::index_model;
pub struct IndexServices;

impl IndexServices {
    pub fn send_hello_world() -> index_model::Response {
        index_model::Response::new("Hello world")
    }
}

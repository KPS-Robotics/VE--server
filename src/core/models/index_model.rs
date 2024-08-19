pub struct Response {
    pub message: String,
}
impl Response {
    pub fn new(message: &str) -> Self {
        Response {
            message: message.to_string(),
        }
    }
}

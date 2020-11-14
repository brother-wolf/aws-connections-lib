#[derive(Debug, PartialEq)]
pub struct ClientError {
    message: String,
}

impl ClientError {
    pub fn from(message: String) -> ClientError {
        ClientError {message}
    }
}
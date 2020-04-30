use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("TypeError: {0}")]
    TypeError(String),
    #[error("InternalServerError: {0}")]
    InternalServerError(String),
    #[error("NoneError: {0}")]
    NoneError(String),
}

impl MyError {
    pub fn type_error(s: &str) -> MyError {
        MyError::TypeError(s.to_string())
    }

    pub fn internal_server_error(s: &str) -> MyError {
        MyError::InternalServerError(s.to_string())
    }

    pub fn none_error(s: &str) -> MyError {
        MyError::NoneError(s.to_string())
    }
}
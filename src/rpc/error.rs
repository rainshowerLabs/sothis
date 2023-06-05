use std::{fmt, num::ParseIntError};

#[derive(Debug)]
#[allow(dead_code)]
pub enum RequestError {
    RequestFailed(String),
    JsonDeserializationFailed(String),
    JsonSerializationFailed(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::RequestFailed(err) => write!(f, "Request failed: {}", err),
            RequestError::JsonSerializationFailed(err) => {
                write!(f, "JSON serialization failed: {}", err)
            }
            RequestError::JsonDeserializationFailed(err) => {
                write!(f, "JSON deserialization failed: {}", err)
            }
        }
    }
}

impl std::error::Error for RequestError {}

// Implement From trait conversions
impl From<ParseIntError> for RequestError {
    fn from(err: ParseIntError) -> Self {
        RequestError::RequestFailed(err.to_string())
    }
}

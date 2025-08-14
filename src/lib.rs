use serde::Deserialize;
use serde_json::Error as JsonError;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
pub mod geolocation;
pub mod monitoring;

#[derive(Deserialize, Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<JsonError> for Error {
    fn from(json_error: JsonError) -> Self {
        Error {
            message: format!("A deserialisation error occurred {json_error}"),
        }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error {
            message: format!("An IO error occurred: {error}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error {
            message: format!("An HTTP error occurred: {value}"),
        }
    }
}

impl From<reqwest::StatusCode> for Error {
    fn from(status: reqwest::StatusCode) -> Self {
        Error {
            message: format!("An HTTP error occurred with status code: {}", status),
        }
    }
}

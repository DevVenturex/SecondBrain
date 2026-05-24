use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(std::io::Error),
    #[error("Resource not found: {0}")]
    NotFound(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::IOError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg)
        };
        (status, msg).into_response()
    }
}
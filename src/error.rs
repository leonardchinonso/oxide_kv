use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
pub enum OxideKvError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Server error: {0}")]
    Server(String),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

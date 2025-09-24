use hyperware_process_lib::http::client::HttpClientError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum TtsError {
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] HttpClientError),
    #[error("Invalid model: {0}")]
    InvalidModel(String),
    #[error("Invalid voice: {0}")]
    InvalidVoice(String),
    #[error("Input text too long (max 4096 characters), got {0} characters")]
    InputTooLong(usize),
    #[error("Invalid speed: {0} (must be between 0.25 and 4.0)")]
    InvalidSpeed(f32),
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },
    #[error("Missing API key")]
    MissingApiKey,
    #[error("Input text is required")]
    MissingInput,
    #[error("Failed to serialize request: {0}")]
    SerializationError(String),
}
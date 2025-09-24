use hyperware_process_lib::http::client::HttpClientError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum SttError {
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] HttpClientError),
    #[error("Invalid model: {0}")]
    InvalidModel(String),
    #[error("Invalid audio format: {0}")]
    InvalidAudioFormat(String),
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Missing API key")]
    MissingApiKey,
    #[error("Failed to build multipart form: {0}")]
    MultipartError(String),
    #[error("File is required for transcription")]
    MissingFile,
}
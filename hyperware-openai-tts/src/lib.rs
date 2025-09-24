pub mod client;
pub mod error;
pub mod types;

#[cfg(test)]
mod tests;

pub use client::{SpeechClient, SpeechRequestBuilder};
pub use error::TtsError;
pub use types::{
    AudioFormat, SpeechRequest, SpeechResponse, TtsModel, Voice,
};
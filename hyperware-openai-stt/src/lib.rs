pub mod client;
pub mod error;
pub mod multipart;
pub mod types;

#[cfg(test)]
mod tests;

pub use client::{TranscriptionClient, TranscriptionRequestBuilder};
pub use error::SttError;
pub use types::{
    Model, ResponseFormat, TranscriptionRequest, TranscriptionResponse,
    TokenDetails, Usage,
};
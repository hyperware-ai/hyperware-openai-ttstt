use crate::error::TtsError;
use crate::types::{
    ApiErrorResponse, AudioFormat, SpeechRequest, SpeechRequestJson, SpeechResponse, TtsModel,
    Voice,
};
use hyperware_process_lib::http::client::send_request_await_response;
use hyperware_process_lib::http::client::HttpClientError;
use http::Method;
use std::collections::HashMap;

const MAX_INPUT_LENGTH: usize = 4096;
const MIN_SPEED: f32 = 0.25;
const MAX_SPEED: f32 = 4.0;

pub struct SpeechClient {
    api_key: String,
    base_url: String,
    timeout: u64,
}

impl SpeechClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com".to_string(),
            timeout: 60000, // 60 seconds default
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn synthesize(&self) -> SpeechRequestBuilder {
        SpeechRequestBuilder {
            client: self,
            request: SpeechRequest::default(),
        }
    }

    async fn send_speech_request(
        &self,
        request: SpeechRequest,
    ) -> Result<SpeechResponse, TtsError> {
        // Validate input
        if request.input.is_empty() {
            return Err(TtsError::MissingInput);
        }

        if request.input.len() > MAX_INPUT_LENGTH {
            return Err(TtsError::InputTooLong(request.input.len()));
        }

        if let Some(speed) = request.speed {
            if speed < MIN_SPEED || speed > MAX_SPEED {
                return Err(TtsError::InvalidSpeed(speed));
            }
        }

        if self.api_key.is_empty() {
            return Err(TtsError::MissingApiKey);
        }

        // Convert to JSON request
        let json_request = SpeechRequestJson::from(request.clone());
        
        // Serialize to JSON
        let body = serde_json::to_vec(&json_request)
            .map_err(|e| TtsError::SerializationError(e.to_string()))?;

        // Prepare headers
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        // Construct URL
        let url = url::Url::parse(&format!("{}/v1/audio/speech", self.base_url))
            .map_err(|e| TtsError::HttpClient(HttpClientError::BadUrl {
                url: e.to_string(),
            }))?;

        // Send request
        let response = send_request_await_response(
            Method::POST,
            url,
            Some(headers),
            self.timeout,
            body,
        )
        .await
        .map_err(TtsError::HttpClient)?;

        // Handle response
        let status = response.status();
        let body = response.into_body();

        if status.is_success() {
            // Success - body contains raw audio data
            let format = request.response_format.unwrap_or(AudioFormat::Mp3);
            Ok(SpeechResponse {
                audio_data: body,
                format,
            })
        } else {
            // Try to parse error response
            if let Ok(error_response) = serde_json::from_slice::<ApiErrorResponse>(&body) {
                Err(TtsError::ApiError {
                    status: status.as_u16(),
                    message: error_response.error.message,
                })
            } else {
                // Fallback to raw body text
                let message = String::from_utf8_lossy(&body).to_string();
                Err(TtsError::ApiError {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }
}

pub struct SpeechRequestBuilder<'a> {
    client: &'a SpeechClient,
    request: SpeechRequest,
}

impl<'a> SpeechRequestBuilder<'a> {
    pub fn input(mut self, text: impl Into<String>) -> Self {
        self.request.input = text.into();
        self
    }

    pub fn model(mut self, model: TtsModel) -> Self {
        self.request.model = model;
        self
    }

    pub fn voice(mut self, voice: Voice) -> Self {
        self.request.voice = voice;
        self
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request.instructions = Some(instructions.into());
        self
    }

    pub fn response_format(mut self, format: AudioFormat) -> Self {
        self.request.response_format = Some(format);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.request.speed = Some(speed);
        self
    }

    pub async fn execute(self) -> Result<SpeechResponse, TtsError> {
        self.client.send_speech_request(self.request).await
    }
}


use crate::error::SttError;
use crate::multipart::{get_content_type_for_extension, MultipartFormData};
use crate::types::{ApiErrorResponse, Model, ResponseFormat, TranscriptionRequest, TranscriptionResponse};
use hyperware_process_lib::http::client::send_request_await_response;
use hyperware_process_lib::http::client::HttpClientError;
use http::Method;
use std::collections::HashMap;

pub struct TranscriptionClient {
    api_key: String,
    base_url: String,
    timeout: u64,
}

impl TranscriptionClient {
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

    pub fn transcribe(&self) -> TranscriptionRequestBuilder {
        TranscriptionRequestBuilder {
            client: self,
            request: TranscriptionRequest::default(),
        }
    }

    async fn send_transcription_request(
        &self,
        request: TranscriptionRequest,
    ) -> Result<TranscriptionResponse, SttError> {
        if request.file.is_empty() {
            return Err(SttError::MissingFile);
        }

        if self.api_key.is_empty() {
            return Err(SttError::MissingApiKey);
        }

        // Build multipart form data
        let mut form = MultipartFormData::new();
        
        // Add file
        let content_type = get_content_type_for_extension(&request.file_name);
        form.add_file("file", &request.file_name, content_type, request.file);
        
        // Add model
        form.add_text("model", request.model.as_str());
        
        // Add optional fields
        if let Some(language) = request.language {
            form.add_text("language", language);
        }
        
        if let Some(prompt) = request.prompt {
            form.add_text("prompt", prompt);
        }
        
        if let Some(format) = request.response_format {
            form.add_text("response_format", format.as_str());
        }
        
        if let Some(temperature) = request.temperature {
            form.add_text("temperature", temperature.to_string());
        }

        let (body, content_type) = form.build();

        // Prepare headers
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.api_key));
        headers.insert("Content-Type".to_string(), content_type);

        // Construct URL
        let url = url::Url::parse(&format!("{}/v1/audio/transcriptions", self.base_url))
            .map_err(|e| SttError::HttpClient(HttpClientError::BadUrl {
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
        .map_err(SttError::HttpClient)?;

        // Handle response
        let status = response.status();
        let body = response.into_body();

        if status.is_success() {
            // Parse successful response
            serde_json::from_slice(&body)
                .map_err(|e| SttError::ParseError(e.to_string()))
        } else {
            // Try to parse error response
            if let Ok(error_response) = serde_json::from_slice::<ApiErrorResponse>(&body) {
                Err(SttError::ApiError {
                    status: status.as_u16(),
                    message: error_response.error.message,
                })
            } else {
                // Fallback to raw body text
                let message = String::from_utf8_lossy(&body).to_string();
                Err(SttError::ApiError {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }
}

pub struct TranscriptionRequestBuilder<'a> {
    client: &'a TranscriptionClient,
    request: TranscriptionRequest,
}

impl<'a> TranscriptionRequestBuilder<'a> {
    pub fn file(mut self, data: Vec<u8>, name: impl Into<String>) -> Self {
        self.request.file = data;
        self.request.file_name = name.into();
        self
    }

    pub fn model(mut self, model: Model) -> Self {
        self.request.model = model;
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.request.language = Some(language.into());
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.request.prompt = Some(prompt.into());
        self
    }

    pub fn response_format(mut self, format: ResponseFormat) -> Self {
        self.request.response_format = Some(format);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    pub async fn execute(self) -> Result<TranscriptionResponse, SttError> {
        self.client.send_transcription_request(self.request).await
    }
}


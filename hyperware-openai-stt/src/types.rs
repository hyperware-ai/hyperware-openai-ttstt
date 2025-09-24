use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    #[serde(rename = "gpt-4o-transcribe")]
    Gpt4oTranscribe,
    #[serde(rename = "gpt-4o-mini-transcribe")]
    Gpt4oMiniTranscribe,
    #[serde(rename = "whisper-1")]
    Whisper1,
}

impl Model {
    pub fn as_str(&self) -> &str {
        match self {
            Model::Gpt4oTranscribe => "gpt-4o-transcribe",
            Model::Gpt4oMiniTranscribe => "gpt-4o-mini-transcribe",
            Model::Whisper1 => "whisper-1",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Json,
    Text,
    Srt,
    #[serde(rename = "verbose_json")]
    VerboseJson,
    Vtt,
}

impl ResponseFormat {
    pub fn as_str(&self) -> &str {
        match self {
            ResponseFormat::Json => "json",
            ResponseFormat::Text => "text",
            ResponseFormat::Srt => "srt",
            ResponseFormat::VerboseJson => "verbose_json",
            ResponseFormat::Vtt => "vtt",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TranscriptionRequest {
    pub file: Vec<u8>,
    pub file_name: String,
    pub model: Model,
    pub language: Option<String>,
    pub prompt: Option<String>,
    pub response_format: Option<ResponseFormat>,
    pub temperature: Option<f32>,
}

impl Default for TranscriptionRequest {
    fn default() -> Self {
        Self {
            file: Vec::new(),
            file_name: String::new(),
            model: Model::Whisper1,
            language: None,
            prompt: None,
            response_format: None,
            temperature: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Usage {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_token_details: Option<TokenDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TtsModel {
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1Hd,
    #[serde(rename = "gpt-4o-mini-tts")]
    Gpt4oMiniTts,
}

impl TtsModel {
    pub fn as_str(&self) -> &str {
        match self {
            TtsModel::Tts1 => "tts-1",
            TtsModel::Tts1Hd => "tts-1-hd",
            TtsModel::Gpt4oMiniTts => "gpt-4o-mini-tts",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Fable,
    Onyx,
    Nova,
    Sage,
    Shimmer,
    Verse,
}

impl Voice {
    pub fn as_str(&self) -> &str {
        match self {
            Voice::Alloy => "alloy",
            Voice::Ash => "ash",
            Voice::Ballad => "ballad",
            Voice::Coral => "coral",
            Voice::Echo => "echo",
            Voice::Fable => "fable",
            Voice::Onyx => "onyx",
            Voice::Nova => "nova",
            Voice::Sage => "sage",
            Voice::Shimmer => "shimmer",
            Voice::Verse => "verse",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Mp3,
    Opus,
    Aac,
    Flac,
    Wav,
    Pcm,
}

impl AudioFormat {
    pub fn as_str(&self) -> &str {
        match self {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Opus => "opus",
            AudioFormat::Aac => "aac",
            AudioFormat::Flac => "flac",
            AudioFormat::Wav => "wav",
            AudioFormat::Pcm => "pcm",
        }
    }
}

impl Default for AudioFormat {
    fn default() -> Self {
        AudioFormat::Mp3
    }
}

#[derive(Debug, Clone)]
pub struct SpeechRequest {
    pub input: String,
    pub model: TtsModel,
    pub voice: Voice,
    pub instructions: Option<String>,
    pub response_format: Option<AudioFormat>,
    pub speed: Option<f32>,
}

impl Default for SpeechRequest {
    fn default() -> Self {
        Self {
            input: String::new(),
            model: TtsModel::Tts1,
            voice: Voice::Alloy,
            instructions: None,
            response_format: None,
            speed: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpeechRequestJson {
    pub input: String,
    pub model: String,
    pub voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

impl From<SpeechRequest> for SpeechRequestJson {
    fn from(req: SpeechRequest) -> Self {
        Self {
            input: req.input,
            model: req.model.as_str().to_string(),
            voice: req.voice.as_str().to_string(),
            instructions: req.instructions,
            response_format: req.response_format.map(|f| f.as_str().to_string()),
            speed: req.speed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpeechResponse {
    pub audio_data: Vec<u8>,
    pub format: AudioFormat,
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
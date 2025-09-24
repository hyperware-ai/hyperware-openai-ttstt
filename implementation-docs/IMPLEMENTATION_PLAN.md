# OpenAI STT and TTS Libraries Implementation Plan

## Overview
This document outlines the implementation plan for two Rust libraries that provide Hyperware processes with access to OpenAI's Speech-to-Text (STT) and Text-to-Speech (TTS) APIs. Both libraries will use the Hyperware HTTP client for all API communications and will NOT support streaming mode.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Hyperware Process                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐     ┌─────────────────────┐      │
│  │ hyperware-openai-stt │     │ hyperware-openai-tts │      │
│  └──────────┬──────────┘     └──────────┬──────────┘      │
│             │                            │                  │
│             └──────────┬─────────────────┘                  │
│                        ▼                                    │
│             ┌─────────────────────┐                        │
│             │ Hyperware HTTP Client│                        │
│             └──────────┬──────────┘                        │
└────────────────────────┼────────────────────────────────────┘
                         ▼
                   OpenAI API Endpoints

```

## Library 1: hyperware-openai-stt

### Purpose
Provide speech-to-text transcription capabilities using OpenAI's transcription API.

### Core Components

#### 1. TranscriptionClient
```rust
pub struct TranscriptionClient {
    api_key: String,
    base_url: String,
    timeout: u64,
}
```

#### 2. Request Types
```rust
pub enum Model {
    Gpt4oTranscribe,
    Gpt4oMiniTranscribe,
    Whisper1,
}

pub enum ResponseFormat {
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

pub struct TranscriptionRequest {
    file: Vec<u8>,
    file_name: String,
    model: Model,
    language: Option<String>,
    prompt: Option<String>,
    response_format: Option<ResponseFormat>,
    temperature: Option<f32>,
}
```

#### 3. Response Types
```rust
pub struct TranscriptionResponse {
    pub text: String,
    pub usage: Option<Usage>,
}

pub struct Usage {
    pub r#type: String,
    pub input_tokens: u32,
    pub input_token_details: TokenDetails,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

pub struct TokenDetails {
    pub text_tokens: u32,
    pub audio_tokens: u32,
}
```

#### 4. Error Handling
```rust
#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum SttError {
    #[error("HTTP client error: {0}")]
    HttpClient(HttpClientError),
    #[error("Invalid model: {0}")]
    InvalidModel(String),
    #[error("Invalid audio format")]
    InvalidAudioFormat,
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Missing API key")]
    MissingApiKey,
}
```

### API Design
```rust
// Basic usage
let client = TranscriptionClient::new(api_key);
let response = client.transcribe()
    .file(audio_bytes, "audio.mp3")
    .model(Model::Gpt4oTranscribe)
    .execute()
    .await?;

// Advanced usage with optional parameters
let response = client.transcribe()
    .file(audio_bytes, "speech.wav")
    .model(Model::Whisper1)
    .language("en")
    .prompt("Technical discussion about Rust")
    .response_format(ResponseFormat::VerboseJson)
    .temperature(0.2)
    .execute()
    .await?;
```

### Implementation Details

1. **Multipart Form Data Construction**
   - Build multipart/form-data request body
   - Include file as binary data with proper Content-Type
   - Add all text parameters as form fields

2. **HTTP Request**
   ```rust
   // Endpoint: POST https://api.openai.com/v1/audio/transcriptions
   // Headers:
   //   - Authorization: Bearer {api_key}
   //   - Content-Type: multipart/form-data; boundary={boundary}
   ```

3. **Supported Audio Formats**
   - flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, webm

## Library 2: hyperware-openai-tts

### Purpose
Provide text-to-speech synthesis capabilities using OpenAI's speech API.

### Core Components

#### 1. SpeechClient
```rust
pub struct SpeechClient {
    api_key: String,
    base_url: String,
    timeout: u64,
}
```

#### 2. Request Types
```rust
pub enum TtsModel {
    Tts1,
    Tts1Hd,
    Gpt4oMiniTts,
}

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

pub enum AudioFormat {
    Mp3,
    Opus,
    Aac,
    Flac,
    Wav,
    Pcm,
}

pub struct SpeechRequest {
    input: String,
    model: TtsModel,
    voice: Voice,
    instructions: Option<String>,
    response_format: Option<AudioFormat>,
    speed: Option<f32>,
}
```

#### 3. Response Type
```rust
pub struct SpeechResponse {
    pub audio_data: Vec<u8>,
    pub format: AudioFormat,
}
```

#### 4. Error Handling
```rust
#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum TtsError {
    #[error("HTTP client error: {0}")]
    HttpClient(HttpClientError),
    #[error("Invalid model: {0}")]
    InvalidModel(String),
    #[error("Invalid voice: {0}")]
    InvalidVoice(String),
    #[error("Input text too long (max 4096 characters)")]
    InputTooLong,
    #[error("Invalid speed: {0} (must be between 0.25 and 4.0)")]
    InvalidSpeed(f32),
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },
    #[error("Missing API key")]
    MissingApiKey,
}
```

### API Design
```rust
// Basic usage
let client = SpeechClient::new(api_key);
let audio = client.synthesize()
    .input("Hello, world!")
    .model(TtsModel::Gpt4oMiniTts)
    .voice(Voice::Alloy)
    .execute()
    .await?;

// Advanced usage with optional parameters
let audio = client.synthesize()
    .input(text)
    .model(TtsModel::Gpt4oMiniTts)
    .voice(Voice::Nova)
    .instructions("Speak with enthusiasm and energy")
    .response_format(AudioFormat::Wav)
    .speed(1.2)
    .execute()
    .await?;
```

### Implementation Details

1. **JSON Request Body Construction**
   ```rust
   {
       "input": "text to synthesize",
       "model": "gpt-4o-mini-tts",
       "voice": "alloy",
       "instructions": "optional instructions",
       "response_format": "mp3",
       "speed": 1.0
   }
   ```

2. **HTTP Request**
   ```rust
   // Endpoint: POST https://api.openai.com/v1/audio/speech
   // Headers:
   //   - Authorization: Bearer {api_key}
   //   - Content-Type: application/json
   ```

3. **Response Handling**
   - Direct binary audio data in specified format
   - No JSON parsing required for response body

## Common Implementation Aspects

### Hyperware HTTP Client Integration

Both libraries will use the provided Hyperware HTTP client:
```rust
use http::Method;

pub async fn send_request_await_response(
    method: Method,
    url: url::Url,
    headers: Option<HashMap<String, String>>,
    timeout: u64,
    body: Vec<u8>,
) -> std::result::Result<http::Response<Vec<u8>>, HttpClientError>
```

### Shared Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
http = "1.0"
url = "2.5"
thiserror = "1.0"
```

### Builder Pattern Implementation

Both libraries will use the builder pattern for constructing requests:

```rust
pub struct TranscriptionRequestBuilder<'a> {
    client: &'a TranscriptionClient,
    request: TranscriptionRequest,
}

impl<'a> TranscriptionRequestBuilder<'a> {
    pub fn file(mut self, data: Vec<u8>, name: String) -> Self {
        self.request.file = data;
        self.request.file_name = name;
        self
    }
    
    pub fn model(mut self, model: Model) -> Self {
        self.request.model = model;
        self
    }
    
    pub async fn execute(self) -> Result<TranscriptionResponse, SttError> {
        self.client.send_transcription_request(self.request).await
    }
}
```

## Testing Strategy

### Unit Tests
- Request serialization/deserialization
- Error handling for invalid inputs
- Response parsing for all formats

### Integration Tests
- Mock HTTP server responses
- Test all parameter combinations
- Error response handling

### Example Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request_serialization() {
        // Test JSON serialization for TTS
        let request = SpeechRequest {
            input: "Test".to_string(),
            model: TtsModel::Gpt4oMiniTts,
            voice: Voice::Alloy,
            instructions: None,
            response_format: Some(AudioFormat::Mp3),
            speed: Some(1.0),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"gpt-4o-mini-tts\""));
    }
    
    #[test]
    fn test_multipart_form_construction() {
        // Test multipart form data for STT
        // Implementation details...
    }
}
```

## Error Handling Philosophy

1. **Fail Fast**: Validate inputs early before making API calls
2. **Detailed Errors**: Provide context about what went wrong
3. **Recoverable Errors**: Allow retry for transient failures
4. **Type Safety**: Use enums for all known values (models, voices, formats)

## Security Considerations

1. **API Key Management**
   - Never log or expose API keys
   - Support environment variable configuration
   - Clear sensitive data from memory when possible

2. **Input Validation**
   - Enforce text length limits (4096 chars for TTS)
   - Validate audio format before sending
   - Sanitize file names in multipart forms

3. **Rate Limiting**
   - Respect OpenAI rate limits
   - Implement exponential backoff for retries
   - Track usage statistics if needed

## Performance Optimizations

1. **Memory Management**
   - Stream large audio files when possible (future enhancement)
   - Use references where appropriate
   - Minimize allocations in hot paths

2. **Connection Pooling**
   - Reuse HTTP connections when possible
   - Configure appropriate timeouts

3. **Caching**
   - Consider caching synthesized speech for repeated text
   - Cache model/voice validation results

## Future Enhancements

1. **Batch Processing**
   - Support multiple transcriptions in parallel
   - Batch TTS requests for efficiency

2. **Advanced Features**
   - Chunking strategy for STT (when non-streaming)
   - Timestamp granularities for verbose transcriptions
   - Log probabilities for confidence scoring

3. **Monitoring**
   - Usage tracking
   - Performance metrics
   - Error rate monitoring

## Implementation Timeline

### Phase 1: Core Implementation (Week 1)
- [ ] Set up project structure and dependencies
- [ ] Implement basic STT client with required parameters
- [ ] Implement basic TTS client with required parameters
- [ ] Basic error handling

### Phase 2: Full Feature Set (Week 2)
- [ ] Add all optional parameters for STT
- [ ] Add all optional parameters for TTS
- [ ] Comprehensive error handling
- [ ] Input validation

### Phase 3: Testing & Documentation (Week 3)
- [ ] Unit tests for all components
- [ ] Integration tests with mock server
- [ ] API documentation
- [ ] Usage examples

### Phase 4: Optimization & Polish (Week 4)
- [ ] Performance optimizations
- [ ] Security review
- [ ] Final documentation
- [ ] Release preparation

## Success Criteria

1. **Functional Requirements**
   - Successfully transcribe audio in all supported formats
   - Successfully synthesize speech with all voice options
   - Handle all API errors gracefully

2. **Non-Functional Requirements**
   - Response time < 5 seconds for typical requests
   - Memory usage < 100MB for typical audio files
   - 100% test coverage for critical paths

3. **Developer Experience**
   - Intuitive API with builder pattern
   - Clear error messages
   - Comprehensive documentation with examples

## Conclusion

This implementation plan provides a solid foundation for building robust OpenAI STT and TTS libraries for Hyperware processes. The design emphasizes type safety, ease of use, and proper error handling while leveraging the existing Hyperware HTTP client infrastructure.
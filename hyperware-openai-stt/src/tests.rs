#[cfg(test)]
mod tests {
    use crate::multipart::{get_content_type_for_extension, MultipartFormData};
    use crate::types::{Model, ResponseFormat, TranscriptionResponse};

    #[test]
    fn test_content_type_detection() {
        assert_eq!(get_content_type_for_extension("audio.mp3"), "audio/mpeg");
        assert_eq!(get_content_type_for_extension("test.wav"), "audio/wav");
        assert_eq!(get_content_type_for_extension("file.flac"), "audio/flac");
        assert_eq!(get_content_type_for_extension("something.ogg"), "audio/ogg");
        assert_eq!(get_content_type_for_extension("unknown.xyz"), "application/octet-stream");
    }

    #[test]
    fn test_multipart_form_construction() {
        let mut form = MultipartFormData::new();
        form.add_text("name", "test_value");
        form.add_file("file", "test.txt", "text/plain", b"file content".to_vec());
        
        let (body, content_type) = form.build();
        
        // Check that content type contains boundary
        assert!(content_type.starts_with("multipart/form-data; boundary="));
        
        // Check that body contains the expected parts
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("Content-Disposition: form-data; name=\"name\""));
        assert!(body_str.contains("test_value"));
        assert!(body_str.contains("Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\""));
        assert!(body_str.contains("Content-Type: text/plain"));
        assert!(body_str.contains("file content"));
    }

    #[test]
    fn test_model_serialization() {
        assert_eq!(Model::Gpt4oTranscribe.as_str(), "gpt-4o-transcribe");
        assert_eq!(Model::Gpt4oMiniTranscribe.as_str(), "gpt-4o-mini-transcribe");
        assert_eq!(Model::Whisper1.as_str(), "whisper-1");
    }

    #[test]
    fn test_response_format_serialization() {
        assert_eq!(ResponseFormat::Json.as_str(), "json");
        assert_eq!(ResponseFormat::Text.as_str(), "text");
        assert_eq!(ResponseFormat::Srt.as_str(), "srt");
        assert_eq!(ResponseFormat::VerboseJson.as_str(), "verbose_json");
        assert_eq!(ResponseFormat::Vtt.as_str(), "vtt");
    }

    #[test]
    fn test_transcription_response_deserialization() {
        let json = r#"{
            "text": "Hello, world!",
            "usage": {
                "type": "tokens",
                "input_tokens": 14,
                "input_token_details": {
                    "text_tokens": 0,
                    "audio_tokens": 14
                },
                "output_tokens": 3,
                "total_tokens": 17
            }
        }"#;

        let response: TranscriptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text, "Hello, world!");
        assert!(response.usage.is_some());
        
        let usage = response.usage.unwrap();
        assert_eq!(usage.usage_type, Some("tokens".to_string()));
        assert_eq!(usage.input_tokens, Some(14));
        assert_eq!(usage.output_tokens, Some(3));
        assert_eq!(usage.total_tokens, Some(17));
        
        // Check token details
        assert!(usage.input_token_details.is_some());
        let token_details = usage.input_token_details.unwrap();
        assert_eq!(token_details.text_tokens, Some(0));
        assert_eq!(token_details.audio_tokens, Some(14));
    }

    #[test]
    fn test_simple_transcription_response() {
        let json = r#"{
            "text": "Simple transcription without usage data"
        }"#;

        let response: TranscriptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text, "Simple transcription without usage data");
        assert!(response.usage.is_none());
    }
    
    #[test]
    fn test_partial_usage_response() {
        // Test with partial usage data (missing some fields)
        let json = r#"{
            "text": "Transcription with partial usage",
            "usage": {
                "total_tokens": 25
            }
        }"#;

        let response: TranscriptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.text, "Transcription with partial usage");
        assert!(response.usage.is_some());
        
        let usage = response.usage.unwrap();
        assert_eq!(usage.total_tokens, Some(25));
        assert_eq!(usage.input_tokens, None);
        assert_eq!(usage.output_tokens, None);
        assert_eq!(usage.usage_type, None);
        assert_eq!(usage.input_token_details, None);
    }
}
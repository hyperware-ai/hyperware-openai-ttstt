#[cfg(test)]
mod tests {
    use crate::types::{AudioFormat, SpeechRequestJson, TtsModel, Voice, SpeechRequest};

    #[test]
    fn test_model_serialization() {
        assert_eq!(TtsModel::Tts1.as_str(), "tts-1");
        assert_eq!(TtsModel::Tts1Hd.as_str(), "tts-1-hd");
        assert_eq!(TtsModel::Gpt4oMiniTts.as_str(), "gpt-4o-mini-tts");
    }

    #[test]
    fn test_voice_serialization() {
        assert_eq!(Voice::Alloy.as_str(), "alloy");
        assert_eq!(Voice::Ash.as_str(), "ash");
        assert_eq!(Voice::Ballad.as_str(), "ballad");
        assert_eq!(Voice::Coral.as_str(), "coral");
        assert_eq!(Voice::Echo.as_str(), "echo");
        assert_eq!(Voice::Fable.as_str(), "fable");
        assert_eq!(Voice::Onyx.as_str(), "onyx");
        assert_eq!(Voice::Nova.as_str(), "nova");
        assert_eq!(Voice::Sage.as_str(), "sage");
        assert_eq!(Voice::Shimmer.as_str(), "shimmer");
        assert_eq!(Voice::Verse.as_str(), "verse");
    }

    #[test]
    fn test_audio_format_serialization() {
        assert_eq!(AudioFormat::Mp3.as_str(), "mp3");
        assert_eq!(AudioFormat::Opus.as_str(), "opus");
        assert_eq!(AudioFormat::Aac.as_str(), "aac");
        assert_eq!(AudioFormat::Flac.as_str(), "flac");
        assert_eq!(AudioFormat::Wav.as_str(), "wav");
        assert_eq!(AudioFormat::Pcm.as_str(), "pcm");
    }

    #[test]
    fn test_speech_request_json_serialization() {
        let request = SpeechRequest {
            input: "Hello, world!".to_string(),
            model: TtsModel::Gpt4oMiniTts,
            voice: Voice::Alloy,
            instructions: Some("Speak with enthusiasm".to_string()),
            response_format: Some(AudioFormat::Mp3),
            speed: Some(1.5),
        };

        let json_request = SpeechRequestJson::from(request);
        let json = serde_json::to_string(&json_request).unwrap();

        assert!(json.contains("\"input\":\"Hello, world!\""));
        assert!(json.contains("\"model\":\"gpt-4o-mini-tts\""));
        assert!(json.contains("\"voice\":\"alloy\""));
        assert!(json.contains("\"instructions\":\"Speak with enthusiasm\""));
        assert!(json.contains("\"response_format\":\"mp3\""));
        assert!(json.contains("\"speed\":1.5"));
    }

    #[test]
    fn test_speech_request_json_minimal() {
        let request = SpeechRequest {
            input: "Test".to_string(),
            model: TtsModel::Tts1,
            voice: Voice::Nova,
            instructions: None,
            response_format: None,
            speed: None,
        };

        let json_request = SpeechRequestJson::from(request);
        let json = serde_json::to_string(&json_request).unwrap();

        assert!(json.contains("\"input\":\"Test\""));
        assert!(json.contains("\"model\":\"tts-1\""));
        assert!(json.contains("\"voice\":\"nova\""));
        // Optional fields should not be present
        assert!(!json.contains("\"instructions\""));
        assert!(!json.contains("\"response_format\""));
        assert!(!json.contains("\"speed\""));
    }

    #[test]
    fn test_audio_format_default() {
        let default_format = AudioFormat::default();
        assert!(matches!(default_format, AudioFormat::Mp3));
    }
}
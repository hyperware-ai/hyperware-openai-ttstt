# Hyperware OpenAI STT and TTS Libraries

This workspace contains two Rust libraries that provide Hyperware processes with access to OpenAI's Speech-to-Text (STT) and Text-to-Speech (TTS) APIs.

## Libraries

### hyperware-openai-stt
Speech-to-Text transcription library using OpenAI's transcription API.

**Features:**
- Support for multiple models: gpt-4o-transcribe, gpt-4o-mini-transcribe, whisper-1
- Multiple audio formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, webm
- Multiple response formats: json, text, srt, verbose_json, vtt
- Optional language detection and prompt guidance
- Builder pattern for easy request construction

**Usage:**
```rust
use hyperware_openai_stt::{TranscriptionClient, Model};

let client = TranscriptionClient::new(api_key);
let response = client.transcribe()
    .file(audio_bytes, "audio.mp3")
    .model(Model::Gpt4oTranscribe)
    .language("en")
    .execute()
    .await?;

println!("Transcription: {}", response.text);
```

### hyperware-openai-tts
Text-to-Speech synthesis library using OpenAI's speech API.

**Features:**
- Support for multiple models: tts-1, tts-1-hd, gpt-4o-mini-tts
- 11 different voice options
- Multiple audio formats: mp3, opus, aac, flac, wav, pcm
- Adjustable speed (0.25x to 4.0x)
- Optional voice instructions (for gpt-4o-mini-tts)
- Builder pattern for easy request construction

**Usage:**
```rust
use hyperware_openai_tts::{SpeechClient, TtsModel, Voice, AudioFormat};

let client = SpeechClient::new(api_key);
let audio = client.synthesize()
    .input("Hello, world!")
    .model(TtsModel::Gpt4oMiniTts)
    .voice(Voice::Alloy)
    .response_format(AudioFormat::Mp3)
    .speed(1.0)
    .execute()
    .await?;

// audio.audio_data contains the generated audio bytes
```

## Integration with Hyperware

Both libraries use the Hyperware HTTP client for all API communications. The `send_request_await_response` function is provided by the Hyperware runtime and should be implemented by the host environment.

## Building

```bash
cargo build --all
```

## Testing

```bash
cargo test --all
```

## License

See LICENSE file for details.
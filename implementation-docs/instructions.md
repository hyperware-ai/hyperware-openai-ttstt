# 250923

## OpenAI STT and TTS crates

*OpenAI STT api*

post
 
https://api.openai.com/v1/audio/transcriptions
Transcribes audio into the input language.

Request body
file
file

Required
The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.

model
string

Required
ID of the model to use. The options are gpt-4o-transcribe, gpt-4o-mini-transcribe, and whisper-1 (which is powered by our open source Whisper V2 model).

chunking_strategy
"auto" or object

Optional
Controls how the audio is cut into chunks. When set to "auto", the server first normalizes loudness and then uses voice activity detection (VAD) to choose boundaries. server_vad object can be provided to tweak VAD detection parameters manually. If unset, the audio is transcribed as a single block.


Show possible types
include[]
array

Optional
Additional information to include in the transcription response. logprobs will return the log probabilities of the tokens in the response to understand the model's confidence in the transcription. logprobs only works with response_format set to json and only with the models gpt-4o-transcribe and gpt-4o-mini-transcribe.

language
string

Optional
The language of the input audio. Supplying the input language in ISO-639-1 (e.g. en) format will improve accuracy and latency.

prompt
string

Optional
An optional text to guide the model's style or continue a previous audio segment. The prompt should match the audio language.

response_format
string

Optional
Defaults to json
The format of the output, in one of these options: json, text, srt, verbose_json, or vtt. For gpt-4o-transcribe and gpt-4o-mini-transcribe, the only supported format is json.

stream
boolean

Optional
Defaults to false
If set to true, the model response data will be streamed to the client as it is generated using server-sent events. See the Streaming section of the Speech-to-Text guide for more information.

Note: Streaming is not supported for the whisper-1 model and will be ignored.

temperature
number

Optional
Defaults to 0
The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.

timestamp_granularities[]
array

Optional
Defaults to segment
The timestamp granularities to populate for this transcription. response_format must be set verbose_json to use timestamp granularities. Either or both of these options are supported: word, or segment. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency.

Returns
The transcription object, a verbose transcription object or a stream of transcript events.

```
curl https://api.openai.com/v1/audio/transcriptions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: multipart/form-data" \
  -F file="@/path/to/file/audio.mp3" \
  -F model="gpt-4o-transcribe"
```

```
{
  "text": "Imagine the wildest idea that you've ever had, and you're curious about how it might scale to something that's a 100, a 1,000 times bigger. This is a place where you can get to do that.",
  "usage": {
    "type": "tokens",
    "input_tokens": 14,
    "input_token_details": {
      "text_tokens": 0,
      "audio_tokens": 14
    },
    "output_tokens": 45,
    "total_tokens": 59
  }
}

```

*OpenAI TTS api*

post
 
https://api.openai.com/v1/audio/speech
Generates audio from the input text.

Request body
input
string

Required
The text to generate audio for. The maximum length is 4096 characters.

model
string

Required
One of the available TTS models: tts-1, tts-1-hd or gpt-4o-mini-tts.

voice
string

Required
The voice to use when generating the audio. Supported voices are alloy, ash, ballad, coral, echo, fable, onyx, nova, sage, shimmer, and verse. Previews of the voices are available in the Text to speech guide.

instructions
string

Optional
Control the voice of your generated audio with additional instructions. Does not work with tts-1 or tts-1-hd.

response_format
string

Optional
Defaults to mp3
The format to audio in. Supported formats are mp3, opus, aac, flac, wav, and pcm.

speed
number

Optional
Defaults to 1
The speed of the generated audio. Select a value from 0.25 to 4.0. 1.0 is the default.

stream_format
string

Optional
Defaults to audio
The format to stream the audio in. Supported formats are sse and audio. sse is not supported for tts-1 or tts-1-hd.

Returns
The audio file content or a stream of audio events.

```
curl https://api.openai.com/v1/audio/speech \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4o-mini-tts",
    "input": "The quick brown fox jumped over the lazy dog.",
    "voice": "alloy"
  }' \
  --output speech.mp3
```

*Hyperware HTTP Client API*

```rust
use http::Method;

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum HttpClientError {
    // HTTP errors
    #[error("request could not be deserialized to valid HttpClientRequest")]
    MalformedRequest,
    #[error("http method not supported: {method}")]
    BadMethod { method: String },
    #[error("url could not be parsed: {url}")]
    BadUrl { url: String },
    #[error("http version not supported: {version}")]
    BadVersion { version: String },
    #[error("client failed to build request: {0}")]
    BuildRequestFailed(String),
    #[error("client failed to execute request: {0}")]
    ExecuteRequestFailed(String),

    // WebSocket errors
    #[error("could not open connection to {url}")]
    WsOpenFailed { url: String },
    #[error("sent WebSocket push to unknown channel {channel_id}")]
    WsPushUnknownChannel { channel_id: u32 },
    #[error("WebSocket push failed because message had no blob attached")]
    WsPushNoBlob,
    #[error("WebSocket push failed because message type was Text, but blob was not a valid UTF-8 string")]
    WsPushBadText,
    #[error("failed to close connection {channel_id} because it was not open")]
    WsCloseFailed { channel_id: u32 },
}

/// Make an HTTP request using http-client and await its response.
pub async fn send_request_await_response(
    method: Method,
    url: url::Url,
    headers: Option<HashMap<String, String>>,
    timeout: u64,
    body: Vec<u8>,
) -> std::result::Result<http::Response<Vec<u8>>, HttpClientError>
```

*Prompt*

Create an implementation plan for two Rust libraries for Hyperware processes to access the OpenAI STT and TTS APIs, respectively

Above are docs for the OpenAI STT and TTS APIs and hyperware HTTP client methods. Use the Hyperware HTTP client methods to talk to the OpenAI API. Do not use ever use streaming mode

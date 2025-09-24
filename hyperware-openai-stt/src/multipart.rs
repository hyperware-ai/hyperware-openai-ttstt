use rand::Rng;

pub struct MultipartFormData {
    boundary: String,
    parts: Vec<Part>,
}

struct Part {
    name: String,
    filename: Option<String>,
    content_type: Option<String>,
    data: Vec<u8>,
}

impl MultipartFormData {
    pub fn new() -> Self {
        let boundary = generate_boundary();
        Self {
            boundary,
            parts: Vec::new(),
        }
    }

    pub fn add_text(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.parts.push(Part {
            name: name.into(),
            filename: None,
            content_type: None,
            data: value.into().into_bytes(),
        });
    }

    pub fn add_file(
        &mut self,
        name: impl Into<String>,
        filename: impl Into<String>,
        content_type: impl Into<String>,
        data: Vec<u8>,
    ) {
        self.parts.push(Part {
            name: name.into(),
            filename: Some(filename.into()),
            content_type: Some(content_type.into()),
            data,
        });
    }

    pub fn build(self) -> (Vec<u8>, String) {
        let mut body = Vec::new();
        let boundary_bytes = format!("--{}", self.boundary).into_bytes();
        let crlf = b"\r\n";

        for part in self.parts {
            body.extend_from_slice(&boundary_bytes);
            body.extend_from_slice(crlf);

            body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
            body.extend_from_slice(part.name.as_bytes());
            body.extend_from_slice(b"\"");

            if let Some(filename) = part.filename {
                body.extend_from_slice(b"; filename=\"");
                body.extend_from_slice(filename.as_bytes());
                body.extend_from_slice(b"\"");
            }
            body.extend_from_slice(crlf);

            if let Some(content_type) = part.content_type {
                body.extend_from_slice(b"Content-Type: ");
                body.extend_from_slice(content_type.as_bytes());
                body.extend_from_slice(crlf);
            }

            body.extend_from_slice(crlf);
            body.extend_from_slice(&part.data);
            body.extend_from_slice(crlf);
        }

        body.extend_from_slice(format!("--{}--", self.boundary).as_bytes());
        body.extend_from_slice(crlf);

        let content_type = format!("multipart/form-data; boundary={}", self.boundary);
        (body, content_type)
    }

    pub fn boundary(&self) -> &str {
        &self.boundary
    }
}

fn generate_boundary() -> String {
    let mut rng = rand::thread_rng();
    let chars: String = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            match idx {
                0..=9 => (b'0' + idx) as char,
                10..=35 => (b'A' + idx - 10) as char,
                36..=61 => (b'a' + idx - 36) as char,
                _ => unreachable!(),
            }
        })
        .collect();
    format!("----WebKitFormBoundary{}", chars)
}

pub fn get_content_type_for_extension(filename: &str) -> &'static str {
    let extension = filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "flac" => "audio/flac",
        "mp3" => "audio/mpeg",
        "mp4" => "audio/mp4",
        "mpeg" | "mpga" => "audio/mpeg",
        "m4a" => "audio/m4a",
        "ogg" => "audio/ogg",
        "wav" => "audio/wav",
        "webm" => "audio/webm",
        _ => "application/octet-stream",
    }
}
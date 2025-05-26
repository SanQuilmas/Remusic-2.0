use base64::{Engine, engine::general_purpose};

pub fn decode_blob(blob: &str) -> Vec<u8> {
    general_purpose::STANDARD
        .decode(blob)
        .expect("Failed to decode base64 blob")
}

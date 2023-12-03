use base64;

pub fn decode_from_base64(data: &str) -> Vec<u8> {
    base64::decode(data).expect("Failed to decode from Base64")
}

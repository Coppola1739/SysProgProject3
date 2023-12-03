use base64;
pub fn encode_to_base64(data: &str) -> String {
    base64::encode(data)
}

use ring::hmac;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn get_hmac_signed_message(secret: &str, message: String) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&key, message.as_bytes());
    return hex::encode(tag.as_ref());
}

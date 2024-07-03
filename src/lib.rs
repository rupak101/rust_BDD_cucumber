use hmac::{ Hmac, Mac };
use sha2::Sha512;
use std::time::{ SystemTime, UNIX_EPOCH };

// Alias for HMAC-SHA512
type HmacSha512 = Hmac<Sha512>;

// Function to generate the API signature
pub fn generate_signature(path: &str, nonce: u64, post_data: &str, secret: &str) -> String {
    let decoded_secret = base64::decode(secret).expect("Failed to decode secret");
    let mut mac = HmacSha512::new_from_slice(&decoded_secret).expect(
        "HMAC can take key of any size"
    );
    let message = format!("{}{}", path, nonce.to_string() + post_data);
    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();
    base64::encode(result)
}

// Function to get the current nonce
pub fn get_nonce() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_epoch.as_millis() as u64
}

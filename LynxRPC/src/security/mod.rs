use sha2::{Sha256, Digest};
use hex;

#[derive(Clone)]
pub struct AuthValidator {
    secret: String,
}

impl AuthValidator {
    pub fn new() -> Self {
        Self {
            secret: std::env::var("LYNX_SECRET").unwrap_or_default(),
        }
    }

    pub fn validate_token(&self, token: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(self.secret.as_bytes());
        let result = hex::encode(hasher.finalize());
        token == result
    }

    pub fn generate_token(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.secret.as_bytes());
        hex::encode(hasher.finalize())
    }
}
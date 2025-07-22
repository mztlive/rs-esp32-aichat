pub mod client;
pub mod pcm_client;
pub mod types;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub base_url: String,
    pub fingerprint: String,
    pub timeout_secs: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000/api".to_string(),
            fingerprint: "esp32-device".to_string(),
            timeout_secs: 300,
        }
    }
}

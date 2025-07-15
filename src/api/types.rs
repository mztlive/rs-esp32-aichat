use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub data: T,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHistory {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHistoryItem {
    pub session_id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug)]
pub enum ApiError {
    Http(esp_idf_svc::sys::EspError),
    Json(serde_json::Error),
    Utf8(std::string::FromUtf8Error),
    Api { status: u16, message: String },
    SessionNotFound,
    InvalidFingerprint,
    Timeout,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Http(e) => write!(f, "HTTP request failed: {}", e),
            ApiError::Json(e) => write!(f, "JSON parsing failed: {}", e),
            ApiError::Utf8(e) => write!(f, "UTF-8 conversion failed: {}", e),
            ApiError::Api { status, message } => write!(f, "API error {}: {}", status, message),
            ApiError::SessionNotFound => write!(f, "Session not found"),
            ApiError::InvalidFingerprint => write!(f, "Invalid fingerprint"),
            ApiError::Timeout => write!(f, "Timeout"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<esp_idf_svc::sys::EspError> for ApiError {
    fn from(error: esp_idf_svc::sys::EspError) -> Self {
        ApiError::Http(error)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::Json(error)
    }
}

impl From<std::string::FromUtf8Error> for ApiError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ApiError::Utf8(error)
    }
}
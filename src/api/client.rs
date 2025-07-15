use super::{types::*, ApiConfig};
use anyhow::Result;
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    io::Write,
    utils::io,
};
use esp_idf_svc::http::client::EspHttpConnection;
use log::{error, info};
use std::time::Duration;

pub struct ApiClient {
    config: ApiConfig,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Result<Self> {
        Ok(Self { config })
    }

    fn build_headers(&self) -> Vec<(&str, &str)> {
        vec![
            ("X-Fingerprint", &self.config.fingerprint),
            ("Content-Type", "application/json"),
        ]
    }

    fn create_client(&self) -> Result<HttpClient<EspHttpConnection>> {
        let http_config = esp_idf_svc::http::client::Configuration {
            timeout: Some(Duration::from_secs(self.config.timeout_secs)),
            ..Default::default()
        };

        let connection = EspHttpConnection::new(&http_config)?;
        Ok(HttpClient::wrap(connection))
    }

    fn read_response_body<R>(mut reader: R) -> Result<String>
    where
        R: embedded_svc::io::Read,
    {
        let mut buf = [0u8; 1024];
        let bytes_read = io::try_read_full(&mut reader, &mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read response: {:?}", e.0))?;

        match std::str::from_utf8(&buf[0..bytes_read]) {
            Ok(response_text) => Ok(response_text.to_string()),
            Err(e) => {
                error!("Error decoding response body: {}", e);
                Err(anyhow::anyhow!("UTF-8 decoding error: {}", e))
            }
        }
    }

    fn handle_api_error(status: u16, response_text: &str) -> Result<()> {
        let error_response: ApiResponse<serde_json::Value> =
            serde_json::from_str(response_text)?;
        Err(anyhow::anyhow!(
            "API error {}: {}",
            status,
            error_response
                .message
                .unwrap_or_else(|| "Unknown error".to_string())
        ))
    }

    fn execute_get_request(&self, url: &str) -> Result<(u16, String)> {
        let mut client = self.create_client()?;
        let headers = self.build_headers();
        info!("-> GET {}", url);
        let request = client.request(Method::Get, url, &headers)?;
        let mut response = request.submit()?;

        let status = response.status();
        info!("<- {}", status);
        let response_text = Self::read_response_body(response)?;
        
        Ok((status, response_text))
    }

    fn execute_post_request(&self, url: &str, body: &str) -> Result<(u16, String)> {
        let mut client = self.create_client()?;
        let headers = self.build_headers();

        info!("-> POST {}", url);
        let mut request = client.request(Method::Post, url, &headers)?;
        request.write_all(body.as_bytes())?;
        request.flush()?;

        let mut response = request.submit()?;
        let status = response.status();
        info!("<- {}", status);
        let response_text = Self::read_response_body(response)?;
        
        Ok((status, response_text))
    }

    pub fn create_session(&self, model: Option<&str>) -> Result<String> {
        let mut url = format!("{}/chat/create", self.config.base_url);
        if let Some(model) = model {
            url.push_str(&format!("?model={}", model));
        }

        let (status, response_text) = self.execute_get_request(&url)?;

        if status == 200 {
            let api_response: ApiResponse<SessionInfo> = serde_json::from_str(&response_text)?;
            Ok(api_response.data.session_id)
        } else {
            Self::handle_api_error(status, &response_text)?;
            unreachable!()
        }
    }

    pub fn send_message(
        &self,
        session_id: &str,
        message: &str,
        files: Option<Vec<String>>,
    ) -> Result<()> {
        let url = format!("{}/chat/message/{}", self.config.base_url, session_id);
        let request_body = MessageRequest {
            message: message.to_string(),
            files,
        };
        let body_json = serde_json::to_string(&request_body)?;

        let (status, response_text) = self.execute_post_request(&url, &body_json)?;

        if status == 200 {
            Ok(())
        } else {
            Self::handle_api_error(status, &response_text)?;
            unreachable!()
        }
    }

    pub fn prompt_sync(
        &self,
        session_id: &str,
        message: &str,
        files: Option<Vec<String>>,
    ) -> Result<String> {
        let url = format!("{}/chat/prompt/{}", self.config.base_url, session_id);
        let request_body = MessageRequest {
            message: message.to_string(),
            files,
        };
        let body_json = serde_json::to_string(&request_body)?;

        let (status, response_text) = self.execute_post_request(&url, &body_json)?;

        if status == 200 {
            let api_response: ApiResponse<String> = serde_json::from_str(&response_text)?;
            Ok(api_response.data)
        } else {
            Self::handle_api_error(status, &response_text)?;
            unreachable!()
        }
    }
}

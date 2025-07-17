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

/// HTTP API客户端，用于与聊天服务进行通信
pub struct ApiClient {
    config: ApiConfig,
}

impl ApiClient {
    /// 创建新的API客户端实例
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }

    /// 构建HTTP请求头
    fn build_headers(&self) -> Vec<(&str, &str)> {
        vec![
            ("X-Fingerprint", &self.config.fingerprint),
            ("Content-Type", "application/json"),
        ]
    }

    /// 创建HTTP客户端连接
    fn create_client(&self) -> Result<HttpClient<EspHttpConnection>> {
        let http_config = esp_idf_svc::http::client::Configuration {
            timeout: Some(Duration::from_secs(self.config.timeout_secs)),
            ..Default::default()
        };

        let connection = EspHttpConnection::new(&http_config)?;
        Ok(HttpClient::wrap(connection))
    }

    /// 读取HTTP响应体内容
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

    /// 创建API错误信息
    fn create_api_error(status: u16, response_text: &str) -> anyhow::Error {
        match serde_json::from_str::<ApiResponse<serde_json::Value>>(response_text) {
            Ok(error_response) => anyhow::anyhow!(
                "API error {}: {}",
                status,
                error_response
                    .message
                    .unwrap_or_else(|| "Unknown error".to_string())
            ),
            Err(_) => anyhow::anyhow!("API error {}: {}", status, response_text),
        }
    }

    /// 处理API响应，返回反序列化的数据
    fn handle_response<T>(&self, status: u16, response_text: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        if status == 200 {
            let api_response: ApiResponse<T> = serde_json::from_str(response_text)?;
            Ok(api_response.data)
        } else {
            Err(Self::create_api_error(status, response_text))
        }
    }

    /// 处理无返回数据的API响应
    fn handle_response_unit(&self, status: u16, response_text: &str) -> Result<()> {
        if status == 200 {
            Ok(())
        } else {
            Err(Self::create_api_error(status, response_text))
        }
    }

    /// 执行GET请求
    fn execute_get_request(&self, url: &str) -> Result<(u16, String)> {
        let mut client = self.create_client()?;
        let headers = self.build_headers();
        info!("-> GET {}", url);
        let request = client.request(Method::Get, url, &headers)?;
        let response = request.submit()?;

        let status = response.status();
        info!("<- {}", status);
        let response_text = Self::read_response_body(response)?;

        Ok((status, response_text))
    }

    /// 执行POST请求
    fn execute_post_request(&self, url: &str, body: &str) -> Result<(u16, String)> {
        let mut client = self.create_client()?;
        let headers = self.build_headers();

        info!("-> POST {}", url);
        let mut request = client.request(Method::Post, url, &headers)?;
        request.write_all(body.as_bytes())?;
        request.flush()?;

        let response = request.submit()?;
        let status = response.status();
        info!("<- {}", status);
        let response_text = Self::read_response_body(response)?;

        Ok((status, response_text))
    }

    /// 创建聊天会话
    ///
    /// # 参数
    /// - `model`: 可选的模型名称
    ///
    /// # 返回
    /// 会话ID字符串
    pub fn create_session(&self, model: Option<&str>) -> Result<String> {
        let mut url = format!("{}/chat/create", self.config.base_url);
        if let Some(model) = model {
            url.push_str(&format!("?model={}", model));
        }

        let (status, response_text) = self.execute_get_request(&url)?;
        let session_info: SessionInfo = self.handle_response(status, &response_text)?;
        Ok(session_info.session_id)
    }

    /// 发送消息到聊天会话
    ///
    /// # 参数
    /// - `session_id`: 会话ID
    /// - `message`: 消息内容
    /// - `files`: 可选的文件列表
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
        self.handle_response_unit(status, &response_text)
    }

    /// 同步发送提示并获取响应
    ///
    /// # 参数
    /// - `session_id`: 会话ID
    /// - `message`: 提示消息
    /// - `files`: 可选的文件列表
    ///
    /// # 返回
    /// 聊天响应字符串
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
        self.handle_response(status, &response_text)
    }
}

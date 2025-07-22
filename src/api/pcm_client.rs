use anyhow::Result;
use embedded_svc::http::{client::Client as HttpClient, Method};
use embedded_svc::io::Write as EmbeddedWrite;
use esp_idf_svc::http::client::EspHttpConnection;
use log::{error, info};
use std::time::Duration;

/// PCM音频数据上传配置
pub struct PcmClientConfig {
    /// 服务器基础URL
    pub base_url: String,
    /// 会话ID
    pub session_id: String,
    /// 请求超时时间（秒）
    pub timeout_secs: u64,
}

impl Default for PcmClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://192.168.1.100:8080".to_string(), // 替换为实际服务器地址
            session_id: "esp32_device_001".to_string(),
            timeout_secs: 30,
        }
    }
}

/// PCM音频数据HTTP客户端
pub struct PcmClient {
    config: PcmClientConfig,
}

impl PcmClient {
    /// 创建新的PCM客户端实例
    pub fn new(config: PcmClientConfig) -> Self {
        Self { config }
    }

    /// 创建HTTP客户端连接
    fn create_client(&self) -> Result<HttpClient<EspHttpConnection>> {
        let http_config = esp_idf_svc::http::client::Configuration {
            timeout: Some(Duration::from_secs(self.config.timeout_secs)),
            buffer_size: Some(4096), // 增加缓冲区大小以支持音频流
            ..Default::default()
        };

        let connection = EspHttpConnection::new(&http_config)?;
        Ok(HttpClient::wrap(connection))
    }

    /// 发送PCM音频数据块
    ///
    /// # 参数
    /// - `pcm_data`: PCM音频数据（16位，16kHz，单声道）
    ///
    /// # 返回
    /// 成功返回Ok(())，失败返回错误
    pub fn send_pcm_chunk(&self, pcm_data: &[u8]) -> Result<()> {
        let url = format!("{}/pcm/{}", self.config.base_url, self.config.session_id);

        info!("Sending PCM chunk: {} bytes to {}", pcm_data.len(), url);

        let mut client = self.create_client()?;

        // 设置请求头
        let headers = [
            ("Content-Type", "application/octet-stream"),
            ("Content-Length", &pcm_data.len().to_string()),
        ];

        // 创建POST请求
        let mut request = client.request(Method::Post, &url, &headers)?;

        // 发送PCM数据
        request
            .write_all(pcm_data)
            .map_err(|e| anyhow::anyhow!("Failed to write PCM data: {:?}", e))?;
        request
            .flush()
            .map_err(|e| anyhow::anyhow!("Failed to flush request: {:?}", e))?;

        // 提交请求并获取响应
        let response = request.submit()?;
        let status = response.status();

        if status == 200 {
            info!("PCM chunk sent successfully");
            Ok(())
        } else {
            error!("Failed to send PCM chunk: HTTP {}", status);
            Err(anyhow::anyhow!("HTTP error: {}", status))
        }
    }

    /// 发送PCM音频流
    ///
    /// # 参数
    /// - `pcm_stream`: PCM音频数据迭代器
    /// - `chunk_size`: 每次发送的数据块大小（字节）
    ///
    /// # 返回
    /// 成功返回发送的总字节数，失败返回错误
    pub fn send_pcm_stream<I>(&self, pcm_stream: I, chunk_size: usize) -> Result<usize>
    where
        I: Iterator<Item = Vec<u8>>,
    {
        let mut total_bytes = 0;
        let mut chunk_buffer = Vec::with_capacity(chunk_size);

        for data in pcm_stream {
            chunk_buffer.extend_from_slice(&data);

            // 当缓冲区达到指定大小时发送
            while chunk_buffer.len() >= chunk_size {
                let chunk: Vec<u8> = chunk_buffer.drain(..chunk_size).collect();
                self.send_pcm_chunk(&chunk)?;
                total_bytes += chunk.len();

                // 添加小延迟防止过快发送
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        // 发送剩余数据
        if !chunk_buffer.is_empty() {
            self.send_pcm_chunk(&chunk_buffer)?;
            total_bytes += chunk_buffer.len();
        }

        info!("PCM stream sent: {} total bytes", total_bytes);
        Ok(total_bytes)
    }

    /// 更新会话ID
    pub fn set_session_id(&mut self, session_id: String) {
        self.config.session_id = session_id;
    }

    /// 获取当前会话ID
    pub fn session_id(&self) -> &str {
        &self.config.session_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcm_client_creation() {
        let config = PcmClientConfig::default();
        let client = PcmClient::new(config);
        assert_eq!(client.session_id(), "esp32_device_001");
    }
}

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConfig {
    pub ssid: String,
    pub password: String,
    pub auto_connect: bool,
}

impl WifiConfig {
    pub fn new(ssid: &str, password: &str) -> Self {
        Self {
            ssid: ssid.to_string(),
            password: password.to_string(),
            auto_connect: true,
        }
    }

    pub fn from_env() -> Result<Self> {
        let ssid = std::env::var("WIFI_SSID")
            .map_err(|_| anyhow::anyhow!("WIFI_SSID environment variable not set"))?;
        let password = std::env::var("WIFI_PASS")
            .map_err(|_| anyhow::anyhow!("WIFI_PASS environment variable not set"))?;
        
        Ok(Self::new(&ssid, &password))
    }

    pub fn validate(&self) -> Result<()> {
        if self.ssid.is_empty() {
            return Err(anyhow::anyhow!("SSID cannot be empty"));
        }
        if self.password.len() < 8 {
            return Err(anyhow::anyhow!("Password must be at least 8 characters"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WifiCredentials {
    pub ssid: String,
    pub password: String,
}

impl WifiCredentials {
    pub fn new(ssid: &str, password: &str) -> Self {
        Self {
            ssid: ssid.to_string(),
            password: password.to_string(),
        }
    }
}

impl From<WifiConfig> for WifiCredentials {
    fn from(config: WifiConfig) -> Self {
        Self {
            ssid: config.ssid,
            password: config.password,
        }
    }
}
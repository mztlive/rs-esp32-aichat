pub mod config;

use anyhow::Result;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::info;

pub use config::{WifiConfig, WifiCredentials};

pub struct WifiManager {
    wifi: BlockingWifi<EspWifi<'static>>,
}

impl WifiManager {
    pub fn new(
        modem: Modem,
        sys_loop: EspSystemEventLoop,
        nvs: Option<EspDefaultNvsPartition>,
    ) -> Result<Self> {
        let wifi = BlockingWifi::wrap(EspWifi::new(modem, sys_loop.clone(), nvs)?, sys_loop)?;

        Ok(Self { wifi })
    }

    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<()> {
        let credentials = WifiCredentials::new(ssid, password);
        self.connect_with_credentials(&credentials)
    }

    pub fn connect_with_credentials(&mut self, credentials: &WifiCredentials) -> Result<()> {
        let wifi_configuration = Configuration::Client(ClientConfiguration {
            ssid: credentials
                .ssid
                .as_str()
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid SSID"))?,
            bssid: None,
            auth_method: AuthMethod::WPA2Personal,
            password: credentials
                .password
                .as_str()
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid password"))?,
            channel: None,
            ..Default::default()
        });

        self.wifi.set_configuration(&wifi_configuration)?;
        self.wifi.start()?;
        info!("WiFi started");

        self.wifi.connect()?;
        info!("WiFi connected");

        self.wifi.wait_netif_up()?;
        info!("WiFi netif up");

        Ok(())
    }

    pub fn connect_with_config(&mut self, config: &WifiConfig) -> Result<()> {
        config.validate()?;
        let credentials: WifiCredentials = config.clone().into();
        self.connect_with_credentials(&credentials)
    }

    pub fn disconnect(&mut self) -> Result<()> {
        self.wifi.disconnect()?;
        info!("WiFi disconnected");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.wifi.is_connected().unwrap_or(false)
    }

    pub fn get_ip_info(&self) -> Result<embedded_svc::ipv4::Ipv4Addr> {
        let ip_info = self.wifi.wifi().sta_netif().get_ip_info()?;
        Ok(ip_info.ip)
    }

    pub fn scan_networks(&mut self) -> Result<Vec<embedded_svc::wifi::AccessPointInfo>> {
        self.wifi
            .scan()
            .map_err(|e| anyhow::anyhow!("WiFi scan failed: {}", e))
    }
}

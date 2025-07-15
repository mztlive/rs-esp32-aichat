use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::info;

use crate::peripherals::wifi::{WifiConfig, WifiManager};

#[derive(Debug, Clone)]
pub enum WifiCommand {
    Connect(WifiConfig),
    Disconnect,
    GetStatus,
    Scan,
}

#[derive(Debug, Clone)]
pub enum WifiEvent {
    Connected(String), // IP address
    Disconnected,
    ConnectionFailed(String), // Error message
    StatusUpdate(WifiStatus),
    ScanResult(Vec<String>), // Network names
}

#[derive(Debug, Clone)]
pub enum WifiStatus {
    Connected,
    Disconnected,
    Connecting,
    Scanning,
    Error(String),
}

pub struct WifiActor {
    wifi_manager: WifiManager,
    command_receiver: Receiver<WifiCommand>,
    event_sender: Sender<WifiEvent>,
    current_status: WifiStatus,
}

impl WifiActor {
    pub fn new(
        modem: Modem,
        sys_loop: EspSystemEventLoop,
        nvs: Option<EspDefaultNvsPartition>,
        command_receiver: Receiver<WifiCommand>,
        event_sender: Sender<WifiEvent>,
    ) -> Result<Self> {
        let wifi_manager = WifiManager::new(modem, sys_loop, nvs)?;

        Ok(Self {
            wifi_manager,
            command_receiver,
            event_sender,
            current_status: WifiStatus::Disconnected,
        })
    }

    pub fn run(&mut self) {
        info!("WiFi actor started");

        loop {
            // Check for commands with timeout
            match self
                .command_receiver
                .recv_timeout(Duration::from_millis(1000))
            {
                Ok(command) => {
                    if let Err(e) = self.handle_command(command) {
                        let error_msg = format!("WiFi command failed: {}", e);
                        self.current_status = WifiStatus::Error(error_msg.clone());
                        let _ = self
                            .event_sender
                            .send(WifiEvent::StatusUpdate(WifiStatus::Error(error_msg)));
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Periodic status check
                    self.check_connection_status();
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    info!("WiFi actor command channel disconnected, shutting down");
                    break;
                }
            }
        }
    }

    fn handle_command(&mut self, command: WifiCommand) -> Result<()> {
        match command {
            WifiCommand::Connect(config) => {
                info!("Connecting to WiFi: {}", config.ssid);
                self.current_status = WifiStatus::Connecting;
                let _ = self
                    .event_sender
                    .send(WifiEvent::StatusUpdate(WifiStatus::Connecting));

                match self.wifi_manager.connect_with_config(&config) {
                    Ok(_) => {
                        info!("WiFi connected successfully");
                        self.current_status = WifiStatus::Connected;

                        if let Ok(ip) = self.wifi_manager.get_ip_info() {
                            let ip_str = format!("{}", ip);
                            let _ = self.event_sender.send(WifiEvent::Connected(ip_str));
                        } else {
                            let _ = self
                                .event_sender
                                .send(WifiEvent::Connected("Unknown IP".to_string()));
                        }

                        let _ = self
                            .event_sender
                            .send(WifiEvent::StatusUpdate(WifiStatus::Connected));
                    }
                    Err(e) => {
                        let error_msg = format!("WiFi connection failed: {}", e);
                        info!("{}", error_msg);
                        self.current_status = WifiStatus::Error(error_msg.clone());
                        let _ = self
                            .event_sender
                            .send(WifiEvent::ConnectionFailed(error_msg));
                        let _ = self
                            .event_sender
                            .send(WifiEvent::StatusUpdate(WifiStatus::Disconnected));
                    }
                }
            }
            WifiCommand::Disconnect => {
                info!("Disconnecting WiFi");
                match self.wifi_manager.disconnect() {
                    Ok(_) => {
                        self.current_status = WifiStatus::Disconnected;
                        let _ = self.event_sender.send(WifiEvent::Disconnected);
                        let _ = self
                            .event_sender
                            .send(WifiEvent::StatusUpdate(WifiStatus::Disconnected));
                    }
                    Err(e) => {
                        let error_msg = format!("WiFi disconnect failed: {}", e);
                        self.current_status = WifiStatus::Error(error_msg.clone());
                        let _ = self
                            .event_sender
                            .send(WifiEvent::StatusUpdate(WifiStatus::Error(error_msg)));
                    }
                }
            }
            WifiCommand::GetStatus => {
                let _ = self
                    .event_sender
                    .send(WifiEvent::StatusUpdate(self.current_status.clone()));
            }
            WifiCommand::Scan => {
                info!("Scanning for WiFi networks");
                self.current_status = WifiStatus::Scanning;
                let _ = self
                    .event_sender
                    .send(WifiEvent::StatusUpdate(WifiStatus::Scanning));

                match self.wifi_manager.scan_networks() {
                    Ok(networks) => {
                        let network_names: Vec<String> =
                            networks.into_iter().map(|ap| ap.ssid.to_string()).collect();
                        let _ = self.event_sender.send(WifiEvent::ScanResult(network_names));

                        // Restore previous status after scan
                        let status = if self.wifi_manager.is_connected() {
                            WifiStatus::Connected
                        } else {
                            WifiStatus::Disconnected
                        };
                        self.current_status = status.clone();
                        let _ = self.event_sender.send(WifiEvent::StatusUpdate(status));
                    }
                    Err(e) => {
                        let error_msg = format!("WiFi scan failed: {}", e);
                        let _ = self
                            .event_sender
                            .send(WifiEvent::StatusUpdate(WifiStatus::Error(error_msg)));

                        // Restore previous status
                        let status = if self.wifi_manager.is_connected() {
                            WifiStatus::Connected
                        } else {
                            WifiStatus::Disconnected
                        };
                        self.current_status = status.clone();
                        let _ = self.event_sender.send(WifiEvent::StatusUpdate(status));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_connection_status(&mut self) {
        let is_connected = self.wifi_manager.is_connected();

        match (&self.current_status, is_connected) {
            (WifiStatus::Connected, false) => {
                info!("WiFi connection lost");
                self.current_status = WifiStatus::Disconnected;
                let _ = self.event_sender.send(WifiEvent::Disconnected);
                let _ = self
                    .event_sender
                    .send(WifiEvent::StatusUpdate(WifiStatus::Disconnected));
            }
            (WifiStatus::Disconnected, true) => {
                info!("WiFi connection restored");
                self.current_status = WifiStatus::Connected;
                if let Ok(ip) = self.wifi_manager.get_ip_info() {
                    let ip_str = format!("{}", ip);
                    let _ = self.event_sender.send(WifiEvent::Connected(ip_str));
                }
                let _ = self
                    .event_sender
                    .send(WifiEvent::StatusUpdate(WifiStatus::Connected));
            }
            _ => {} // No status change
        }
    }
}

pub struct WifiActorManager {
    command_sender: Sender<WifiCommand>,
    event_receiver: Receiver<WifiEvent>,
}

impl WifiActorManager {
    pub fn new(
        modem: Modem,
        sys_loop: EspSystemEventLoop,
        nvs: Option<EspDefaultNvsPartition>,
    ) -> Result<Self> {
        let (command_sender, command_receiver) = std::sync::mpsc::channel::<WifiCommand>();
        let (event_sender, event_receiver) = std::sync::mpsc::channel::<WifiEvent>();

        let event_sender_clone = event_sender.clone();

        thread::Builder::new()
            .stack_size(64 * 1024)
            .name("wifi_actor".to_string())
            .spawn(move || {
                match WifiActor::new(
                    modem,
                    sys_loop,
                    nvs,
                    command_receiver,
                    event_sender_clone.clone(),
                ) {
                    Ok(mut actor) => {
                        actor.run();
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to create WiFi actor: {}", e);
                        let _ = event_sender_clone
                            .send(WifiEvent::StatusUpdate(WifiStatus::Error(error_msg)));
                    }
                }
            })
            .expect("Failed to spawn WiFi actor thread");

        Ok(Self {
            command_sender,
            event_receiver,
        })
    }

    pub fn connect(&self, config: WifiConfig) -> Result<()> {
        self.command_sender.send(WifiCommand::Connect(config))?;
        Ok(())
    }

    pub fn disconnect(&self) -> Result<()> {
        self.command_sender.send(WifiCommand::Disconnect)?;
        Ok(())
    }

    pub fn get_status(&self) -> Result<()> {
        self.command_sender.send(WifiCommand::GetStatus)?;
        Ok(())
    }

    pub fn scan_networks(&self) -> Result<()> {
        self.command_sender.send(WifiCommand::Scan)?;
        Ok(())
    }

    pub fn try_recv_event(&self) -> Result<WifiEvent, std::sync::mpsc::TryRecvError> {
        self.event_receiver.try_recv()
    }

    pub fn recv_event(&self) -> Result<WifiEvent, std::sync::mpsc::RecvError> {
        self.event_receiver.recv()
    }

    pub fn recv_event_timeout(
        &self,
        timeout: Duration,
    ) -> Result<WifiEvent, std::sync::mpsc::RecvTimeoutError> {
        self.event_receiver.recv_timeout(timeout)
    }
}

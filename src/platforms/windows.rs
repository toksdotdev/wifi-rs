use platforms::Config;
// use platforms::WifiError;
use platforms::WifiInterface;
// use std::process::Command;

#[derive(Debug)]
pub struct Connection {
    pub(crate) ssid: String,
}

/// Wireless network interface for windows operating system.
#[derive(Debug)]
pub struct Windows {
    pub(crate) connection: Option<Connection>,
    pub(crate) interface: String,
}

impl Windows {
    pub fn new(config: Option<Config>) -> Self {
        Windows {
            connection: None,
            interface: config.map_or("wlan0".to_string(), |cfg| {
                cfg.interface.unwrap_or("wlan0").to_string()
            }),
        }
    }
}

/// Wifi interface for windows operating system.
/// This provides basic functionalities for wifi interface.
impl WifiInterface for Windows {}

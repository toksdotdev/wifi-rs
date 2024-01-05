use crate::platforms::{Config, WifiError, WifiInterface};
use std::process::Command;

#[derive(Debug)]
pub struct Connection {
    // pub(crate) ssid: String,
}

/// Wireless network interface for linux operating system.
#[derive(Debug)]
pub struct Linux {
    pub(crate) connection: Option<Connection>,
    pub(crate) interface: String,
}

impl Linux {
    pub fn new(config: Option<Config>) -> Self {
        Linux {
            connection: None,
            interface: config.map_or("wlan0".to_string(), |cfg| {
                cfg.interface.unwrap_or("wlan0").to_string()
            }),
        }
    }
}

/// Wifi interface for linux operating system.
/// This provides basic functionalities for wifi interface.
impl WifiInterface for Linux {
    /// Check if wireless network adapter is enabled.
    fn is_wifi_enabled() -> Result<bool, WifiError> {
        let output = Command::new("nmcli")
            .args(&["radio", "wifi"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .replace(" ", "")
            .replace("\n", "")
            .contains("enabled"))
    }

    /// Turn on the wireless network adapter.
    fn turn_on() -> Result<(), WifiError> {
        Command::new("nmcli")
            .args(&["radio", "wifi", "on"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }

    /// Turn off the wireless network adapter.
    fn turn_off() -> Result<(), WifiError> {
        Command::new("nmcli")
            .args(&["radio", "wifi", "off"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }
}

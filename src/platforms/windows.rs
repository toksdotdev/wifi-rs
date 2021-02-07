use crate::platforms::{Config, WifiError, WifiInterface};
use std::process::Command;

const WINDOWS_INTERFACE: &'static str = "Wireless Network Connection";

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
impl WifiInterface for Windows {
    /// Check if wireless network adapter is enabled.
    fn is_wifi_enabled() -> Result<bool, WifiError> {
        let output = Command::new("netsh")
            .args(&[
                "wlan",
                "show",
                "interface",
                &format!("name= \"{}\"", WINDOWS_INTERFACE),
            ])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(!String::from_utf8_lossy(&output.stdout).contains("There is no wireless interface"))
    }

    /// Turn on the wireless network adapter.
    fn turn_on() -> Result<(), WifiError> {
        Command::new("netsh")
            .args(&[
                "interface",
                "set",
                "interface",
                &format!("name= \"{}\"", WINDOWS_INTERFACE),
                "ENABLED",
            ])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }

    /// Turn off the wireless network adapter.
    fn turn_off() -> Result<(), WifiError> {
        let _output = Command::new("netsh")
            .args(&[
                "interface",
                "set",
                "interface",
                &format!("name= \"{}\"", WINDOWS_INTERFACE),
                "DISABLED",
            ])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }
}

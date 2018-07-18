use platforms::{WifiError, WifiInterface};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug)]
pub(crate) struct Connection {
    pub ssid: String,
}

#[derive(Debug)]
pub(crate) struct Linux {
    hotspot: Option<HashMap<String, String>>,
    pub connection: Option<Connection>,
    pub interface: String,
}

impl Linux {
    pub fn new(name: &str, interface: Option<&str>) -> Self {
        Linux {
            hotspot: None,
            connection: None,
            interface: String::from("wlan0"),
        }
    }
}

impl WifiInterface for Linux {
    fn is_wifi_enabled() -> bool {
        let output = Command::new("nmcli").args(&["radio", "wifi"]).output();

        if let Err(_) = output {
            return false;
        }

        String::from_utf8_lossy(&output.unwrap().stdout)
            .replace(" ", "")
            .replace("\n", "")
            .contains("enabled")
    }

    fn turn_on() -> Result<bool, WifiError> {
        let _output = Command::new("nmcli")
            .args(&["radio", "wifi", "on"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(true)
    }

    fn turn_off() -> Result<bool, WifiError> {
        let _output = Command::new("nmcli")
            .args(&["radio", "wifi", "off"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(true)
    }
}

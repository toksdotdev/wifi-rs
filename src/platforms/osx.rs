use platforms::{WifiError, WifiInterface};
use std::process::Command;

#[derive(Debug)]
pub struct Connection {
    pub(crate) ssid: String,
}

#[derive(Debug)]
pub struct Osx {
    pub(crate) connection: Option<Connection>,
    pub(crate) interface: String,
}

impl Osx {
    pub fn new(name: &str, config: Option<Config>) -> Self {
        Osx {
            connection: None,
            interface: config.map_or("en0".to_string(), |cfg| {
                cfg.interface.unwrap_or("en0").to_string()
            }),
        }
    }
}

impl WifiInterface for Osx {
    fn is_wifi_enabled() -> Result<bool, WifiError> {
        let output = Command::new("networksetup")
            .args(&["radio", "wifi"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .replace(" ", "")
            .replace("\n", "")
            .contains("enabled"))
    }

    fn turn_on() -> Result<(), WifiError> {
        let output = Command::new("networksetup")
            .args(&["-setairportpower", "en0", "on"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }

    fn turn_off() -> Result<(), WifiError> {
        let output = Command::new("networksetup")
            .args(&["-setairportpower", "en0", "off"])
            .output()
            .map_err(|err| WifiError::IoError(err))?;

        Ok(())
    }
}

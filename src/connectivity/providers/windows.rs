use connectivity::handlers::NetworkXmlProfileHandler;
use connectivity::{Connectivity, WifiConnectionError};
use platforms::{Connection, WiFi, WifiError, WifiInterface};
use std::process::Command;

impl WiFi {
    /// Add the wireless network profile of network to connect to,
    /// (this is specific to windows operating system).
    fn add_profile(ssid: &str, password: &str) -> Result<(), WifiConnectionError> {
        let mut handler = NetworkXmlProfileHandler::new();
        handler.content = handler
            .content
            .replace("{SSID}", ssid)
            .replace("{password}", password);

        let temp_file = handler.write_to_temp_file()?;

        Command::new("netsh")
            .args(&[
                "wlan",
                "add",
                "profile",
                &format!("filename={}", temp_file.path().to_str().unwrap()),
            ])
            .output()
            .map_err(|_| WifiConnectionError::AddNetworkProfileFailed)?;

        Ok(())
    }
}

/// Wireless network connectivity functionality.
impl Connectivity for WiFi {
    /// Attempts to connect to a wireless network with a given SSID and password.
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError> {
        if !WiFi::is_wifi_enabled().map_err(|err| WifiConnectionError::Other { kind: err })? {
            return Err(WifiConnectionError::Other {
                kind: WifiError::WifiDisabled,
            });
        }

        Self::add_profile(ssid, password)?;

        let output = Command::new("netsh")
            .args(&[
                "wlan",
                "connect",
                &format!("name={}", ssid),
            ])
            .output()
            .map_err(|err| WifiConnectionError::FailedToConnect(format!("{}", err)))?;

        if !String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("completed successfully")
        {
            return Ok(false);
        }

        self.connection = Some(Connection {
            ssid: String::from(ssid),
        });

        Ok(true)
    }

    /// Attempts to disconnect from a wireless network currently connected to.
    fn disconnect(&self) -> Result<bool, WifiConnectionError> {
        let output = Command::new("netsh")
            .args(&["wlan", "disconnect"])
            .output()
            .map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }
}

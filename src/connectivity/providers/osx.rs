use connectivity::{Connectivity, WifiConnectionError};
use platforms::{Connection, WiFi, WifiError, WifiInterface};
use std::process::Command;

/// Wireless network connectivity functionality.
impl Connectivity for WiFi {
    /// Attempts to connect to a wireless network with a given SSID and password.
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError> {
        if !WiFi::is_wifi_enabled().map_err(|err| WifiConnectionError::Other { kind: err })? {
            return Err(WifiConnectionError::Other {
                kind: WifiError::WifiDisabled,
            });
        }

        let output = Command::new("networksetup")
            .args(&["-setairportnetwork", &self.interface, &ssid, &password])
            .output()
            .map_err(|err| WifiConnectionError::FailedToConnect(format!("{}", err)))?;

        if String::from_utf8_lossy(&output.stdout).as_ref() != "" {
            return Ok(false);
        }

        self.connection = Some(Connection {
            ssid: String::from(ssid),
        });

        Ok(true)
    }

    /// Attempts to disconnect from a wireless network currently connected to.
    fn disconnect(&self) -> Result<bool, WifiConnectionError> {
        let output = Command::new("networksetup")
            .args(&[
                "-removepreferredwirelessnetwork",
                &*self.interface,
                &*self.connection.as_ref().unwrap().ssid,
            ])
            .output()
            .map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }
}

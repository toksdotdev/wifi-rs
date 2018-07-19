use connectivity::{Network, WifiConnectionError};
use platforms::{Connection, WiFi, WifiError, WifiInterface};
use std::process::Command;

impl Network for WiFi {
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError> {
        if !WiFi::is_wifi_enabled().map_err(|err| WifiConnectionError::Other { kind: err })? {
            return Err(WifiConnectionError::Other {
                kind: WifiError::InterfaceDisabled,
            });
        }

        let output = Command::new("nmcli")
            .args(&[
                "d",
                "wifi",
                "connect",
                ssid,
                "password",
                &password,
                "ifname",
                &self.interface,
            ])
            .output()
            .map_err(|err| WifiConnectionError::FailedToConnect(format!("{}", err)))?;

        if !String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully activated")
        {
            return Ok(false);
        }

        self.connection = Some(Connection {
            ssid: String::from(ssid),
        });

        Ok(true)
    }

    fn disconnect(&self) -> Result<bool, WifiConnectionError> {
        let output = Command::new("nmcli")
            .args(&["d", "disconnect", "ifname", &self.interface])
            .output()
            .map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }
}

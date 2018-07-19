use connectivity::handlers::NetworkXmlProfileHandler;
use connectivity::{Network, WifiConnectionError};
use platforms::{Connection, WiFi, WifiError, WifiInterface};
use std::process::Command;

impl WiFi {
    fn add_profile(ssid: &str, password: &str) -> Result<(), WifiConnectionError> {
        let mut handler = NetworkXmlProfileHandler::new();
        handler.content = handler
            .content
            .replace("{SSID}", ssid)
            .replace("{password}", password);

        let temp_file = handler.write_to_temp_file()?;

        // Add the network profile
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

impl Network for Windows {
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError> {
        if !WiFi::is_wifi_enabled().map_err(|err| WifiConnectionError::Other { kind: err })? {
            return Err(WifiConnectionError::Other {
                kind: WifiError::InterfaceDisabled,
            });
        }

        Self::add_profile(ssid, password)?;

        let output = Command::new("netsh")
            .args(&[
                "wlan",
                "connect",
                &format!("name={}", *self.connection.unwrap().ssid),
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
        let output = Command::new("netsh")
            .args(&["wlan", "disconnect"])
            .output()
            .map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }
}

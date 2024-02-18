use crate::connectivity::{Connectivity, WifiConnectionError};
use crate::platforms::{Connection, WiFi, WifiError, WifiInterface};
use std::process::{Command, Stdio};
use std::str;
use std::format;

/// Wireless network connectivity functionality.
impl Connectivity for WiFi {
    /// Attempts to connect to a wireless network with a given SSID and password.
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError> {
        if !WiFi::is_wifi_enabled().map_err(|err| WifiConnectionError::Other { kind: err })? {
            return Err(WifiConnectionError::Other {
                kind: WifiError::WifiDisabled,
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
            // ssid: String::from(ssid),
        });

        Ok(true)
    }

    /// Attempts to disconnect from a wireless network currently connected to.
    fn disconnect(&self) -> Result<bool, WifiConnectionError> {
        let output = Command::new("nmcli")
            .args(&["d", "disconnect", "ifname", &self.interface])
            .output()
            .map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }

    fn speed(&self) -> Result<String, WifiConnectionError> {
        let nmcli_speed = Command::new("nmcli")
            .arg("dev")
            .arg("wifi")
            .arg("list")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let piped_awk = Command::new("awk")
            .arg(r"/\*/{if (NR!=1) {print $6}}")
            .stdin(Stdio::from(nmcli_speed.stdout.unwrap())) // Pipe through.
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let result = piped_awk.wait_with_output().unwrap();
        let result = str::from_utf8(&result.stdout).unwrap().to_string();
        let formatted_result = format!("{} Mbit/s", result.trim());
        Ok(formatted_result)
    }
}

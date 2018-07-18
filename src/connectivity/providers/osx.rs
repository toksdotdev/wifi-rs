use connectivity::{Network, WifiConnectionError};
use std::process::Command;

#[derive(Debug)]
pub struct OSX {
    pub name: String,
    interface: String,
}

impl OSX {
    #[cfg(target_os = "windows")]
    pub fn new(name: &str, interface: Option<&str>) -> Self {
        OSX {
            name: name.into(),
            interface: interface.unwrap_or("en0").into(),
        }
    }
}

impl Network for OSX {
    fn connect(&self, password: &str) -> Result<bool, WifiConnectionError> {
        let output = Command::new("networksetup")
            .args(&["-setairportnetwork", &self.interface, &self.name, &password])
            .output()
            .map_err(|err| WifiConnectionError::FailedToConnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully activated"))
    }

    fn disconnect(&self) -> Result<bool, WifiConnectionError> {
        let output = Command::new("networksetup")
            .args(&[
                "-removepreferredwirelessnetwork",
                &self.interface,
                &self.name,
            ])
            .output()
            .map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }

    fn is_wifi_enabled(&self) -> bool {
        let output = Command::new("networksetup")
            .args(&["radio", "wifi"])
            .output();

        if let Err(_) = output {
            return false;
        }

        String::from_utf8_lossy(&output.unwrap().stdout)
            .replace(" ", "")
            .replace("\n", "")
            .contains("enabled")
    }

    fn connnection_up(&self) -> bool {
        let output = Command::new("networksetup")
            .args(&["-setairportpower", &self.interface, "on"])
            .output();

        if let Err(_) = output {
            return false;
        }

        false
    }

    fn connnection_down(&self) -> bool {
        let output = Command::new("networksetup")
            .args(&["-setairportpower", &self.interface, "off"])
            .output();

        if let Err(_) = output {
            return false;
        }

        false
    }
}

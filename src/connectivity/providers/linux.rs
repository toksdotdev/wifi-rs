use connectivity::{Network, WifiConnectionError};
use std::process::Command;

#[derive(Debug)]
pub struct Linux {
    pub name: String,
    interface: String,
}

impl Linux {
    pub fn new(name: &str, interface: Option<&str>) -> Self {
        Linux {
            name: name.into(),
            interface: interface.unwrap_or("wlan0").into(),
        }
    }
}

impl Network for Linux {
    fn connect(&self, password: &str) -> Result<bool, WifiConnectionError> {
        let output = Command::new("nmcli")
            .args(&[
                "d",
                "wifi",
                "connect",
                &self.name,
                "password",
                &password,
                "ifname",
                &self.interface,
            ])
            .output()
            .map_err(|err| WifiConnectionError::FailedToConnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully activated"))
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

    fn is_wifi_enabled(&self) -> bool {
        let output = Command::new("nmcli").args(&["radio", "wifi"]).output();

        if let Err(_) = output {
            return false;
        }

        String::from_utf8_lossy(&output.unwrap().stdout)
            .replace(" ", "")
            .replace("\n", "")
            .contains("enabled")
    }

    fn connnection_up(&self) -> bool {
        let output = Command::new("nmcli")
            .args(&["radio", "wifi", "on"])
            .output();

        if let Err(_) = output {
            return false;
        }

        false
    }

    fn connnection_down(&self) -> bool {
        let output = Command::new("nmcli")
            .args(&["radio", "wifi", "off"])
            .output();

        if let Err(_) = output {
            return false;
        }

        false
    }
}

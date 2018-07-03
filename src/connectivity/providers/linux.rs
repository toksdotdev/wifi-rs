use connectivity::{Network, NetworkError};
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
    fn connect(&self, password: &str) -> Result<bool, NetworkError> {
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
            .map_err(|err| NetworkError::FailedToConnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully activated"))
    }

    fn disconnect(&self) -> Result<bool, NetworkError> {
        let output = Command::new("nmcli")
            .args(&["d", "disconnect", "ifname", &self.interface])
            .output()
            .map_err(|err| NetworkError::FailedToDisconnect(format!("{}", err)))?;

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
}

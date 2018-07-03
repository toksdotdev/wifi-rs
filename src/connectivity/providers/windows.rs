use connectivity::handlers::NetworkXmlProfileHandler;
use connectivity::{Network, NetworkError};

use std::process::Command;

#[derive(Debug)]
pub(crate) struct Windows {
    name: String,
    pub output_xml_path: String,
}

impl Windows {
    #[cfg(target_os = "windows")]
    pub fn new(name: &str, interface: Option<&str>) -> Self {
        Windows {
            name: String::from(name),
            output_xml_path: OUTPUT_XML_FILE_PATH.into(),
        }
    }

    pub(crate) fn add_profile(&self, password: &str) -> Result<(), NetworkError> {
        let mut handler = NetworkXmlProfileHandler::new();
        handler.content = handler
            .content
            .replace("{SSID}", &self.name)
            .replace("{password}", password);

        // Write details to new xml file
        let _ = handler
            .to_file(&self.output_xml_path)
            .map_err(|err| NetworkError::IoError(err))?;

        // Add the network profile
        Command::new("netsh")
            .args(&[
                "wlan",
                "add",
                "profile",
                &format!("filename={}", self.output_xml_path),
            ])
            .output()
            .map_err(|_| NetworkError::AddNetworkProfileFailed)?;

        Ok(())
    }
}

impl Network for Windows {
    fn connect(&self, password: &str) -> Result<bool, NetworkError> {
        self.add_profile(password)?;

        let output = Command::new("netsh")
            .args(&["wlan", "connect", &format!("name={}", self.name)])
            .output()
            .map_err(|err| NetworkError::FailedToConnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully activated"))
    }

    fn disconnect(&self) -> Result<bool, NetworkError> {
        let output = Command::new("netsh")
            .args(&["wlan", "disconnect"])
            .output()
            .map_err(|err| NetworkError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }

    fn is_wifi_enabled(&self) -> bool {
        // let output = Command::new("nmcli").args(&["radio", "wifi"]).output();

        // if let Err(_) = output {
        //     return false;
        // }

        // match String::from_utf8_lossy(&output.unwrap().stdout).as_ref() {
        //     "enabled" => true,
        //     _ => false,
        // }
        false
    }
}

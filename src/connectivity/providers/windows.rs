use connectivity::handlers::NetworkXmlProfileHandler;
use connectivity::Network;
use std::io;
use std::process::Command;

const OUTPUT_XML_FILE_PATH: &str = "output.xml";

#[cfg(target_os = "windows")]
pub(crate) struct Windows {
    name: String,
    pub output_xml_path: String,
}

impl Windows {
    pub fn new(name: &str) -> Result<Self, io::Error> {
        Ok(Windows {
            name: String::from(name),
            output_xml_path: OUTPUT_XML_FILE_PATH.into(),
        })
    }

    pub(crate) fn add_profile(&self, password: &str) -> bool {
        let mut handler = NetworkXmlProfileHandler::new();
        handler.content = handler
            .content
            .replace("{SSID}", &self.name)
            .replace("{password}", password);

        // Write details to new xml file
        if let Err(_) = handler.to_file(&self.output_xml_path).map_err(|_err| false) {
            return false;
        }

        // Add the network profile
        if let Err(_) = Command::new("netsh")
            .args(&["wlan", "add", "profile", &format!("filename={}", self.output_xml_path)])
            .output()
            {
                return false;
            }
        
        true
    }
}

impl Network for Windows {
    fn connect(&self, password: &str) -> bool {
        if self.add_profile(password) == false {
            return false;
        }
        
        let output = Command::new("netsh")
            .args(&["wlan", "connect", &format!("name={}", self.name)])
            .output();

        match output {
            Ok(res) => res.status.success(),
            Err(_) => false,
        }
    }
}

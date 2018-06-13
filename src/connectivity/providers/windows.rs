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
        let profile_file_name = format!("netsh wlan add profile filename=\"{}\"", name);

        Command::new("cmd")
            .args(&["/C", &profile_file_name[..]])
            .output()?;

        Ok(Windows {
            name: String::from(name),
            output_xml_path: OUTPUT_XML_FILE_PATH.into(),
        })
    }
}

impl Network for Windows {
    fn connect(&self, password: &str) -> bool {
        {
            let mut handler = NetworkXmlProfileHandler::new();

            handler.content = handler
                .content
                .replace("{SSID}", &self.name)
                .replace("{password}", password);

            // Write details to new xml file
            if let Err(err) = handler.to_file(&self.output_xml_path).map_err(|_err| false) {
                return err;
            }
        }

        let output = Command::new("netsh")
            .args(&["wlan", "connect", &format!("name = \"{}\"", self.name)])
            .output();

        match output {
            Ok(res) => res.status.success(),
            Err(_) => false,
        }
    }
}

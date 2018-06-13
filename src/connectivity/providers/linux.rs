use connectivity::{Network, NetworkType, NetworkTypeParseError};
use std::io;
use std::io::{Error, ErrorKind};
use std::process::{Command, Output};

pub struct Linux {
    pub name: String,
    pub network_type: NetworkType,
}

impl Linux {
    pub fn new(name: String) -> Result<Self, io::Error> {
        match Linux::check_if_web_or_wpa(name.clone()) {
            Ok(t) => match t {
                NetworkType::WEP => Ok(Linux {
                    name: name.clone(),
                    network_type: NetworkType::WEP,
                }),
                _ => Ok(Linux {
                    name: name.clone(),
                    network_type: NetworkType::WPA,
                }),
            },
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to parse")), // use the NetworkTypeParseError::IoError here
        }
    }

    /// Detects the network type of a given network.
    fn check_if_web_or_wpa(name: String) -> Result<NetworkType, NetworkTypeParseError> {
        Command::new("nmcli")
            .args(&[
                "con",
                "list",
                "id",
                "\"",
                &name,
                "\"",
                "|",
                "awk",
                "'/key-mgmt/ {{ print $2 }}'",
            ])
            .output()
            .map_err(|err| NetworkTypeParseError::IoError(err))
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .map_err(|err| NetworkTypeParseError::FromUtf8Error(err))
                    .and_then(|result| match result.as_ref() {
                        "wpa-psk" => Ok(NetworkType::WPA),
                        _ => Ok(NetworkType::WEP),
                    })
            })
    }

    pub fn connect_to_wep_network(&self, password: &str) -> Result<Output, io::Error> {
        Command::new("iwconfig")
            .args(&["wlan0", "essid", &self.name, "key", password])
            .output()
    }

    pub fn connect_to_wpa_network(&self, password: &str) -> Result<Output, io::Error> {
        // Dynamically generate differennt version of file (if running sync)
        Command::new("wpa_passphrase")
            .args(&[&self.name, password, "wpa.conf"])
            .output()?;

        Ok(Command::new("wpa_supplicant")
            .args(&["-Dwext", "-i", "wlan0", "-c/root/wpa.conf"])
            .output()?)
    }
}

impl Network for Linux {
    fn connect(&self, password: &str) -> bool {
        match self.network_type {
            NetworkType::WEP => self
                .connect_to_wep_network(password)
                .map_err(|_err| false)
                .unwrap()
                .status
                .success(),
            _ => self
                .connect_to_wpa_network(password)
                .map_err(|_err| false)
                .unwrap()
                .status
                .success(),
        }
    }
}

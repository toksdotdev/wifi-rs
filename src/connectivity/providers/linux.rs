extern crate tempfile;

use self::tempfile::NamedTempFile;
use connectivity::{Network, NetworkError, NetworkType};
use std::process::{Command, Output};
use std::{
    fs::File, io::{Error, Read, Write},
};

#[derive(Debug)]
pub struct Linux {
    pub name: String,
    pub network_type: NetworkType,
}

impl Linux {
    pub fn new(name: &str) -> Result<Self, NetworkError> {
        match Linux::check_if_web_or_wpa(name.into()) {
            Ok(t) => match t {
                NetworkType::WEP => Ok(Linux {
                    name: name.into(),
                    network_type: NetworkType::WEP,
                }),
                _ => Ok(Linux {
                    name: name.into(),
                    network_type: NetworkType::WPA,
                }),
            },
            Err(err) => Err(err),
        }
    }

    /// Detects the network type of a given network.
    fn check_if_web_or_wpa(name: String) -> Result<NetworkType, NetworkError> {
        Command::new("nmcli")
            .args(&[
                "-t",
                "-f",
                "802-11-wireless-security.key-mgmt",
                "con",
                "show",
                &name,
            ])
            .output()
            .map_err(|err| NetworkError::IoError(err))
            .and_then(|output| {
                let output = String::from_utf8_lossy(&output.stdout);
                let psk = {
                    let mut split = output.split(':');
                    let _ = split.next();
                    match split.next() {
                        Some(x) => x,
                        None => return Err(NetworkError::SsidNotFound),
                    }
                };

                match psk.trim() {
                    "wpa-psk" => Ok(NetworkType::WPA),
                    _ => Ok(NetworkType::WEP),
                }
            })
    }

    pub fn connect_to_wep_network(&self, password: &str) -> Result<Output, NetworkError> {
        Ok(Command::new("iwconfig")
            .args(&["wlan0", "essid", &self.name, "key", password])
            .output()
            .map_err(|err| NetworkError::FailedToConnect(format!("{:?}", err)))?)
    }

    pub fn connect_to_wpa_network(&self, password: &str) -> Result<Output, NetworkError> {
        let passphrase = Command::new("wpa_passphrase")
            .args(&[&self.name, password])
            .output()?;

        let wpa_conf_file = self.create_conf_file(&String::from_utf8_lossy(&passphrase.stdout))?;

        let mut file = File::open(wpa_conf_file.path())?;
        let mut new_content = String::new();
        file.read_to_string(&mut new_content)?;

        Command::new("wpa_supplicant")
            .args(&[
                "-D",
                "wext",
                "-B",
                "-i",
                "wlo1",
                wpa_conf_file.path().to_str().unwrap(),
            ])
            .output()
            .map_err(|err| NetworkError::FailedToConnect(format!("{:?}", err)))?;

        let a = Command::new("dhclient").args(&["wlo1"]).output()?;
        println!("{:?}", String::from_utf8_lossy(&a.stdout));

        Ok(Command::new("dhclient")
            .args(&["wlo1"])
            .output()
            .map_err(|_| NetworkError::IpAssignFailed)?)
    }

    fn create_conf_file(&self, content: &str) -> Result<NamedTempFile, Error> {
        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "{}", content)?;

        Ok(temp_file)
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

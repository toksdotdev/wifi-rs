pub mod profile_network;
mod providers;

#[cfg(target_os = "windows")]
mod handlers;
#[cfg(target_os = "windows")]
mod stubs;

use std::{fmt, io};

pub trait Network: fmt::Debug {
    /// Makes an attempt to connect to a selected wireless network with password specified.
    fn connect(&self, password: &str) -> Result<bool, WifiConnectionError>;
    fn disconnect(&self) -> Result<bool, WifiConnectionError>;
    fn is_wifi_enabled(&self) -> bool;
    fn connnection_up(&self) -> bool;
    fn connnection_down(&self) -> bool;

    // Hotspot
    // fn create_hotspot(&self, ssid: &str, password: &str) -> Result<bool, WifiHotspotError>;
}

// #[derive(Debug)]
// pub enum NetworkType {
//     WEP,
//     WPA,
//     WPA2,
// }

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub interface: Option<&'a str>,
}

#[derive(Debug)]
pub enum WifiConnectionError {
    SsidNotFound,
    OsNotSupported,
    IpAssignFailed,
    AddNetworkProfileFailed,
    IoError(io::Error),
    FailedToConnect(String),
    FailedToDisconnect(String),
    WiFiInterfaceDisabled,
}

pub enum WifiHotspotError {
    CreationFailed,
}

impl From<io::Error> for WifiConnectionError {
    fn from(error: io::Error) -> Self {
        WifiConnectionError::IoError(error)
    }
}

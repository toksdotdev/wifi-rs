#[cfg(target_os = "windows")]
mod handlers;

#[cfg(target_os = "windows")]
mod stubs;

mod providers;

use platforms::WifiError;
use std::{fmt, io};

pub trait Network: fmt::Debug {
    /// Makes an attempt to connect to a selected wireless network with password specified.
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError>;

    /// Disconnects from a wireless network currently connected to.
    fn disconnect(&self) -> Result<bool, WifiConnectionError>;
}

/// Configuration for a wifi network.
#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub interface: Option<&'a str>,
}

#[derive(Debug)]
pub enum WifiConnectionError {
    SsidNotFound,
    IpAssignFailed,
    AddNetworkProfileFailed,
    FailedToConnect(String),
    FailedToDisconnect(String),
    Other { kind: WifiError },
}

impl From<io::Error> for WifiConnectionError {
    fn from(error: io::Error) -> Self {
        WifiConnectionError::Other {
            kind: WifiError::IoError(error),
        }
    }
}

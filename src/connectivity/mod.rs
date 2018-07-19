#[cfg(target_os = "windows")]
mod handlers;
mod providers;
#[cfg(target_os = "windows")]
mod stubs;

use platforms::WifiError;
use std::{fmt, io};

pub trait Network: fmt::Debug {
    /// Makes an attempt to connect to a selected wireless network with password specified.
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError>;

    /// Disconnects from a wireless network currently connected to.
    fn disconnect(&self) -> Result<bool, WifiConnectionError>;
}

#[derive(Debug)]
pub enum WifiConnectionError {
    // SsidNotFound,
    // IpAssignFailed,
    #[cfg(target_os = "windows")]
    AddNetworkProfileFailed,
    FailedToConnect(String),
    FailedToDisconnect(String),
    Other {
        kind: WifiError,
    },
}

impl From<io::Error> for WifiConnectionError {
    fn from(error: io::Error) -> Self {
        WifiConnectionError::Other {
            kind: WifiError::IoError(error),
        }
    }
}

use connectivity::{Network, WifiConnectionError};
use std::process::Command;

#[derive(Debug)]
pub struct OSX {
    pub name: String,
    interface: String,
}

impl OSX {
    #[cfg(target_os = "windows")]
    pub fn new(name: &str, interface: Option<&str>) -> Self {
        OSX {
            name: name.into(),
            interface: interface.unwrap_or("en0").into(),
        }
    }
}

use platforms::{WifiError, WifiInterface};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Connection {
    pub(crate) ssid: String,
}

#[derive(Debug)]
pub struct Windows {
    pub(crate) connection: Option<Connection>,
    pub(crate) interface: String,
}

impl Windows {
    pub fn new(name: &str, config: Option<Config>) -> Self {
        Windows {
            connection: None,
            interface: config.map_or("wlan0".to_string(), |cfg| {
                cfg.interface.unwrap_or("wlan0").to_string()
            }),
        }
    }
}

impl WifiInterface for Windows {}

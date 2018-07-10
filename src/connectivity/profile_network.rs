use connectivity::{providers::Machine, Config, Network, WifiConnectionError};

#[derive(Debug)]
pub struct ProfileNetwork {
    handler: Box<Network>,
}

/// Profile Network handler responsible to connect to a wireless network.
impl ProfileNetwork {
    pub fn new(name: &str, config: Option<Config>) -> Result<Self, WifiConnectionError> {
        if !(cfg!(target_os = "linux") || cfg!(target_os = "windows")) {
            return Err(WifiConnectionError::OsNotSupported);
        }

        let handler = Machine::new(
            name,
            config.map_or(None, |cfg| cfg.interface.map_or(None, |x| Some(x))),
        );

        Ok(ProfileNetwork {
            handler: Box::new(handler),
        })
    }

    pub fn connect(&self, password: &str) -> Result<bool, WifiConnectionError> {
        if !self.handler.is_wifi_enabled() {
            return Err(WifiConnectionError::WiFiInterfaceDisabled);
        }

        self.handler.connect(password)
    }

    pub fn connection_up(&self) -> bool {
        false
    }
}

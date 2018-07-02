use connectivity::{providers::Machine, Config, Network, NetworkError};

#[derive(Debug)]
pub struct ProfileNetwork {
    handler: Box<Network>,
}

/// Profile Network handler responsible to connect to a wireless network.
impl ProfileNetwork {
    pub fn new(name: &str, config: Option<Config>) -> Result<Self, NetworkError> {
        if !(cfg!(target_os = "linux") || cfg!(target_os = "windows")) {
            return Err(NetworkError::OsNotSupported);
        }

        let handler = Machine::new(
            name,
            config.map_or(None, |cfg| cfg.interface.map_or(None, |x| Some(&x))),
        );

        Ok(ProfileNetwork {
            handler: Box::new(handler),
        })
    }

    pub fn connect(&self, password: &str) -> Result<bool, NetworkError> {
        self.handler.connect(password)
    }
}

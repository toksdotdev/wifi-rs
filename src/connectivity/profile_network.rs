use connectivity::{providers::Machine, Network, NetworkError};

#[derive(Debug)]
pub struct ProfileNetwork {
    handler: Box<Network>,
}

/// Profile Network handler responsible to connect to a wireless network.
impl ProfileNetwork {
    pub fn new(name: &str) -> Result<Self, NetworkError> {
        if !(cfg!(target_os = "linux") || cfg!(target_os = "windows")) {
            return Err(NetworkError::OsNotSupported);
        }

        let handler = Machine::new(name)?;

        Ok(ProfileNetwork {
            handler: Box::new(handler),
        })
    }

    pub fn connect(&self, password: &str) -> bool {
        self.handler.connect(password)
    }
}

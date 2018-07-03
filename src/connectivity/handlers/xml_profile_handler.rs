extern crate tempfile;

use self::tempfile::NamedTempFile;
use connectivity::stubs::windows_wifi_profile;
use std::{io, io::Write};

pub(crate) struct NetworkXmlProfileHandler {
    pub content: String,
}

impl NetworkXmlProfileHandler {
    pub fn new() -> Self {
        NetworkXmlProfileHandler {
            content: NetworkXmlProfileHandler::read_from_stub(),
        }
    }

    pub fn read_from_stub() -> String {
        windows_wifi_profile::get_wifi_profile()
    }

    /// Recreate the file and dump the processed contents to it
    pub fn write_to_temp_file(&mut self) -> Result<NamedTempFile, io::Error> {
        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "{}", self.content)?;

        Ok(temp_file)
    }
}

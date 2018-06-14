use std::fs;
use std::io;
use connectivity::stubs::windows_wifi_profile;

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
    pub fn to_file(&mut self, file_path: &str) -> Result<(), io::Error> {
        Ok(fs::write(&file_path, self.content.as_bytes())?)
    }
}

use hotspot::{WifiHotspot, WifiHotspotError};
use platforms::Linux;

// #[derive(Debug)]
// pub struct Linux {
//     pub name: String,
//     interface: String,
// }

// impl Linux {
//     pub fn new(name: &str, interface: Option<&str>) -> Self {
//         Linux {
//             name: name.into(),
//             interface: interface.unwrap_or("wlan0").into(),
//         }
//     }
// }

impl WifiHotspot for Linux {
    fn create_hotspot(ssid: &str, password: &str) -> Result<bool, WifiHotspotError> {
        unimplemented!();
    }

    fn start_hotspot() -> Result<bool, WifiHotspotError> {
        unimplemented!();
    }

    fn stop_hotspot() -> Result<bool, WifiHotspotError> {
        unimplemented!();
    }
}

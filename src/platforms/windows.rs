use std::collections::HashMap;
use 

#[derive(Debug)]
pub struct Windows {
    hotspot: Option<HashMap>,
    connectivity: Option<HashMap>,
}

impl WifiInterface for Windows {
    fn is_wifi_enabled() -> bool {
        unimplemented!()
    }

    fn turn_on() -> Result<bool, WifiError> {
        unimplemented!()
    }

    fn turn_off() -> Result<bool, WifiError> {
        unimplemented!()
    }
}

pub mod providers;

use self::providers::prelude::HotspotConfig;
use crate::platforms::{WifiError, WifiInterface};
use std::fmt;

/// Error that might occur when interacting managing wireless hotspot.
#[derive(Debug)]
pub enum WifiHotspotError {
    /// Failed to ceate wireless hotspot.s
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    CreationFailed,

    /// Failed to stop wireless hotspot service. Try turning off
    /// the wireless interface via ```wifi.turn_off()```.
    #[cfg(target_os = "linux")]
    FailedToStop(std::io::Error),

    /// A wireless interface error occurred.
    Other { kind: WifiError },
}

/// Wireless hotspot functionality for a wifi interface.
pub trait WifiHotspot: fmt::Debug + WifiInterface {
    /// Creates wireless hotspot service for host machine. This only creates the wifi network,
    /// and isn't responsible for initiating the serving of the wifi network process.
    /// To begin serving the hotspot, use ```start_hotspot()```.
    fn create_hotspot(
        &mut self,
        ssid: &str,
        password: &str,
        configuration: Option<&HotspotConfig>,
    ) -> Result<bool, WifiHotspotError> {
        let _a = ssid;
        let _b = password;
        let _c = configuration;

        unimplemented!();
    }

    /// Start serving publicly an already created wireless hotspot.
    fn start_hotspot() -> Result<bool, WifiHotspotError> {
        unimplemented!();
    }

    /// Stop serving a wireless network.
    /// **NOTE: All users connected will automatically be disconnected.**
    fn stop_hotspot(&mut self) -> Result<bool, WifiHotspotError> {
        unimplemented!();
    }
}

impl From<WifiError> for WifiHotspotError {
    fn from(error: WifiError) -> Self {
        WifiHotspotError::Other { kind: error }
    }
}

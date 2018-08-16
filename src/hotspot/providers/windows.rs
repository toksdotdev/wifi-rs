use hotspot::{WifiHotspot, WifiHotspotError};
use platforms::{WiFi, WifiError, WifiInterface};
use std::process::Command;

/// Configuration for a wireless hotspot.
pub struct HotspotConfig {}

impl WiFi {
    /// Attempts to turn on a wireless networ if down.
    fn try_turn_on_network_if_down() -> Result<(), WifiError> {
        if !Self::is_wifi_enabled()? {
            Self::turn_on().map_err(|err| WifiError::InterfaceFailedToOn)?;
        }

        Ok(())
    }
}

/// Wireless hotspot functionality for a wifi interface.
impl WifiHotspot for WiFi {
    /// Creates wireless hotspot service for host machine. This only creats the wifi network,
    /// and isn't responsible for initiating the serving of the wifi network process.
    /// To begin serving the hotspot, use ```start_hotspot()```.
    fn create_hotspot(
        &mut self,
        ssid: &str,
        password: &str,
        configuration: Option<&HotspotConfig>,
    ) -> Result<bool, WifiHotspotError> {
        let output = Command::new("netsh")
            .args(&[
                "wlan",
                "set",
                "hostednetwork",
                "mode = allow",
                &format!("ssid={}", ssid),
                &format!("key={}", password),
            ])
            .output()
            .map_err(|_err| WifiHotspotError::CreationFailed)?;

        if !String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully changed")
        {}

        Self::try_turn_on_network_if_down()?;

        let output = Command::new("netsh")
            .args(&["wlan", "start", "hostednetwork"])
            .output()
            .map_err(|err| WifiHotspotError::CreationFailed)?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("hosted network started"))
    }

    /// Start serving publicly an already created wireless hotspot.
    fn start_hotspot() -> Result<bool, WifiHotspotError> {
        Self::try_turn_on_network_if_down()?;

        let output = Command::new("netsh")
            .args(&["wlan", "start", "hostednetwork"])
            .output()
            .map_err(|err| WifiHotspotError::CreationFailed)?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("hosted network started"))
    }

    /// Stop serving a wireless network.
    ///
    /// **NOTE: All users connected will automatically be disconnected.**
    fn stop_hotspot(&mut self) -> Result<bool, WifiHotspotError> {
        let output = Command::new("netsh")
            .args(&["wlan", "stop", "hostednetwork"])
            .output()
            .map_err(|err| WifiHotspotError::CreationFailed)?;

        return Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("hosted network stopped"));
    }
}

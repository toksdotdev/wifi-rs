use connectivity::handlers::NetworkXmlProfileHandler;
use connectivity::{
    Network, WifiConnectionError, WifiError, WifiHotspot, WifiHotspotError, WifiInterface,
};
use platforms::WiFi;
use std::process::Command;

impl WifiHotspot for WiFi {
    fn create_hotspot(ssid: &str, password: &str) -> Result<bool, WifiHotspotError> {
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
            .map_err(|err| WifiHotspotError::CreationFailed)?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully changed"))
    }

    fn start_hotspot() -> Result<bool, WifiHotspotError> {
        if !Self::is_wifi_enabled() {
            if !Self::turn_on().map_err(|err| WifiHotspotError::Other { kind: err })? {
                return Err(WifiHotspotError::Other {
                    kind: WifiError::InterfaceFailedToOn,
                });
            }
        }

        let output = Command::new("netsh")
            .args(&["wlan", "start", "hostednetwork"])
            .output()
            .map_err(|err| WifiHotspotError::CreationFailed)?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("hosted network started"))
    }

    fn stop_hotspot() -> Result<bool, WifiHotspotError> {
        let output = Command::new("netsh")
            .args(&["wlan", "stop", "hostednetwork"])
            .output()
            .map_err(|err| WifiHotspotError::CreationFailed)?;

        return Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("hosted network stopped"));
    }
}

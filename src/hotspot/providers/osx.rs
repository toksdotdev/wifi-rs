use hotspot::{WifiHotspot, WifiHotspotError};
use connectivity::{Connectivity, WifiConnectionError};
use platforms::WiFi;
use std::process::Command;

/// Configuration for a wireless hotspot.
pub struct HotspotConfig {}

/// Wireless hotspot functionality for a wifi interface.
impl WifiHotspot for WiFi {}

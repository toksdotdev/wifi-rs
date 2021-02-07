use crate::{hotspot::WifiHotspot, platforms::WiFi};

/// Configuration for a wireless hotspot.
#[allow(dead_code)]
pub struct HotspotConfig;

/// Wireless hotspot functionality for a wifi interface.
impl WifiHotspot for WiFi {}

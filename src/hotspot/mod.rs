mod providers;

use platforms::{WifiError, WifiInterface};
use std::fmt;

#[derive(Debug)]
pub enum WifiHotspotError {
  CreationFailed,
  Other { kind: WifiError },
}

/// Adds support for wifi hotspot functionality
pub trait WifiHotspot: fmt::Debug + WifiInterface {
  /// Creates wifi hotspot service for host machine. This only creats the wifi network,
  /// and isn't responsible for initiating the serving of the wifi network process.
  /// To begin serving the hotspot, use ```start_hotspot()```.
  fn create_hotspot(ssid: &str, password: &str) -> Result<bool, WifiHotspotError> {
    unimplemented!();
  }

  /// Start serving publicly an already created wifi hotspot.
  fn start_hotspot() -> Result<bool, WifiHotspotError> {
    unimplemented!();
  }

  /// Stop serving a wifi network.
  ///
  /// > All users connected will automatically be disconnected.
  fn stop_hotspot() -> Result<bool, WifiHotspotError> {
    unimplemented!();
  }
}

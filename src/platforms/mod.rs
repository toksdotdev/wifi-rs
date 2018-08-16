#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "osx")]
mod osx;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use self::linux::{Connection, Linux as WiFi};
#[cfg(target_os = "osx")]
pub use self::osx::{Connection, Osx as WiFi};
#[cfg(target_os = "windows")]
pub use self::windows::{Connection, Windows as WiFi};

use std::{fmt, io};

/// Configuration for a wifi network.
#[derive(Debug, Clone)]
pub struct Config<'a> {
  /// The interface the wifi module is situated.
  pub interface: Option<&'a str>,
}

#[derive(Debug)]
pub enum WifiError {
  // The specified wifi  is currently disabled. Try switching it on.
  WifiDisabled,
  /// The wifi interface interface failed to switch on.
  #[cfg(target_os = "windows")]
  InterfaceFailedToOn,
  /// IO Error occurred.
  IoError(io::Error),
}

/// Wifi interface for an operating system.
/// This provides basic functionalities for wifi interface.
pub trait WifiInterface: fmt::Debug {
  /// Check if the wifi interface on host machine is enabled.
  fn is_wifi_enabled() -> Result<bool, WifiError> {
    unimplemented!();
  }

  /// Turn on the wifi interface of host machine.
  fn turn_on() -> Result<(), WifiError> {
    unimplemented!();
  }

  /// Turn off the wifi interface of host machine.
  fn turn_off() -> Result<(), WifiError> {
    unimplemented!();
  }
}

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
  pub interface: Option<&'a str>,
}

#[derive(Debug)]
pub enum WifiError {
  // OsNotSupported,
  InterfaceDisabled,

  #[cfg(target_os = "windows")]
  InterfaceFailedToOn,

  IoError(io::Error),
}

pub trait WifiInterface: fmt::Debug {
  /// Checks if the wifi interface on host machine is enables.
  fn is_wifi_enabled() -> Result<bool, WifiError> {
    unimplemented!();
  }

  /// Turns on the wifi interface of host machine.
  fn turn_on() -> Result<(), WifiError> {
    unimplemented!();
  }

  // Turns off the wifi interface of host machine.
  fn turn_off() -> Result<(), WifiError> {
    unimplemented!();
  }
}

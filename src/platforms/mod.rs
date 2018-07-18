#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "osx")]
mod osx;

// #[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub(crate) use self::linux::{Connection, Linux};

#[cfg(target_os = "osx")]
pub use self::osx::OSX;

// #[cfg(target_os = "windows")]
pub use self::windows::Windows;

use std::{fmt, io};

#[derive(Debug)]
pub enum WifiError {
  OsNotSupported,
  InterfaceDisabled,
  InterfaceFailedToOn,
  IoError(io::Error),
}

pub trait WifiInterface: fmt::Debug {
  /// Checks if the wifi interface on host machine is enables.
  fn is_wifi_enabled() -> bool;

  /// Turns on the wifi interface of host machine.
  fn turn_on() -> Result<bool, WifiError>;

  // Turns off the wifi interface of host machine.
  fn turn_off() -> Result<bool, WifiError>;
}

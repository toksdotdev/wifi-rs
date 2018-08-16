#![allow(dead_code)]

mod connectivity;
#[cfg(target_os = "windows")]
mod handler;
mod hotspot;
mod platforms;

/// Pre-requisite module for `Connectivity`, `Hotspot` functionality.
pub mod prelude {
  pub use connectivity::*;
  pub use hotspot::*;
}

pub use platforms::WiFi;

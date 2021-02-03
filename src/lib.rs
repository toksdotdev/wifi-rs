mod connectivity;
mod hotspot;
mod platforms;

/// Pre-requisite module for `Connectivity`, `Hotspot` functionality.
pub mod prelude {
  pub use crate::connectivity::*;
  pub use crate::hotspot::*;
  pub use crate::hotspot::providers::prelude::*;
  pub use crate::platforms::Config;
}

pub use crate::platforms::WiFi;

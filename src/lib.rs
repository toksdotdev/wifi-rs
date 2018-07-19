#![allow(dead_code)]

mod connectivity;
mod hotspot;
mod platforms;

pub mod prelude {
  pub use connectivity::*;
  pub use hotspot::*;
}

pub use platforms::*;

#[cfg(test)]
mod tests {
  use super::prelude::*;
  use super::*;

  #[test]
  fn connect_to_wifi_failed() {
    let config = Some(Config {
      interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    assert_eq!(wifi.connect("ssid", "password").unwrap(), false);
  }

  #[test]
  fn create_hotspot() {
    let hotspot_created = WiFi::create_hotspot("hello", "hi").unwrap();

    assert_eq!(hotspot_created, true);
  }
}

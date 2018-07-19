extern crate wifi_rs;
use self::wifi_rs::{prelude::*, Config, WiFi};

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

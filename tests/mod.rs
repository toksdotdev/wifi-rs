extern crate wifi_rs;
use self::wifi_rs::{prelude::*, WiFi};

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
  let config = Some(Config {
    interface: Some("wlo1"),
  });

  let mut wifi = WiFi::new(config);
  let hotspot_created = wifi.create_hotspot("hello", "hi", None).unwrap();

  assert_eq!(hotspot_created, true);
}

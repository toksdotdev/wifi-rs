#![allow(dead_code)]

mod connectivity;
mod hotspot;
mod platforms;

pub use connectivity::{Config, *};
pub use hotspot::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_wifi_failed() {
        // let config = Some(Config {
        //     interface: Some("wlo1"),
        // });

        // let wifi = WiFi::new("hello", config).unwrap();

        // assert_eq!(wifi.connect("password").unwrap(), false);
    }
}

#![allow(dead_code)]

mod connectivity;
pub use connectivity::{profile_network::ProfileNetwork as WiFi, Config};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_wifi_failed() {
        let config = Some(Config {
            interface: Some("wlo1"),
        });

        let wifi = WiFi::new("hello", config).unwrap();

        assert_eq!(wifi.connect("password").unwrap(), false);
    }
}

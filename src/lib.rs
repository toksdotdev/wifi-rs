#![allow(dead_code)]

mod connectivity;
pub use connectivity::{profile_network::ProfileNetwork as WiFi, *};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_wifi_failed() {
        let wifi = WiFi::new("hello").unwrap();
        assert_eq!(wifi.connect("password"), false);
    }
}

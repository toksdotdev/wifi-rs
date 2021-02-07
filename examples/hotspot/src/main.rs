use std::io;
use wifi_rs::{prelude::*, WiFi};

fn main() -> Result<(), io::Error> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);
    let config = HotspotConfig::new(Some(HotspotBand::Bg), Some(Channel::One));

    wifi.create_hotspot("test-hotspot", "password", Some(&config));

    Ok(())
}

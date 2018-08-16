mod connectivity;
mod hotspot;
mod platforms;

use connectivity::{Connectivity, WifiConnectionError};
use platforms::{Config, WiFi};

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    match wifi.connect("AndroidAPSD22", "belm4235") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successfull."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    Ok(())
}

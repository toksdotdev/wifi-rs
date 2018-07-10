# WiFi-rs

A rust crate to interface and manage Wi-Fi networks.

## How it works

- The command creates a new connection and then activates it on a device.
- This is a command-line counterpart of clicking an SSID in a GUI client.
- The command always creates a new connection and thus it is mainly useful for connecting to new Wi-Fi networks.
- If a connection for the network already exists, it is better to bring up (activate) the existing connection as follows: `WiFi::connection_up("SSID")`.

## Currently supported network types

Note that only **open**, **WEP** and **WPA-PSK** networks are supported at the moment.

> It is also supposed that IP configuration is obtained via DHCP.

## Supported Operating Systems

- Windows
- Linux
- OSx

## Example

```RUST
extern crate wifi_rs;
use wifi_rs::{WiFi, Config, WifiConnectionError};

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"), // interface : None would default to `wlan0`.
    });

    let wifi = WiFi::new("AndroidAPSD", config)?;

    match wifi.connect("belm4235") {
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
```

## To do

- [x] Bundle windows profile sample as literals
- [x] Add support for Windows
- [x] Add support for linux
- [x] Add disconnect feature
- [x] Use `tempfile` crate on windows to generate windows profile temporary file.
- [ ] Add support for OSX
- [ ] Fix the implementation for `check_if_wifi_is_enabled` for windows.
- [ ] Add get network type feature
- [ ] Add create hotspot functionality
- [ ] Write documentation
- [ ] Write tests
- [ ] Add multi-threaded support

# Contribution

Any feature you feel is missing, why not send in a Pull Request, and let's help make this project better. Or are there any bugs, kindly create an issue, so we could work towards fixing it.

# Support

Love this project, and you'll love to buy me a coffee, kindly visit:

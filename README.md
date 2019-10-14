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
- MacOS

## Example

```RUST
extern crate wifi_rs;
use wifi_rs::prelude::*;
use wifi_rs::{WiFi, Config};

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
```

## Features

### Windows

- [x] Support for Windows.
- [x] Bundle windows profile sample as literals.
- [x] Add hotspot functionality.
- [x] Use `tempfile` crate on windows to generate windows profile temporary file.
- [x] Fix the implementation for `is_wifi_enabled` for windows.
- [x] Add implementation for WifiInterface trait.
- [ ] Add get network type feature.

### Linux

- [x] Support for linux.
- [x] Add disconnect feature.
- [x] Add hotspot functionality.
- [ ] Add get network type feature.

### MacOS

- [x] Add support for MacOS.
- [ ] Add get network type feature.
- [ ] Add hotspot functionality.

### General

- [x] Return detailed error messages.
- [x] Write documentation.
- [ ] Update `wifi-CLI` with recent updates.
- [ ] Write tests.

## Contribution

Any feature you feel is missing, why not send in a Pull Request, and let's help make this project better. Or are there any bugs, kindly create an issue, so we could work towards fixing it.

## Support

Love this project, please feel free to buy me a coffee below:
<br>
<a href="https://www.buymeacoffee.com/tnkemdilim" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

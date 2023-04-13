# WiFi-rs

A rust crate to interface and manage Wi-Fi networks.

This is a command-line counterpart of managing networks instead of via a GUI.

## Features

- Connect to a WiFi (`Windows`, `Linux`, `MacOS`).
- Disconnect from a WiFi network (`Windows`, `Linux`, `MacOS`).
- Create hotspot (`Windows`, `Linux`).

## Currently supported network types

Note that only **open**, **WEP** and **WPA-PSK** networks are supported at the moment.

> It is also supposed that IP configuration is obtained via DHCP.

## Supported Operating Systems

- Windows
- Linux
- MacOS

## Example

```Rust
use wifi_rs::{prelude::*, WiFi};

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    match wifi.connect("AndroidAPSD22", "belm4235") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successful."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    Ok(())
}
```

## Todos

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
- [ ] Add hotspot functionality.
- [ ] Add get network type feature.

### General

- [x] Return detailed error messages.
- [x] Write documentation.
- [x] Update `wifi-CLI` with recent updates.

## Contribution

Any feature you feel is missing, why not send in a Pull Request, and let's help make this project better. Or if there are any bugs, kindly create an issue, so we could work together towards fixing it.

## Support

Love this project, please show some love by starring the project 😃.

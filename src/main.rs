mod connectivity;
mod hotspot;
mod platforms;

use std::env::args;
use crate::connectivity::{Connectivity, WifiConnectionError};
use crate::platforms::{Config, WiFi};

use std::process::{Command, Stdio};
use std::str;

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    match wifi.connect("TP-Link_CEE2", "05Bs@sdfvn") {
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

    let output = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("nothing");
    // nmcli --fields RATE device wifi list
    let awk = Command::new("awk")
        .arg(r"/\*/{if (NR!=1) {print $6}}")
        .stdin(Stdio::from(output.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let result = awk.wait_with_output().unwrap();
    let result = str::from_utf8(&result.stdout).unwrap();
    println!("{}Mbit/s", result);




    Ok(())
}

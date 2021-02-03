extern crate clap;
extern crate wifi_rs;

use clap::{App, Arg};
use std::io;
use wifi_rs::WiFi;
use wifi_rs::prelude::*;

fn main() -> Result<(), io::Error> {
    let matches = App::new("Wi-Fi")
        .version("0.0.1")
        .author("Tochukwu Nkemdilim")
        .about("Connect to a Wi-Fi network 🎉")
        .arg(
            Arg::with_name("ssid")
                .short("s")
                .long("ssid")
                .multiple(false)
                .required(true)
                .takes_value(true)
                .help("SSID of wireless network."),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .multiple(false)
                .required(true)
                .takes_value(true)
                .help("Password of the wireless network."),
        )
        .arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .multiple(false)
                .default_value("wlan0")
                .takes_value(true)
                .help("Wireless interface to connect through."),
        )
        .get_matches();

    // Get Password
    let password = matches.value_of("password").unwrap();

    // Get SSID
    let ssid = matches.value_of("ssid").unwrap();

    // Get Wireless Interface
    let interface = matches.value_of("interface").unwrap();

    let config = Some(Config {
        interface: Some(interface),
    });

    // let wifi = WiFi::new(ssid)?;
    let mut wifi = WiFi::new(config);
    println!("Connection Status: {:?}", wifi.connect(ssid, password));

    Ok(())
}
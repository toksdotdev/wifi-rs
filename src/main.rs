use std::io;
mod connectivity;
pub use connectivity::profile_network::ProfileNetwork as WiFi;


fn main() -> Result<(), io::Error> {
    let mut wifi = WiFi::new("AndroidAPS")?;

    println!("{}", wifi.connect("belm4235"));

    Ok(())
}
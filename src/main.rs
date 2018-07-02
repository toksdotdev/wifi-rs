mod connectivity;
pub use connectivity::{profile_network::ProfileNetwork as WiFi, NetworkError};

fn main() -> Result<(), NetworkError> {
    let wifi = WiFi::new("AndroidAPSD")?;
    println!("{}", wifi.connect("belm4235"));

    Ok(())
}

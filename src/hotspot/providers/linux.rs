use crate::hotspot::{WifiHotspot, WifiHotspotError};
use crate::platforms::WiFi;
use std::fmt;
use std::process::Command;

/// Name of the group upon which the hotspot would be created.
/// This name must be fixed, in order to still interact with
/// the same hotspot previosuly created.
const HOTSPOT_GROUP: &'static str = "Hotspot";

/// Configuration for a wireless hotspot.
pub struct HotspotConfig {
    /// The band tor broadcast network on.
    band: Option<HotspotBand>,
    /// The channel to broadcast network on.
    channel: Option<Channel>,
}

#[allow(dead_code)]
#[derive(Debug)]
/// Band type of wireless hotspot.
pub enum HotspotBand {
    /// Band `A`
    A,
    /// Band `BG`
    Bg,
}

/// Channel to broadcast wireless hotspot on.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Channel {
    /// Channel 1
    One = 1,
    /// Channel 2
    Two = 2,
    /// Channel 3
    Three = 3,
    /// Channel 4
    Four = 4,
    /// Channel 5
    Five = 5,
    /// Channel 6
    Six = 6,
}

/// Wireless hotspot functionality for a wifi interface.
impl WifiHotspot for WiFi {
    /// Creates wireless hotspot service for host machine. This only creats the wifi network,
    /// and isn't responsible for initiating the serving of the wifi network process.
    /// To begin serving the hotspot, use ```start_hotspot()```.
    fn create_hotspot(
        &mut self,
        ssid: &str,
        password: &str,
        configuration: Option<&HotspotConfig>,
    ) -> Result<bool, WifiHotspotError> {
        let mut command = vec![
            "device".to_string(),
            "wifi".to_string(),
            "ifname".to_string(),
            self.interface.to_string(),
            "hotspot".to_string(),
            "con-name".to_string(),
            HOTSPOT_GROUP.to_string(),
            "ssid".to_string(),
            ssid.to_string(),
            "password".to_string(),
            password.to_string(),
        ];

        let mut cmd = generate_command_param_from_config(configuration);
        command.append(&mut cmd);

        let output = Command::new("nmcli")
            .args(&command)
            .output()
            .map_err(|_err| WifiHotspotError::CreationFailed)?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("successfully activated"))
    }

    /// Start serving publicly an already created wireless hotspot.
    fn start_hotspot() -> Result<bool, WifiHotspotError> {
        let output = Command::new("nmcli")
            .args(&["con", "up", HOTSPOT_GROUP])
            .output()
            .map_err(|err| WifiHotspotError::FailedToStop(err))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("Connection successfully activated"))
    }

    /// Stop serving a wireless network.
    ///
    /// **NOTE: All users connected will automatically be disconnected.**
    fn stop_hotspot(&mut self) -> Result<bool, WifiHotspotError> {
        let output = Command::new("nmcli")
            .args(&["con", "down", HOTSPOT_GROUP])
            .output()
            .map_err(|err| WifiHotspotError::FailedToStop(err))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("Connection 'Hotspot' successfully deactivated"))
    }
}

/// Generate vector values from a given config paramenters.
fn generate_command_param_from_config(configuration: Option<&HotspotConfig>) -> Vec<String> {
    let mut command = vec![];

    if let Some(ref config) = configuration {
        if let Some(ref band) = config.band {
            let band = format!("{}", band);
            let mut a = vec!["band".to_string(), band];
            command.append(&mut a);
        }

        if let Some(channel) = config.channel {
            let channel = format!("{}", channel as u8);
            command.append(&mut vec!["channel".to_string(), channel]);
        }
    };

    command
}

impl fmt::Display for HotspotBand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HotspotBand::A => "a",
                HotspotBand::Bg => "bg",
            }
        )
    }
}

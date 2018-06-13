mod linux;
mod windows;

#[cfg(target_os = "windows")]
pub(crate) use self::windows::Windows as Machine;

#[cfg(target_os = "linux")]
pub(crate) use self::linux::Linux as Machine;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub(crate) use self::windows::Windows as Machine;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use self::linux::Linux as Machine;

#[cfg(target_os = "osx")]
mod osx;
#[cfg(target_os = "osx")]
pub(crate) use self::osx::OSX as Machine;

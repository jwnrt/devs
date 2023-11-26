//! This module deals with handles to connected devices.

use std::path::PathBuf;

/// Handle for a connected device.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// Path in the Linux sysfs representing the device.
    #[cfg(target_os = "linux")]
    sysfs_path: PathBuf,
}

#[cfg(target_os = "linux")]
impl From<PathBuf> for Device {
    fn from(sysfs_path: PathBuf) -> Self {
        Self { sysfs_path }
    }
}

impl Device {
    /// Get the path to the Linux sysfs entry for this device.
    #[cfg(target_os = "linux")]
    pub fn sysfs_path(&self) -> &PathBuf {
        &self.sysfs_path
    }
}

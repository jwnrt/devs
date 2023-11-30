//! This module deals with handles to connected devices.

use std::hash::{Hash, Hasher};
use std::path::PathBuf;

/// Handle for a connected device.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// Path in the Linux sysfs representing the device.
    #[cfg(target_os = "linux")]
    pub(crate) sysfs_path: PathBuf,
}

#[cfg(target_os = "linux")]
impl From<PathBuf> for Device {
    fn from(sysfs_path: PathBuf) -> Self {
        Self { sysfs_path }
    }
}

#[cfg(target_os = "linux")]
impl Hash for Device {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sysfs_path.hash(state);
    }
}

impl Device {
    /// Get the path to the Linux sysfs entry for this device.
    #[cfg(target_os = "linux")]
    pub fn sysfs_path(&self) -> &PathBuf {
        &self.sysfs_path
    }

    /// Find the parent of this device from a collection of devices.
    pub fn parent<'a>(&self, devices: &'a [Device]) -> Option<&'a Device> {
        let others = devices.iter().filter(|device| device != &self);
        let ancestors = others.filter(|device| self.sysfs_path.starts_with(&device.sysfs_path));
        ancestors.max_by_key(|device| device.sysfs_path.components().count())
    }

    /// Iterator over the descendants of this device in a collection of devices.
    pub fn descendants<'a>(&'a self, devices: &'a [Device]) -> impl Iterator<Item = &'a Device> {
        let others = devices.iter().filter(move |device| device != &self);
        others.filter(move |device| device.sysfs_path.starts_with(&self.sysfs_path))
    }
}

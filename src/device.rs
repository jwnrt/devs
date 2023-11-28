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

    /// Find the parent of this device from a collection of devices.
    pub fn parent<'a>(&self, devices: &'a [Device]) -> Option<&'a Device> {
        let others = devices.iter().filter(|device| device != &self);
        let ancestors = others.filter(|device| self.sysfs_path.starts_with(&device.sysfs_path));
        ancestors.max_by_key(|device| device.sysfs_path.components().count())
    }

    /// Iterator over the children of this device in a collection of devices.
    pub fn children<'a>(&'a self, devices: &'a [Device]) -> impl Iterator<Item = &'a Device> {
        let others = devices.iter().filter(move |device| device != &self);
        others.filter(move |device| device.sysfs_path.starts_with(&self.sysfs_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scanner;
    use std::io;

    /// Check that asking for devices' parents does not error.
    ///
    /// Does not check that the parents are correct.
    #[test]
    fn parents() -> io::Result<()> {
        let devices = Scanner::scan()?;
        let parents: Vec<_> = devices
            .iter()
            .filter_map(|dev| Some((dev.sysfs_path(), dev.parent(&devices)?.sysfs_path())))
            .collect();
        println!("{parents:#?}");
        Ok(())
    }

    /// Check that asking for devices' children does not error.
    ///
    /// Does not check that the children are correct.
    #[test]
    fn children() -> io::Result<()> {
        let devices = Scanner::scan()?;
        let children: Vec<_> = devices
            .iter()
            .map(|dev| {
                (
                    dev.sysfs_path(),
                    dev.children(&devices)
                        .map(Device::sysfs_path)
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        println!("{children:#?}");
        Ok(())
    }
}

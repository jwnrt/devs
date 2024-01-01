//! This module deals with handles to connected devices.

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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::testing;

    use super::*;

    /// Check that asking for devices' parents does not error.
    #[test]
    fn parents() {
        let devices: Vec<Device> = testing::mock_dev_paths().map(Into::into).collect();

        // Collect each device's parent into a `BTreeSet` by only its `sysfs_path`.
        // The set allows us to compare without worrying about the order of discovery.
        let parents: BTreeSet<_> = devices
            .iter()
            .map(|dev| {
                let parent = dev.parent(&devices).map(|parent| parent.sysfs_path.clone());
                (dev.sysfs_path.clone(), parent)
            })
            .collect();

        // These are the expected parents for each device by its path relative to
        // `/sys/devices/`.
        let expected_parents: BTreeSet<_> = [
            ("device_0", None),
            ("device_1", None),
            ("device_1/device_2", Some("device_1")),
            ("device_1/device_2/device_3", Some("device_1/device_2")),
            ("device_1/device_2/device_4", Some("device_1/device_2")),
            ("device_5", None),
            ("device_6", None),
            ("device_7", None),
        ]
        .into_iter()
        .map(|(path, parent)| {
            (
                testing::prefix_dev_path(path),
                parent.map(testing::prefix_dev_path),
            )
        })
        .collect();

        assert_eq!(parents, expected_parents);
    }

    /// Check that asking for devices' descendants does not error.
    #[test]
    fn descendants() {
        let devices: Vec<Device> = testing::mock_dev_paths().map(Into::into).collect();

        // Collect each device's descendants into a `BTreeSet` by only its `sysfs_path`.
        // The set allows us to compare without worrying about the order of discovery.
        let descendants: BTreeSet<_> = devices
            .iter()
            .map(|dev| {
                let descendants = dev
                    .descendants(&devices)
                    .map(|descendant| descendant.sysfs_path.clone())
                    .collect::<BTreeSet<_>>();
                (dev.sysfs_path.clone(), descendants)
            })
            .collect();

        // These are the expected descendants for each device by their paths relative to
        // `/sys/devices/`.
        let expected_descendants: BTreeSet<_> = [
            ("device_0", vec![]),
            (
                "device_1",
                vec![
                    "device_1/device_2",
                    "device_1/device_2/device_3",
                    "device_1/device_2/device_4",
                ],
            ),
            (
                "device_1/device_2",
                vec!["device_1/device_2/device_3", "device_1/device_2/device_4"],
            ),
            ("device_1/device_2/device_3", vec![]),
            ("device_1/device_2/device_4", vec![]),
            ("device_5", vec![]),
            ("device_6", vec![]),
            ("device_7", vec![]),
        ]
        .into_iter()
        .map(|(path, descendants)| {
            let descendants = descendants
                .into_iter()
                .map(testing::prefix_dev_path)
                .collect::<BTreeSet<_>>();
            (testing::prefix_dev_path(path), descendants)
        })
        .collect();

        assert_eq!(descendants, expected_descendants);
    }
}

//! Module of internal test utilities.
//!
//! Only code used by multiple modules needs to go here.

use std::path::PathBuf;

/// Returns the list of device paths in the mock sysfs.
///
/// This list will be kept up to date with the mock directory in this repo,
/// otherwise the `scan` tests will fail.
///
/// Unit tests should use this device list even if not getting their devices
/// from a scan.
pub fn mock_dev_paths() -> impl Iterator<Item = PathBuf> {
    [
        "device_0",
        "device_1",
        "device_1/device_2",
        "device_1/device_2/device_3",
        "device_1/device_2/device_4",
        "device_5",
        "device_6",
        "device_7",
    ]
    .into_iter()
    .map(prefix_dev_path)
}

/// Prefix a device path relative to `/sys/devices/` to give an absolute path.
pub fn prefix_dev_path(dev_path: &str) -> PathBuf {
    let root_path: PathBuf = [crate::SYSFS_PATH, "devices"].iter().collect();
    root_path.join(dev_path)
}

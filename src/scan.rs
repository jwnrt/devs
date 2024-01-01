//! Library for finding connected devices.

#[cfg(not(target_os = "linux"))]
core::compile_error!("unsupported platform");

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::Device;

/// Connected device scanner.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scanner {
    /// Paths in `/sys/devices` discovered by the scanner.
    device_paths: HashSet<PathBuf>,
}

impl Scanner {
    /// Scan the Linux sysfs for devices.
    pub fn scan() -> io::Result<Vec<Device>> {
        let mut scanner = Scanner::default();

        scanner.scan_bus()?;
        scanner.scan_class()?;
        scanner.scan_block()?;

        let devices = scanner.device_paths.into_iter().map(Into::into).collect();

        Ok(devices)
    }

    /// Scan the `/sys/bus/` directory for devices and print their sysfs paths.
    fn scan_bus(&mut self) -> io::Result<()> {
        let path: PathBuf = [crate::SYSFS_PATH, "bus"].iter().collect();

        for subsys in fs::read_dir(path)? {
            let devices = subsys?.path().join("devices");

            for device in fs::read_dir(&devices)? {
                let device_link = device?.path().read_link()?;
                let device_path = devices.join(device_link).canonicalize()?;

                self.device_paths.insert(device_path);
            }
        }

        Ok(())
    }

    /// Scan the `/sys/class/` directory for devices and print their sysfs paths.
    fn scan_class(&mut self) -> io::Result<()> {
        let path: PathBuf = [crate::SYSFS_PATH, "class"].iter().collect();

        for class in fs::read_dir(path)? {
            let devices = class?.path();

            for device in fs::read_dir(&devices)? {
                let device_path = device?.path();

                if !device_path.is_symlink() {
                    continue;
                }

                let device_link = device_path.read_link()?;
                let device_path = devices.join(device_link).canonicalize()?;

                self.device_paths.insert(device_path);
            }
        }

        Ok(())
    }

    /// Scan the `/sys/block/` directory for devices and print their sysfs paths.
    fn scan_block(&mut self) -> io::Result<()> {
        let path: PathBuf = [crate::SYSFS_PATH, "block"].iter().collect();

        for device in fs::read_dir(&path)? {
            let device_link = device?.path().read_link()?;
            let device_path = path.join(device_link).canonicalize()?;

            self.device_paths.insert(device_path);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::testing;

    use super::*;

    /// Check scanning does not error.
    ///
    /// Does not assert anything about the results of the scan.
    #[test]
    fn scan() {
        let devices = Scanner::scan().expect("failed to scan");

        // Collect the device paths and compare with expected paths under `/sys/devices/`.
        // We use a `BTreeSet` so we don't have to worry about the order of discovery
        // when we compare them later.
        let device_paths: BTreeSet<_> = devices.into_iter().map(|dev| dev.sysfs_path).collect();
        let expected_paths: BTreeSet<_> = testing::mock_dev_paths().collect();

        assert_eq!(device_paths, expected_paths);
    }
}

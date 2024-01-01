//! Library for finding connected devices.

#[cfg(not(target_os = "linux"))]
core::compile_error!("unsupported platform");

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::Device;

const SYSFS_PATH: &str = if cfg!(test) {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sys")
} else {
    "/sys"
};

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

        let devices = scanner
            .device_paths
            .into_iter()
            .map(|path| path.into())
            .collect();

        Ok(devices)
    }

    /// Scan the `/sys/bus/` directory for devices and print their sysfs paths.
    fn scan_bus(&mut self) -> io::Result<()> {
        let path: PathBuf = [SYSFS_PATH, "bus"].iter().collect();

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
        let path: PathBuf = [SYSFS_PATH, "class"].iter().collect();

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
        let path: PathBuf = [SYSFS_PATH, "block"].iter().collect();

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

        let root_path: PathBuf = [SYSFS_PATH, "devices"].iter().collect();
        let expected_paths: BTreeSet<_> = [
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
        .map(|path| root_path.join(path))
        .collect();

        assert_eq!(device_paths, expected_paths);
    }

    /// Check that asking for devices' parents does not error.
    #[test]
    fn parents() {
        let devices = Scanner::scan().expect("failed to scan");

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
        let root_path: PathBuf = [SYSFS_PATH, "devices"].iter().collect();
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
            let parent = parent.map(|parent| root_path.join(parent));
            (root_path.join(path), parent)
        })
        .collect();

        assert_eq!(parents, expected_parents);
    }

    /// Check that asking for devices' descendants does not error.
    #[test]
    fn descendants() {
        let devices = Scanner::scan().expect("failed to scan");

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
        let root_path: PathBuf = [SYSFS_PATH, "devices"].iter().collect();
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
                .map(|descendant| root_path.join(descendant))
                .collect::<BTreeSet<_>>();
            (root_path.join(path), descendants)
        })
        .collect();

        assert_eq!(descendants, expected_descendants);
    }
}

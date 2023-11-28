//! Library for finding connected devices.

#[cfg(not(target_os = "linux"))]
core::compile_error!("unsupported platform");

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

use crate::Device;

/// Connected device scanner.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scanner {
    devices: HashSet<Device>,
}

impl Scanner {
    /// Scan the Linux sysfs for devices.
    pub fn scan() -> io::Result<Vec<Device>> {
        let mut scanner = Scanner::default();

        scanner.scan_bus()?;
        scanner.scan_class()?;
        scanner.scan_block()?;

        let devices = scanner.devices.into_iter().collect();
        Ok(devices)
    }

    /// Scan the `/sys/bus/` directory for devices and print their sysfs paths.
    fn scan_bus(&mut self) -> io::Result<()> {
        const PATH: &str = "/sys/bus";

        for subsys in fs::read_dir(PATH)? {
            let devices = subsys?.path().join("devices");

            for device in fs::read_dir(&devices)? {
                let device_link = device?.path().read_link()?;
                let device_path = devices.join(device_link).canonicalize()?;

                self.devices.insert(device_path.into());
            }
        }

        Ok(())
    }

    /// Scan the `/sys/class/` directory for devices and print their sysfs paths.
    fn scan_class(&mut self) -> io::Result<()> {
        const PATH: &str = "/sys/class";

        for class in fs::read_dir(PATH)? {
            let devices = class?.path();

            for device in fs::read_dir(&devices)? {
                let device_path = device?.path();

                if !device_path.is_symlink() {
                    continue;
                }

                let device_link = device_path.read_link()?;
                let device_path = devices.join(device_link).canonicalize()?;

                self.devices.insert(device_path.into());
            }
        }

        Ok(())
    }

    /// Scan the `/sys/block/` directory for devices and print their sysfs paths.
    fn scan_block(&mut self) -> io::Result<()> {
        const PATH: &str = "/sys/block";

        for device in fs::read_dir(PATH)? {
            let device_link = device?.path().read_link()?;
            let device_path = Path::new(PATH).join(device_link).canonicalize()?;

            self.devices.insert(device_path.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check scanning does not error.
    ///
    /// Does not assert anything about the results of the scan.
    #[test]
    fn scan() -> io::Result<()> {
        let devices = Scanner::scan()?;
        let devices: Vec<_> = devices.iter().map(Device::sysfs_path).collect();
        println!("{devices:#?}");
        Ok(())
    }
}

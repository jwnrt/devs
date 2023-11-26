//! Library for finding connected devices.

#[cfg(not(target_os = "linux"))]
core::compile_error!("unsupported platform");

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scanner {
    device_paths: Vec<PathBuf>,
}

impl Scanner {
    /// Scan the Linux sysfs for devices.
    pub fn scan() -> io::Result<Vec<PathBuf>> {
        let mut scanner = Scanner::default();

        scanner.scan_bus()?;
        scanner.scan_class()?;
        scanner.scan_block()?;

        Ok(scanner.device_paths)
    }

    /// Scan the `/sys/bus/` directory for devices and print their sysfs paths.
    fn scan_bus(&mut self) -> io::Result<()> {
        const PATH: &str = "/sys/bus";

        for subsys in fs::read_dir(PATH)? {
            let devices = subsys?.path().join("devices");

            for device in fs::read_dir(&devices)? {
                let device_link = device?.path().read_link()?;
                let device = devices.join(device_link).canonicalize()?;

                self.device_paths.push(device);
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
                let device = devices.join(device_link).canonicalize()?;

                self.device_paths.push(device);
            }
        }

        Ok(())
    }

    /// Scan the `/sys/block/` directory for devices and print their sysfs paths.
    fn scan_block(&mut self) -> io::Result<()> {
        const PATH: &str = "/sys/block";

        for device in fs::read_dir(PATH)? {
            let device_link = device?.path().read_link()?;
            let device = Path::new(PATH).join(device_link).canonicalize()?;

            self.device_paths.push(device);
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
        println!("{devices:#?}");
        Ok(())
    }
}

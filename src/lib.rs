//! Linux device discovery library.
//!
//! At the moment it can enumerate connected devices and provide their sysfs
//! device paths. In the future, it may have more features for filtering and
//! inspecting devices.
//!
//! # Example
//!
//! ```rust
//! let devices = devs::scan().expect("failed to scan for devices");
//! assert!(devices.len() > 0);
//! ```

pub(crate) mod device;
pub(crate) mod scan;

#[cfg(test)]
pub(crate) mod testing;

use std::io;

#[doc(inline)]
pub use device::Device;

/// Path to Linux sysfs directory.
///
/// The path is changed for tests so we get consistent results regardless of
/// where the tests are run.
pub(crate) const SYSFS_PATH: &str = if cfg!(test) {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sys")
} else {
    "/sys"
};

/// Scan the Linux sysfs for devices.
pub fn scan() -> io::Result<Vec<Device>> {
    scan::Scanner::scan()
}

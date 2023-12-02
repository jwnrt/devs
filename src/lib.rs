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

use std::io;

#[doc(inline)]
pub use device::Device;

/// Scan the Linux sysfs for devices.
pub fn scan() -> io::Result<Vec<Device>> {
    scan::Scanner::scan()
}

//! File system utilities
//!
//! This module contains utilty methods for working with the file system.
//! This includes reading/writingt to files, and working with directories.

mod file;
mod open_options;

pub use file::File;
pub use open_options::OpenOptions;

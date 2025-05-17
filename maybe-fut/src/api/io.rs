//! Traits, helpers, and type definitions for core I/O functionality.
//!
//! The `io` module contains a number of common things you'll need when doing input and output.

mod read;
mod seek;
mod write;

pub use self::read::Read;
pub use self::seek::Seek;
pub use self::write::Write;

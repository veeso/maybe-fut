//! Traits, helpers, and type definitions for core I/O functionality.
//!
//! The `io` module contains a number of common things you'll need when doing input and output.
//!
//! Reference:
//!
//! - std: <https://doc.rust-lang.org/std/io/index.html>
//! - tokio: <https://docs.rs/tokio/latest/tokio/io/index.html>

mod buf_reader;
mod lines;
mod read;
mod seek;
mod split;
mod stderr;
mod stdin;
mod stdout;
mod write;

pub use self::buf_reader::{BufRead, BufReader};
pub use self::lines::Lines;
pub use self::read::Read;
pub use self::seek::Seek;
pub use self::split::Split;
pub use self::stderr::{Stderr, stderr};
pub use self::stdin::{Stdin, stdin};
pub use self::stdout::{Stdout, stdout};
pub use self::write::Write;

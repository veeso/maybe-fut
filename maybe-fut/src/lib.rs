#![crate_name = "maybe_fut"]
#![crate_type = "lib"]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # maybe-fut
//!
//! **Maybe-fut** is a Rust library that provides a way to export both a **sync** and an **async** API from the same codebase.
//! It allows you to write your code once and have it work in both synchronous and asynchronous contexts.
//!
//! This is achieved through a complex mechanism of **proc macros** and wrappers around `tokio` and `std` libraries.
//!
//! Maybe-fut provides its own type library, for `fs`, `io`, `net`, `sync` and `time` modules, which are designed to
//! use `std` or `tokio` types as needed. Mind that for compatibility reasons, the `io` module has been re-implemented from scratch.
//!
//! At runtime it checks whether the thread is running in a **sync** or **async** context and calls the appropriate function.
//! This allows you to write your code once and have it work in both synchronous and asynchronous contexts.
//!
//! In order to check whether the current context is synchronous or asynchronous, you can use the [`is_async_context`] function.
//!
//! This is a simple example of how it works:
//!
//! 1. Setup your logic to be exported using `maybe-fut` types:
//!
//!     ```rust
//!     use std::path::{Path, PathBuf};
//!
//!     use maybe_fut::fs::File;
//!
//!     struct FsClient {
//!         path: PathBuf,
//!     }
//!
//!     #[maybe_fut::maybe_fut(
//!         sync = SyncFsClient,
//!         tokio = TokioFsClient,
//!         tokio_feature = "tokio"
//!     )]
//!     impl FsClient {
//!         /// Creates a new `FsClient` instance.
//!         pub fn new(path: impl AsRef<Path>) -> Self {
//!             Self {
//!                 path: path.as_ref().to_path_buf(),
//!             }
//!         }
//!
//!         /// Creates a new file at the specified path.
//!         pub async fn create(&self) -> std::io::Result<()> {
//!             // Create a new file at the specified path.
//!             let file = File::create(&self.path).await?;
//!             file.sync_all().await?;
//!
//!             Ok(())
//!         }
//!     }
//!     ```
//!
//!     If you see there is an attribute macro there, called `maybe_fut`. This macro takes 3 arguments:
//!
//!     - `sync`: The name of the sync struct that will be generated.
//!     - `tokio`: The name of the async struct that will be generated.
//!     - `tokio_feature`: The name of the feature that will be used to enable the async struct.
//!
//! 2. Users can now access the public API exported from the library:
//!
//!     ```rust,ignore
//!     fn sync_main(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//!         println!("Running in sync mode");
//!
//!         let client = SyncFsClient::new(path);
//!         client.create()?;
//!
//!         Ok(())
//!     }
//!
//!     #[cfg(feature = "tokio")]
//!     async fn tokio_main(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//!         println!("Running in async mode");
//!
//!         let client = TokioFsClient::new(path);
//!         client.create().await?;
//!
//!         Ok(())
//!     }
//!     ```
//!
//! A full example can be found in the [examples](./maybe-fut/examples/) folder and can be run using the following command:
//!
//! ```bash
//! cargo run --example fs-client --features tokio-fs -- /tmp/test.txt
//! ```
//!
//! And the `maybe_fut` macro can be applied to traits as well, even combining generics:
//!
//! ```rust,ignore
//! use std::fmt::Display;
//!
//! #[derive(Debug, Clone, Copy)]
//! struct TestStruct<T: Sized + Copy + Display> {
//!     value: T,
//! }
//!
//! #[maybe_fut::maybe_fut(
//!     sync = SyncTestStruct,
//!     tokio = TokioTestStruct,
//!     tokio_feature = "tokio",
//! )]
//! impl<T> TestStruct<T>
//! where
//!     T: Sized + Copy + Display,
//! {
//!     /// Creates a new [`TestStruct`] instance.
//!     pub fn new(value: T) -> Self {
//!         Self { value }
//!     }
//!
//!     /// Get underlying value.
//!     pub fn value(&self) -> T {
//!         self.value
//!     }
//! }
//!
//! /// A trait to greet the user.
//! pub trait Greet {
//!     /// Greets the user with a message.
//!     fn greet(&self) -> String;
//!
//!     // Greets the user with a message asynchronously.
//!     fn greet_async(&self) -> impl Future<Output = String>;
//! }
//!
//! #[maybe_fut::maybe_fut(
//!     sync = SyncTestStruct,
//!     tokio = TokioTestStruct,
//!     tokio_feature = "tokio",
//! )]
//! impl<T> Greet for TestStruct<T>
//! where
//!     T: Sized + Copy + Display,
//! {
//!     fn greet(&self) -> String {
//!         format!("Hello, I'm {}", self.value)
//!     }
//!
//!     async fn greet_async(&self) -> String {
//!         format!("Hello, I'm {}", self.value)
//!     }
//! }
//!
//! #[cfg(feature = "tokio")]
//! {
//!     let test_struct = TokioTestStruct::new(42);
//!     test_struct.greet();
//!     test_struct.greet_async().await;
//! }
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-500.png"
)]

#[macro_use]
extern crate maybe_fut_io_derive;
#[macro_use]
extern crate maybe_fut_unwrap_derive;

// private api
mod api;
mod context;
mod macros;
mod rt;
mod unwrap;

// public api (api is exported at top-level)
// export maybe fut derive macro
pub use maybe_fut_derive::maybe_fut;

pub use self::api::*;
pub use self::context::is_async_context;
pub use self::rt::{SyncRuntime, block_on};
pub use self::unwrap::Unwrap;

# maybe-fut

<p align="center">
  <img src="/assets/images/logo.svg" alt="logo" width="256" height="256" />
</p>
<p align="center">Rust library to build totally interoperable async/sync Rust code</p>
<p align="center">
  <a href="https://docs.rs/maybe-fut" target="_blank">Documentation</a>
  ·
  <a href="https://crates.io/crates/maybe-fut" target="_blank">Crates.io</a>
</p>

<p align="center">Developed by <a href="https://veeso.me/">veeso</a>
<p align="center">Current version: 0.1.0 (2025/06/18)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/crates/l/maybe-fut.svg"
      alt="License-Apache-2.0/MIT"
  /></a>
  <a href="https://github.com/veeso/maybe-fut/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/maybe-fut?style=flat"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/maybe-fut"
    ><img
      src="https://img.shields.io/crates/d/maybe-fut.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/maybe-fut"
    ><img
      src="https://img.shields.io/crates/v/maybe-fut.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
  <a href="https://conventionalcommits.org">
    <img
      src="https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white"
      alt="conventional-commits"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/maybe-fut/actions"
    ><img
      src="https://github.com/veeso/maybe-fut/actions/workflows/cargo.yml/badge.svg"
      alt="Lib-CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/maybe-fut"
    ><img
      src="https://coveralls.io/repos/github/veeso/maybe-fut/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/maybe-fut"
    ><img
      src="https://docs.rs/maybe-fut/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [maybe-fut](#maybe-fut)
  - [Introduction](#introduction)
  - [Performance](#performance)
  - [Limitations](#limitations)
  - [Support the developer](#support-the-developer)
  - [Changelog](#changelog)
  - [License](#license)

---

## Introduction

**Maybe-fut** is a Rust library that provides a way to export both a **sync** and an **async** API from the same codebase. It allows you to write your code once and have it work in both synchronous and asynchronous contexts.

This is achieved through a complex mechanism of **proc macros** and wrappers around `tokio` and `std` libraries.

Maybe-fut provides its own type library, for `fs`, `io`, `net`, `sync` and `time` modules, which are designed to use `std` or `tokio` types as needed. Mind that for compatibility reasons, the `io` module has been re-implemented from scratch.

At runtime it checks whether the thread is running in a **sync** or **async** context and calls the appropriate function. This allows you to write your code once and have it work in both synchronous and asynchronous contexts.

This is a simple example of how it works:

1. Setup your logic to be exported using `maybe-fut` types:

    ```rust
    use std::path::{Path, PathBuf};

    use maybe_fut::fs::File;

    struct FsClient {
        path: PathBuf,
    }

    #[maybe_fut::maybe_fut(
        sync = SyncFsClient,
        tokio = TokioFsClient,
        tokio_feature = "tokio"
    )]
    impl FsClient {
        /// Creates a new `FsClient` instance.
        pub fn new(path: impl AsRef<Path>) -> Self {
            Self {
                path: path.as_ref().to_path_buf(),
            }
        }

        /// Creates a new file at the specified path.
        pub async fn create(&self) -> std::io::Result<()> {
            // Create a new file at the specified path.
            let file = File::create(&self.path).await?;
            file.sync_all().await?;

            Ok(())
        }
    }
    ```

    If you see there is an attribute macro there, called `maybe_fut`. This macro takes 3 arguments:

    - `sync`: The name of the sync struct that will be generated.
    - `tokio`: The name of the async struct that will be generated.
    - `tokio_feature`: The name of the feature that will be used to enable the async struct.

2. Users can now access the public API exported from the library:

    ```rust
    fn sync_main(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running in sync mode");

        let client = SyncFsClient::new(path);
        client.create()?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
    async fn tokio_main(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running in async mode");

        let client = TokioFsClient::new(path);
        client.create().await?;

        Ok(())
    }
    ```

A full example can be found in the [examples](./maybe-fut/examples/) folder and can be run using the following command:

```bash
cargo run --example fs-client --features tokio-fs -- /tmp/test.txt
```

And the `maybe_fut` macro can be applied to traits as well, even combining generics:

```rust
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct TestStruct<T: Sized + Copy + Display> {
    value: T,
}

#[maybe_fut::maybe_fut(
    sync = SyncTestStruct,
    tokio = TokioTestStruct,
    tokio_feature = "tokio",
)]
impl<T> TestStruct<T>
where
    T: Sized + Copy + Display,
{
    /// Creates a new [`TestStruct`] instance.
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Get underlying value.
    pub fn value(&self) -> T {
        self.value
    }
}

/// A trait to greet the user.
pub trait Greet {
    /// Greets the user with a message.
    fn greet(&self) -> String;

    // Greets the user with a message asynchronously.
    fn greet_async(&self) -> impl Future<Output = String>;
}

#[maybe_fut::maybe_fut(
    sync = SyncTestStruct,
    tokio = TokioTestStruct,
    tokio_feature = "tokio",
)]
impl<T> Greet for TestStruct<T>
where
    T: Sized + Copy + Display,
{
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.value)
    }

    async fn greet_async(&self) -> String {
        format!("Hello, I'm {}", self.value)
    }
}

#[cfg(feature = "tokio")]
{
    let test_struct = TokioTestStruct::new(42);
    test_struct.greet();
    test_struct.greet_async().await;
}
```

## Performance

As of now, the performance of `maybe-fut` is on par with the `tokio` and `std` libraries. The proc macro generates code that is optimized for both synchronous and asynchronous contexts, so there is no significant overhead when using it.

This overhead is negligible, and it has been benchmarked, as shown at `maybe-fut/benches/async_context.rs`, and the results are actually quite good.

```txt
is_async_context        time:   [3.3511 ns 3.3567 ns 3.3621 ns]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
```

What's the cost of a wrapper? Let's try to measure it by creating a file with `tokio::fs::File::create` and `maybe_fut::fs::File::create`:

```txt
tokio_create_file       time:   [11.529 µs 11.620 µs 11.715 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

maybe_fut_create_file   time:   [11.603 µs 11.696 µs 11.786 µs]
```

So yeah, the cost of the wrapper is a very little higher, but **the difference is negligible**.

## Limitations

Currently, there are some limitations with the proc macro, so the following features are still not supported:

- [ ] Builders (e.g. `fn foo(mut self) -> Self`)
- [ ] Derive of the inner type

---

## Support the developer

If you like **maybe-fut**, please consider a little donation 🥳

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Changelog

[View Changelog here](CHANGELOG.md)

---

## License

Licensed under MIT license ([SEE LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)

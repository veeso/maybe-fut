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
<p align="center">Current version: 0.1.0 WIP (08/03/2025)</p>

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
  - [Limitations](#limitations)
  - [Support the developer](#support-the-developer)
  - [Changelog](#changelog)
  - [License](#license)

---

## Introduction

**Maybe-fut** is a Rust library that provides a way to export both a **sync** and an **async** API from the same codebase. It allows you to write your code once and have it work in both synchronous and asynchronous contexts.

This is achieved through a complex mechanism of **proc macros** and wrappers around `tokio` and `std` libraries.

At runtime it checks whether the thread is running in a **sync** or **async** context and calls the appropriate function. This allows you to write your code once and have it work in both synchronous and asynchronous contexts.

This is a simple example of how it works:

1. Setup your logic to be exported using `maybe-fut` types:

    ```rust
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

View Changelog [here](CHANGELOG.md)

---

## License

Licensed under MIT license ([SEE LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)

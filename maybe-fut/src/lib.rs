#![crate_name = "maybe_fut"]
#![crate_type = "lib"]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # maybe-fut
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-500.png"
)]

// private api
mod api;
mod context;
mod macros;
mod sync;
mod unwrap;

// public api (api is exported at top-level)
pub use self::api::*;
pub use self::context::is_async_context;
pub use self::sync::SyncRuntime;
pub use self::unwrap::Unwrap;

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
mod macros;

// public api
pub mod fs;
pub mod sync;

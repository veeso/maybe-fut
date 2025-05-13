#![crate_name = "maybe_fut_derive"]
#![crate_type = "lib"]

//! # maybe-fut-derive
//!
//! A procedural macro which exposes the async and sync api for a function

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-500.png"
)]

mod args;
mod struct_derive;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn maybe_fut(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match syn::parse(attr) {
        Ok(args) => args,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    struct_derive::maybe_fut_struct(args, item)
}

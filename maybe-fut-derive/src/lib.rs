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

    // check if the item is an impl block for a struct
    if let Ok(struct_item) = syn::parse::<syn::ItemImpl>(item) {
        return struct_derive::maybe_fut_struct(args, struct_item);
    }

    // error
    syn::Error::new(
        proc_macro2::Span::call_site(),
        "maybe_fut can only be used on impl blocks",
    )
    .into_compile_error()
    .into()
}

#![crate_name = "maybe_fut_io_derive"]
#![crate_type = "lib"]

//! # maybe-fut-io-derive
//!
//! Procedural macros to derive `Write`, `Read` and `Seek` traits for `maybe-fut`.
//!
//! ## Example
//!
//! ```rust,ignore
//! #[derive(Read, Write, Seek)]
//! #[io(feature("tokio-fs"))]
//! struct MyWrapper(FileInner);
//!
//! enum FileInner {
//!    Std(std::fs::File),
//!    Tokio(tokio::fs::File),
//! }
//! ```

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/maybe-fut/main/assets/images/logo-500.png"
)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parenthesized, parse_macro_input};

#[proc_macro_derive(Read, attributes(io))]
pub fn read(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    // struct must be a tuple struct
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unnamed(ref fields) => &fields.unnamed,
            Fields::Named(_) => panic!("Unwrap can only be derived for tuple structs"),
            Fields::Unit => panic!("Unwrap can only be derived for tuple structs"),
        },
        _ => panic!("Unwrap can only be derived for structs"),
    };

    // should be a single field
    let parent_struct_field = match fields.len() {
        1 => &fields[0],
        _ => panic!("Unwrap can only be derived for structs with a single field"),
    };

    // this field must be an Enum
    let field_type = match &parent_struct_field.ty {
        syn::Type::Path(path) => path,
        _ => panic!("Unwrap can only be derived for structs with a single field"),
    };

    let field_type_ident = &field_type.path.segments.last().unwrap().ident;

    let Attributes { feature } = attrs(&input);

    let output = quote! {
        const _: () = {
            use crate::io::Read;

            impl Read for #struct_name {
                async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                    use std::io::Read as _;

                    match &mut self.0 {
                        #field_type_ident::Std(inner) => inner.read(buf),
                        #[cfg(feature = #feature)]
                        #field_type_ident::Tokio(inner) => {
                            use tokio::io::AsyncReadExt as _;
                            inner.read(buf).await
                        }
                    }
                }
            }
        };
    };

    output.into()
}

#[proc_macro_derive(Write, attributes(io))]
pub fn write(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    // struct must be a tuple struct
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unnamed(ref fields) => &fields.unnamed,
            Fields::Named(_) => panic!("Write can only be derived for tuple structs"),
            Fields::Unit => panic!("Write can only be derived for tuple structs"),
        },
        _ => panic!("Write can only be derived for structs"),
    };

    // should be a single field
    let parent_struct_field = match fields.len() {
        1 => &fields[0],
        _ => panic!("Write can only be derived for structs with a single field"),
    };

    // this field must be an Enum
    let field_type = match &parent_struct_field.ty {
        syn::Type::Path(path) => path,
        _ => panic!("Write can only be derived for structs with a single field"),
    };

    let field_type_ident = &field_type.path.segments.last().unwrap().ident;

    let Attributes { feature } = attrs(&input);

    let output = quote! {
        const _: () = {
            use crate::io::Write;

            impl Write for #struct_name {
                async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                    use std::io::Write as _;

                    match &mut self.0 {
                        #field_type_ident::Std(inner) => inner.write(buf),
                        #[cfg(feature = #feature)]
                        #field_type_ident::Tokio(inner) => {
                            use tokio::io::AsyncWriteExt as _;
                            inner.write(buf).await
                        }
                    }
                }

                async fn flush(&mut self) -> std::io::Result<()> {
                    use std::io::Write as _;

                    match &mut self.0 {
                        #field_type_ident::Std(inner) => inner.flush(),
                        #[cfg(feature = #feature)]
                        #field_type_ident::Tokio(inner) => {
                            use tokio::io::AsyncWriteExt as _;
                            inner.flush().await
                        }
                    }
                }
            }
        };
    };

    output.into()
}

#[proc_macro_derive(Seek, attributes(io))]
pub fn seek(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    // struct must be a tuple struct
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unnamed(ref fields) => &fields.unnamed,
            Fields::Named(_) => panic!("Seek can only be derived for tuple structs"),
            Fields::Unit => panic!("Seek can only be derived for tuple structs"),
        },
        _ => panic!("Seek can only be derived for structs"),
    };

    // should be a single field
    let parent_struct_field = match fields.len() {
        1 => &fields[0],
        _ => panic!("Seek can only be derived for structs with a single field"),
    };

    // this field must be an Enum
    let field_type = match &parent_struct_field.ty {
        syn::Type::Path(path) => path,
        _ => panic!("Seek can only be derived for structs with a single field"),
    };

    let field_type_ident = &field_type.path.segments.last().unwrap().ident;

    let Attributes { feature } = attrs(&input);

    let output = quote! {
        const _: () = {
            use crate::io::Seek;

            impl Seek for #struct_name {
                async fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
                    use std::io::Seek as _;

                    match &mut self.0 {
                        #field_type_ident::Std(inner) => inner.seek(pos),
                        #[cfg(feature = #feature)]
                        #field_type_ident::Tokio(inner) => {
                            use tokio::io::AsyncSeekExt as _;
                            inner.seek(pos).await
                        }
                    }
                }
            }
        };
    };

    output.into()
}

struct Attributes {
    feature: syn::LitStr,
}

fn attrs(input: &DeriveInput) -> Attributes {
    let mut feature: Option<syn::LitStr> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("io") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("feature") {
                    let content;
                    parenthesized!(content in meta.input);
                    feature = Some(
                        content
                            .parse::<syn::LitStr>()
                            .expect("feature ident not a value"),
                    );
                    Ok(())
                } else if meta.path.is_ident("io") {
                    // This is the main attribute, we can ignore it
                    Ok(())
                } else {
                    Err(meta.error("Expected #[io]"))
                }
            })
            .expect("Invalid syntax in #[io]");
        }
    }

    Attributes {
        feature: feature.expect("Missing `feature` in #[io]"),
    }
}

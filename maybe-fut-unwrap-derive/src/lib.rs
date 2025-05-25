#![crate_name = "maybe_fut_unwrap_derive"]
#![crate_type = "lib"]

//! # maybe-fut-unwrap-derive
//!
//! A procedural macro which exposes the `unwrap` method for `MaybeFuture` types.
//!
//! ## Example
//!
//! ```rust,ignore
//! #[derive(Unwrap)]
//! #[unwrap_types(std(std::fs::File), tokio(tokio::fs::File))]
//! struct MyWrapper(InnerWrapper);
//!
//! enum InnerWrapper {
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

#[proc_macro_derive(Unwrap, attributes(unwrap_types))]
pub fn unwrap(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
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

    let mut std_mod: Option<syn::Type> = None;
    let mut tokio_mod: Option<syn::Type> = None;
    let mut tokio_gated: Option<syn::LitStr> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("unwrap_types") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("std") {
                    let content;
                    parenthesized!(content in meta.input);
                    std_mod = Some(content.parse::<syn::Type>().expect("std ident not a value"));
                    Ok(())
                } else if meta.path.is_ident("tokio") {
                    let content;
                    parenthesized!(content in meta.input);
                    tokio_mod = Some(
                        content
                            .parse::<syn::Type>()
                            .expect("tokio ident not a value"),
                    );
                    Ok(())
                } else if meta.path.is_ident("tokio_gated") {
                    let content;
                    parenthesized!(content in meta.input);
                    tokio_gated = Some(
                        content
                            .parse::<syn::LitStr>()
                            .expect("tokio_gated ident not a value"),
                    );
                    Ok(())
                } else if meta.path.is_ident("unwrap_types") {
                    // This is the main attribute, we can ignore it
                    Ok(())
                } else {
                    Err(meta.error("Expected #[unwrap_types]"))
                }
            })
            .expect("Invalid syntax in #[unwrap_types]");
        }
    }

    let std_inner_type = std_mod.expect("Missing `std` in #[unwrap_types]");
    let tokio_inner_type = tokio_mod.expect("Missing `tokio` in #[unwrap_types]");
    let tokio_gated = tokio_gated
        .as_ref()
        .expect("Missing `tokio_gated` in #[unwrap_types]");

    let output = quote! {
        const _: () = {
            use crate::Unwrap;

            impl #generics Unwrap for #struct_name #generics {
                type StdImpl = #std_inner_type #generics;
                #[cfg(feature = #tokio_gated)]
                type TokioImpl = #tokio_inner_type #generics;
                #[cfg(all(not(feature = #tokio_gated), feature = "tokio"))]
                type TokioImpl = #std_inner_type #generics;


                fn unwrap_std(self) -> Self::StdImpl {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => inner,
                        _ => panic!("Expected Std variant"),
                    }
                }

                #[cfg(feature = #tokio_gated)]
                fn unwrap_tokio(self) -> Self::TokioImpl {
                    match self {
                        #struct_name(#field_type_ident::Tokio(inner)) => inner,
                        _ => panic!("Expected Tokio variant"),
                    }
                }

                #[cfg(all(not(feature = #tokio_gated), feature = "tokio"))]
                fn unwrap_tokio(self) -> Self::TokioImpl {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => inner,
                        _ => panic!("Expected Std variant"),
                    }
                }

                fn unwrap_std_ref(&self) -> &Self::StdImpl {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => inner,
                        _ => panic!("Expected Std variant"),
                    }
                }

                #[cfg(feature = #tokio_gated)]
                fn unwrap_tokio_ref(&self) -> &Self::TokioImpl {
                    match self {
                        #struct_name(#field_type_ident::Tokio(inner)) => inner,
                        _ => panic!("Expected Tokio variant"),
                    }
                }

                #[cfg(all(not(feature = #tokio_gated), feature = "tokio"))]
                fn unwrap_tokio_ref(&self) -> &Self::TokioImpl {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => inner,
                        _ => panic!("Expected Std variant"),
                    }
                }

                fn unwrap_std_mut(&mut self) -> &mut Self::StdImpl {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => inner,
                        _ => panic!("Expected Std variant"),
                    }
                }

                #[cfg(feature = #tokio_gated)]
                fn unwrap_tokio_mut(&mut self) -> &mut Self::TokioImpl {
                    match self {
                        #struct_name(#field_type_ident::Tokio(inner)) => inner,
                        _ => panic!("Expected Tokio variant"),
                    }
                }

                #[cfg(all(not(feature = #tokio_gated), feature = "tokio"))]
                fn unwrap_tokio_mut(&mut self) -> &mut Self::TokioImpl {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => inner,
                        _ => panic!("Expected Std variant"),
                    }
                }

                fn get_std(self) -> Option<Self::StdImpl> {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => Some(inner),
                        _ => None,
                    }
                }

                #[cfg(feature = #tokio_gated)]
                fn get_tokio(self) -> Option<Self::TokioImpl> {
                    match self {
                        #struct_name(#field_type_ident::Tokio(inner)) => Some(inner),
                        _ => None,
                    }
                }

                #[cfg(all(not(feature = #tokio_gated), feature = "tokio"))]
                fn get_tokio(self) -> Option<Self::TokioImpl> {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => Some(inner),
                        _ => None,
                    }
                }

                fn get_std_ref(&self) -> Option<&Self::StdImpl > {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => Some(inner),
                        _ => None,
                    }
                }

                #[cfg(feature = #tokio_gated)]
                fn get_tokio_ref(&self) -> Option<&Self::TokioImpl> {
                    match self {
                        #struct_name(#field_type_ident::Tokio(inner)) => Some(inner),
                        _ => None,
                    }
                }

                #[cfg(all(not(feature = #tokio_gated), feature = "tokio"))]
                fn get_tokio_ref(&self) -> Option<&Self::TokioImpl> {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => Some(inner),
                        _ => None,
                    }
                }

                fn get_std_mut(&mut self) -> Option<&mut Self::StdImpl > {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => Some(inner),
                        _ => None,
                    }
                }

                #[cfg(feature = #tokio_gated)]
                fn get_tokio_mut(&mut self) -> Option<&mut Self::TokioImpl> {
                    match self {
                        #struct_name(#field_type_ident::Tokio(inner)) => Some(inner),
                        _ => None,
                    }
                }

                #[cfg(all(not(feature = #tokio_gated), feature = "tokio"))]
                fn get_tokio_mut(&mut self) -> Option<&mut Self::TokioImpl> {
                    match self {
                        #struct_name(#field_type_ident::Std(inner)) => Some(inner),
                        _ => None,
                    }
                }
            }
        };
    };

    output.into()
}

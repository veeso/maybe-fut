use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::{Ident, ImplItemFn, Type, parse_macro_input};

use super::args::MaybeFutArgs;

pub fn maybe_fut_struct(
    MaybeFutArgs {
        sync: sync_struct_name,
        tokio: tokio_struct_name,
        tokio_feature,
    }: MaybeFutArgs,
    item: TokenStream,
) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemImpl);

    // get struct name of impl
    let implementing_for = match implementing_for(&ast) {
        Ok(ident) => ident,
        Err(err) => return err,
    };

    // get all the methods in the impl block
    let mut methods = Vec::new();
    for impl_item in &ast.items {
        if let syn::ImplItem::Fn(method) = impl_item {
            methods.push(method.clone());
        }
    }

    // make sync structure block
    let sync_quoted_methods = sync_methods(&implementing_for, &ast.self_ty, &methods);

    // make async structure block
    let async_quoted_methods = async_methods(&implementing_for, &ast.self_ty, &methods);

    let output = quote! {
        pub struct #sync_struct_name(#implementing_for);

        impl #sync_struct_name {
            #(#sync_quoted_methods)*
        }

        #[cfg(feature = #tokio_feature)]
        pub struct #tokio_struct_name(#implementing_for);

        #[cfg(feature = #tokio_feature)]
        impl #tokio_struct_name {
            #(#async_quoted_methods)*
        }

        #ast
    };

    output.into()
}

/// Extracts the implementing type from the `ItemImpl` AST node.
fn implementing_for(ast: &syn::ItemImpl) -> Result<syn::Ident, TokenStream> {
    match ast.self_ty.as_ref() {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                Ok(segment.ident.clone())
            } else {
                Err(syn::Error::new_spanned(
                    ast.self_ty.clone(),
                    "Expected a type path with at least one segment",
                )
                .to_compile_error()
                .into())
            }
        }
        _ => Err(syn::Error::new_spanned(
            ast.self_ty.clone(),
            "Expected a type path for the implementing type",
        )
        .to_compile_error()
        .into()),
    }
}

/// Generates sync methods for the given methods in the impl block.
fn sync_methods(
    implementing_for: &Ident,
    self_ty: &Type,
    methods: &[ImplItemFn],
) -> Vec<TokenStream2> {
    methods
        .iter()
        .map(|method| {
            let visibility = &method.vis;
            let method_name = &method.sig.ident;
            let args = &method.sig.inputs;
            let ret_type = &method.sig.output;
            let asyncness = method.sig.asyncness.is_some();
            let attrs = &method.attrs;
            let mut first_is_self = false;
            let constness = method.sig.constness;

            let call_args = call_args(args, &mut first_is_self);
            let is_constructor = is_constructor(self_ty, method);

            let fn_body = if is_constructor {
                quote! {
                    Self(#implementing_for::#method_name(#call_args))
                }
            } else if !first_is_self {
                quote! {
                     #implementing_for::#method_name(#call_args)
                }
            } else {
                quote! {
                    self.0.#method_name(#call_args)
                }
            };

            if asyncness {
                quote! {
                    #(#attrs)*
                    #visibility #constness fn #method_name(#args) #ret_type {
                        ::maybe_fut::SyncRuntime::block_on(
                            #fn_body
                        )
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    #visibility #constness fn #method_name(#args) #ret_type {
                        #fn_body
                    }
                }
            }
        })
        .collect()
}

/// Generates async methods for the given methods in the impl block.
fn async_methods(
    implementing_for: &Ident,
    self_ty: &Type,
    methods: &[ImplItemFn],
) -> Vec<TokenStream2> {
    methods
        .iter()
        .map(|method| {
            let visibility = &method.vis;
            let method_name = &method.sig.ident;
            let args = &method.sig.inputs;
            let ret_type = &method.sig.output;
            let asyncness = method.sig.asyncness;
            let constness = method.sig.constness;
            let is_async = asyncness.is_some();
            let attrs = &method.attrs;
            let mut first_is_self = false;

            let call_args = call_args(args, &mut first_is_self);
            let is_constructor = is_constructor(self_ty, method);

            let await_block = if is_async {
                quote! {
                    .await
                }
            } else {
                quote! {}
            };

            let fn_body = if is_constructor {
                quote! {
                    Self(#implementing_for::#method_name(#call_args)#await_block)
                }
            } else if !first_is_self {
                quote! {
                     #implementing_for::#method_name(#call_args)#await_block
                }
            } else {
                quote! {
                    self.0.#method_name(#call_args)#await_block
                }
            };

            quote! {
                #(#attrs)*
                #visibility #constness #asyncness fn #method_name(#args) #ret_type {
                    #fn_body
                }
            }
        })
        .collect()
}

/// Returns whether the method is a constructor for the
fn is_constructor(self_ty: &Type, method: &ImplItemFn) -> bool {
    // check if this is a constructor of the inner type
    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        let mut a_tokens = proc_macro2::TokenStream::new();
        let mut b_tokens = proc_macro2::TokenStream::new();
        ty.to_tokens(&mut a_tokens);
        self_ty.to_tokens(&mut b_tokens);
        if a_tokens.to_string() == b_tokens.to_string() {
            return true;
        }
    }

    // also check if output is `Self`
    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        if let syn::Type::Path(type_path) = ty.as_ref() {
            if type_path.path.is_ident("Self") {
                return true;
            }
        }
    }

    false
}

/// Returns the call arguments for the method with self removed.
///
/// Also returns whether the first argument is self.
fn call_args(
    args: &Punctuated<syn::FnArg, syn::token::Comma>,
    first_is_self: &mut bool,
) -> Punctuated<Box<syn::Pat>, syn::token::Comma> {
    // arguments to pass must have first argument removed if it is self and we must remove the types
    let mut call_args: Punctuated<Box<syn::Pat>, syn::token::Comma> = Punctuated::new();
    for arg in args.iter() {
        // check if first is self
        if !*first_is_self {
            if let syn::FnArg::Receiver(_) = arg {
                *first_is_self = true;
                continue;
            }
        }

        if let syn::FnArg::Typed(arg) = arg {
            call_args.push(arg.pat.clone());
        }
    }

    call_args
}

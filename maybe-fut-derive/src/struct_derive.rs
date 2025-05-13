use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;

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
    let sync_quoted_methods = methods.iter().map(|method| {
        let visibility = &method.vis;
        let method_name = &method.sig.ident;
        let args = &method.sig.inputs;
        let ret_type = &method.sig.output;
        let attrs = &method.attrs;
        let is_async = method.sig.asyncness.is_some();

        let mut first_is_self = false;
        let mut is_constructor = false;

        // arguments to pass must have first argument removed if it is self and we must remove the types
        let mut call_args: Punctuated<Box<syn::Pat>, syn::token::Comma> = Punctuated::new();
        for arg in args.iter() {
            // check if first is self
            if !first_is_self {
                if let syn::FnArg::Receiver(_) = arg {
                    first_is_self = true;
                    continue;
                }
            }

            if let syn::FnArg::Typed(arg) = arg {
                call_args.push(arg.pat.clone());
            }
        }

        if !first_is_self {
            // check if this is a constructor of the inner type
            if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                let mut a_tokens = proc_macro2::TokenStream::new();
                let mut b_tokens = proc_macro2::TokenStream::new();
                ty.to_tokens(&mut a_tokens);
                ast.self_ty.to_tokens(&mut b_tokens);
                is_constructor = a_tokens.to_string() == b_tokens.to_string();
            }

            // also check if output is `Self`
            if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                if let syn::Type::Path(type_path) = ty.as_ref() {
                    if type_path.path.is_ident("Self") {
                        is_constructor = true;
                    }
                }
            }
        }

        if !first_is_self && is_constructor {
            if is_async {
                quote! {
                    #(#attrs)*
                    #visibility fn #method_name(#args) #ret_type {
                        Self(maybe_fut::SyncRuntime::block_on(
                            #implementing_for::#method_name(#call_args)
                        ))
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    #visibility fn #method_name(#args) #ret_type {
                        Self(
                            #implementing_for::#method_name(#call_args)
                        )
                    }
                }
            }
        } else if !first_is_self {
            if is_async {
                quote! {
                    #(#attrs)*
                    #visibility fn #method_name(#args) #ret_type {
                        maybe_fut::SyncRuntime::block_on(
                            #implementing_for::#method_name(#call_args)
                        )
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    #visibility fn #method_name(#args) #ret_type  {
                        #implementing_for::#method_name(#call_args)
                    }
                }
            }
        } else if is_async {
            quote! {
                #(#attrs)*
                #visibility fn #method_name(#args) #ret_type  {
                    maybe_fut::SyncRuntime::block_on(
                        self.0.#method_name(#call_args)
                    )
                }
            }
        } else {
            quote! {
                #(#attrs)*
                #visibility fn #method_name(#args) #ret_type  {
                    self.0.#method_name(#call_args)
                }
            }
        }
    });

    // make async structure block
    let async_quoted_methods = methods.iter().map(|method| {
        let visibility = &method.vis;
        let method_name = &method.sig.ident;
        let args = &method.sig.inputs;
        let ret_type = &method.sig.output;
        let asyncness = method.sig.asyncness;
        let is_async = asyncness.is_some();
        let attrs = &method.attrs;

        let mut first_is_self = false;
        let mut is_constructor = false;

        // arguments to pass must have first argument removed if it is self and we must remove the types
        let mut call_args: Punctuated<Box<syn::Pat>, syn::token::Comma> = Punctuated::new();
        for arg in args.iter() {
            // check if first is self
            if !first_is_self {
                if let syn::FnArg::Receiver(_) = arg {
                    first_is_self = true;
                    continue;
                }
            }

            if let syn::FnArg::Typed(arg) = arg {
                call_args.push(arg.pat.clone());
            }
        }

        if !first_is_self {
            // check if this is a constructor of the inner type
            if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                let mut a_tokens = proc_macro2::TokenStream::new();
                let mut b_tokens = proc_macro2::TokenStream::new();
                ty.to_tokens(&mut a_tokens);
                ast.self_ty.to_tokens(&mut b_tokens);
                is_constructor = a_tokens.to_string() == b_tokens.to_string();
            }

            // also check if output is `Self`
            if let syn::ReturnType::Type(_, ty) = &method.sig.output {
                if let syn::Type::Path(type_path) = ty.as_ref() {
                    if type_path.path.is_ident("Self") {
                        is_constructor = true;
                    }
                }
            }
        }

        if !first_is_self && is_constructor {
            if is_async {
                quote! {
                    #(#attrs)*
                    #visibility #asyncness fn #method_name(#args) #ret_type {
                        Self(#implementing_for::#method_name(#call_args).await)
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    #visibility #asyncness fn #method_name(#args) #ret_type {
                        Self(#implementing_for::#method_name(#call_args))
                    }
                }
            }
        } else if !first_is_self {
            if is_async {
                quote! {
                    #(#attrs)*
                    #visibility #asyncness fn #method_name(#args) #ret_type {
                        #implementing_for::#method_name(#call_args).await
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    #visibility #asyncness fn #method_name(#args) #ret_type {
                        #implementing_for::#method_name(#call_args)
                    }
                }
            }
        } else if is_async {
            quote! {
                #(#attrs)*
                #visibility #asyncness fn #method_name(#args) #ret_type {
                    self.0.#method_name(#call_args).await
                }
            }
        } else {
            quote! {
                #(#attrs)*
                #visibility #asyncness fn #method_name(#args) #ret_type {
                    self.0.#method_name(#call_args)
                }
            }
        }
    });

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

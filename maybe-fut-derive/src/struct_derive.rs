use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::{Generics, Ident, ImplItemFn, ItemImpl, Type};

use super::args::MaybeFutArgs;

pub fn maybe_fut_struct(
    MaybeFutArgs {
        sync: sync_struct_name,
        tokio: tokio_struct_name,
        tokio_feature,
    }: MaybeFutArgs,
    ast: ItemImpl,
) -> TokenStream {
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

    // get generics impl parameters
    let generics = &ast.generics;
    // get generics parameters
    let where_clause = &ast.generics.where_clause;
    // get trait impl
    let trait_impl = &ast.trait_;

    // make sync structure block
    let sync_quoted_methods =
        gen_methods(&implementing_for, &ast.self_ty, generics, &methods, false);

    // make async structure block
    let async_quoted_methods =
        gen_methods(&implementing_for, &ast.self_ty, generics, &methods, true);

    // check if we have a trait impl; in case it's a trait, we always return the `async_quoted_methods`, because if
    // a function is async, we cannot get rid of that in the sync impl
    if let Some((_, trait_name, for_token)) = trait_impl {
        return quote! {
            impl #generics #trait_name #for_token #sync_struct_name #generics #where_clause {
                #(#async_quoted_methods)*
            }

            #[cfg(feature = #tokio_feature)]
            impl #generics #trait_name #for_token #tokio_struct_name #generics #where_clause {
                #(#async_quoted_methods)*
            }

            #ast
        }
        .into();
    }

    // Normal impl block
    quote! {
        pub struct #sync_struct_name #generics (#implementing_for #generics) #where_clause;

        impl #generics #sync_struct_name #generics
        #where_clause
        {
            #(#sync_quoted_methods)*
        }

        #[cfg(feature = #tokio_feature)]
        pub struct #tokio_struct_name #generics (#implementing_for #generics) #where_clause;

        #[cfg(feature = #tokio_feature)]
        impl #generics #tokio_struct_name #generics
        #where_clause
        {
            #(#async_quoted_methods)*
        }

        #ast
    }
    .into()
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

/// Generates sync or async (based on value of `async_methods`) methods for the given methods in the impl block.
fn gen_methods(
    implementing_for: &Ident,
    self_ty: &Type,
    generics: &Generics,
    methods: &[ImplItemFn],
    async_methods: bool,
) -> Vec<TokenStream2> {
    methods
        .iter()
        .map(|method| {
            let visibility = &method.vis;
            let method_name = &method.sig.ident;
            let args = &method.sig.inputs;
            let ret_type = &method.sig.output;
            let asyncness = method.sig.asyncness;
            let is_async = asyncness.is_some();
            let attrs = &method.attrs;
            let mut first_is_self = false;
            let constness = method.sig.constness;

            let call_args = call_args(args, &mut first_is_self);
            let constructor_args = is_constructor(self_ty, method);

            let await_block = if is_async && async_methods {
                quote! {
                    .await
                }
            } else {
                quote! {}
            };

            let generics_block = if generics.params.is_empty() {
                quote! {}
            } else {
                quote! { ::#generics }
            };

            let fn_body = if let Some(constructor_args) = constructor_args {
                if constructor_args.is_result {
                    quote! {
                        Ok(Self(#implementing_for #generics_block::#method_name(#call_args)#await_block?))
                    }
                } else if constructor_args.is_option {
                    quote! {
                        Some(Self(#implementing_for #generics_block::#method_name(#call_args)#await_block?))
                    }
                } else {
                    quote! {
                        Self(#implementing_for #generics_block::#method_name(#call_args)#await_block)
                    }
                }
            } else if !first_is_self {
                quote! {
                     #implementing_for #generics_block::#method_name(#call_args)#await_block
                }
            } else {
                quote! {
                    self.0.#method_name(#call_args)#await_block
                }
            };

            if is_async && !async_methods {
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
                    #visibility #constness #asyncness fn #method_name(#args) #ret_type {
                        #fn_body
                    }
                }
            }
        })
        .collect()
}

struct ConstructorParams {
    pub is_result: bool,
    pub is_option: bool,
}

/// Returns whether the method is a constructor for the
fn is_constructor(self_ty: &Type, method: &ImplItemFn) -> Option<ConstructorParams> {
    // check if this is a constructor of the inner type
    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        let mut a_tokens = proc_macro2::TokenStream::new();
        let mut b_tokens = proc_macro2::TokenStream::new();
        ty.to_tokens(&mut a_tokens);
        self_ty.to_tokens(&mut b_tokens);
        if a_tokens.to_string() == b_tokens.to_string() {
            return Some(ConstructorParams {
                is_result: false,
                is_option: false,
            });
        }
    }

    // also check if output is `Self`
    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        if let syn::Type::Path(type_path) = ty.as_ref() {
            if type_path.path.is_ident("Self") {
                return Some(ConstructorParams {
                    is_result: false,
                    is_option: false,
                });
            }
        }
    }

    // check if the output is Result<Self, _>
    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        if let syn::Type::Path(type_path) = ty.as_ref() {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Result" {
                    if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
                        if let Some(syn::GenericArgument::Type(syn::Type::Path(inner_type_path))) =
                            args.args.first()
                        {
                            if inner_type_path.path.is_ident("Self") {
                                return Some(ConstructorParams {
                                    is_result: true,
                                    is_option: false,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // check if the output is Option<Self>
    if let syn::ReturnType::Type(_, ty) = &method.sig.output {
        if let syn::Type::Path(type_path) = ty.as_ref() {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Option" {
                    if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
                        if let Some(syn::GenericArgument::Type(syn::Type::Path(inner_type_path))) =
                            args.args.first()
                        {
                            if inner_type_path.path.is_ident("Self") {
                                return Some(ConstructorParams {
                                    is_result: false,
                                    is_option: true,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    None
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

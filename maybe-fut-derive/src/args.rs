use syn::{Ident, LitStr, Token};

pub struct MaybeFutArgs {
    pub sync: Ident,
    pub tokio: Ident,
    pub tokio_feature: LitStr,
}

impl syn::parse::Parse for MaybeFutArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut sync = None;
        let mut tokio = None;
        let mut tokio_feature = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "sync" => sync = Some(input.parse()?),
                "tokio" => tokio = Some(input.parse()?),
                "tokio_feature" => tokio_feature = Some(input.parse()?),
                other => {
                    return Err(syn::Error::new_spanned(
                        key,
                        format!("Unexpected key `{}`", other),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        let sync = match sync {
            Some(ident) => ident,
            None => {
                return Err(syn::Error::new_spanned(sync, "Missing sync attribute"));
            }
        };
        let tokio = match tokio {
            Some(ident) => ident,
            None => {
                return Err(syn::Error::new_spanned(tokio, "Missing tokio attribute"));
            }
        };
        let tokio_feature = match tokio_feature {
            Some(lit) => lit,
            None => {
                return Err(syn::Error::new_spanned(
                    tokio_feature,
                    "Missing tokio_feature attribute",
                ));
            }
        };

        Ok(MaybeFutArgs {
            sync,
            tokio,
            tokio_feature,
        })
    }
}

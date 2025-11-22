use syn::parse::{Parse, ParseStream};
use syn::{LitInt, Token};
use syn::token::Comma;

// very similar to `ObjectSizes`
pub(crate) struct Timeouts {
    pub(crate) first: Option<u16>,
    pub(crate) second: Option<u16>,
}

impl Parse for Timeouts {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![,]) {
            let _: Comma = input.parse().expect("comma to be present after checking for it");
            let second: LitInt  = input.parse()?;
            Ok(Timeouts {
                first: None,
                second: Some(second.base10_parse()?),
            })
        } else {
            let first: LitInt  = input.parse()?;
        
            if input.peek(Token![,]) {
                let _: Comma = input.parse().expect("comma to be present after checking for it");
                let second: LitInt  = input.parse()?;
        
                Ok(Timeouts {
                    first: Some(first.base10_parse()?),
                    second: Some(second.base10_parse()?),
                })
            } else {
                Ok(Timeouts {
                    first: Some(first.base10_parse()?),
                    second: None,
                })
            }
        }
    }
}
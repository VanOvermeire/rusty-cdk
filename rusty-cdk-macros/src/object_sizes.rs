use syn::parse::{Parse, ParseStream};
use syn::{LitInt, Token};
use syn::token::Comma;

pub(crate) struct ObjectSizes {
    pub(crate) first: Option<u32>,
    pub(crate) second: Option<u32>,
}

impl Parse for ObjectSizes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![,]) {
            let _: Comma = input.parse().expect("comma to be present after checking for it");
            let second: LitInt  = input.parse()?;
            Ok(ObjectSizes {
                first: None,
                second: Some(second.base10_parse()?),
            })
        } else {
            let first: LitInt  = input.parse()?;
        
            if input.peek(Token![,]) {
                let _: Comma = input.parse().expect("comma to be present after checking for it");
                let second: LitInt  = input.parse()?;
        
                Ok(ObjectSizes {
                    first: Some(first.base10_parse()?),
                    second: Some(second.base10_parse()?),
                })
            } else {
                Ok(ObjectSizes {
                    first: Some(first.base10_parse()?),
                    second: None,
                })
            }
        }
    }
}
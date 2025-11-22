use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{Ident, LitInt};

pub(crate) struct TransitionInfo {
    pub(crate) days: u16,
    pub(crate) service: String,
}

impl Parse for TransitionInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let days: LitInt = input.parse()?;
        let _: Comma = input.parse()?;
        let service: Ident = input.parse()?;

        Ok(TransitionInfo {
            days: days.base10_parse()?,
            service: service.to_string(),
        })
    }
}

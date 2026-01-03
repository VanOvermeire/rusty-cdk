use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{LitInt, LitStr};

pub(crate) struct BucketTiering {
    pub(crate) access_tier: String,
    pub(crate) days: u16,
}

impl Parse for BucketTiering {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tiering: LitStr = input.parse()?;
        let _: Comma = input.parse()?;
        let days: LitInt = input.parse()?;

        Ok(BucketTiering {
            access_tier: tiering.value(),
            days: days.base10_parse()?,
        })
    }
}

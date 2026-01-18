use syn::{LitStr};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;

pub(crate) struct GenericInput {
    pub(crate) resource_id: String,
    pub(crate) identifier: String,
}

impl Parse for GenericInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let resource_id: LitStr = input.parse()?;
        let _: Comma = input.parse()?;
        let identifier: LitStr = input.parse()?;

        Ok(GenericInput {
            resource_id: resource_id.value(),
            identifier: identifier.value(),
        })
    }
}
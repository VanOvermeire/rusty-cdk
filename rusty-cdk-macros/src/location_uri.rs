use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{LitStr, Token};

pub(crate) struct LocationUri {
    pub(crate) location_uri_type: String,
    pub(crate) content: Option<String>,
}

impl Parse for LocationUri {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let days: LitStr = input.parse()?;

        let content = if input.peek(Token![,]) {
            let _: Comma = input.parse()?;
            let service: LitStr = input.parse()?;
            Some(service.value())
        } else {
            None
        };

        Ok(LocationUri {
            location_uri_type: days.value(),
            content,
        })
    }
}

use syn::{LitInt, LitStr};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;

pub(crate) struct RateExpression {
    pub(crate) value: u16,
    pub(crate) unit: String,
}

// ideally, this would also accept `rate(value unit)` instead of only `value unit`
impl Parse for RateExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let days: LitInt = input.parse()?;
        let _: Comma = input.parse()?;
        let service: LitStr = input.parse()?;

        Ok(RateExpression {
            value: days.base10_parse()?,
            unit: service.value(),
        })
    }
}
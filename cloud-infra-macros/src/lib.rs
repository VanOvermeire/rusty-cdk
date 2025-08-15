use proc_macro::{TokenStream};
use quote::quote;
use syn::{LitInt, LitStr};

/// The outputs (like `StringWithOnlyAlphaNumericsAndUnderscores(pub String);`) produced by this crate are _not defined here_!
/// Instead, they appear in other crates from this library

// TODO tests
#[proc_macro]
pub fn create_alphanumeric_underscore_string(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();

    if output.value().is_empty() {
        panic!("value should not be blank")
    }
    
    if output.value().chars().any(|c| !c.is_alphanumeric() && c != '_') {
        panic!("value should only contain alphanumeric characters and underscores")
    }
    
    quote!(
        StringWithOnlyAlphaNumericsAndUnderscores(#output.to_string())
    ).into()
}

#[proc_macro]
pub fn create_non_zero_number(input: TokenStream) -> TokenStream {
    let output: LitInt = syn::parse(input).unwrap();
    
    let as_number: syn::Result<u32> = output.base10_parse();
    
    let num = if let Ok(num) = as_number {
        if num == 0 {
            panic!("Value should not be null")
        }
        num
    } else {
        panic!("Value is not a valid number")
    };
    
    quote!(
        NonZeroNumber(#num)
    ).into()
}

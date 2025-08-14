use proc_macro::{TokenStream};
use quote::quote;
use syn::LitStr;

/// The outputs (like `StringWithOnlyAlphaNumericsAndUnderscores(pub String);`) produced by this crate are not defined here
/// instead they appear in other crates

// TODO tests, name
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

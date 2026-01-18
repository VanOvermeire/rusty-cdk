use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::parsing::GenericInput;
use crate::roles::lookup_role_ref;

mod roles;
mod cloudcontrol;
mod parsing;

/// Tries to retrieve role information from your AWS environment (the role ARN).
/// This ensures that the role actually exists in your account and that your deployment will not fail.
/// 
/// You should pass on a unique id for the role, as well as the role name, separated by a comma: `get_role_ref!("SomeId","SomeRoleName")`
/// 
#[proc_macro]
pub fn get_role_ref(input: TokenStream) -> TokenStream {
    let input: GenericInput = parse_macro_input!(input);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(lookup_role_ref(&input.resource_id, &input.identifier)).unwrap_or_else(|e| e.into_compile_error().into())
}

use crate::parsing::GenericInput;
use crate::roles::{find_kms_ref, find_role_ref, find_secret_ref};
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod cloudcontrol;
mod parsing;
mod roles;

/// Tries to retrieve IAM role information from your AWS environment (the role ARN).
/// This ensures that the role actually exists in your account and that your deployment will not fail.
///
/// You should pass on a unique id for the role, as well as the role name, separated by a comma: `lookup_role_ref!("SomeId","SomeRoleName")`
#[proc_macro]
pub fn lookup_role_ref(input: TokenStream) -> TokenStream {
    let input: GenericInput = parse_macro_input!(input);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(find_role_ref(&input.resource_id, &input.identifier))
        .unwrap_or_else(|e| e.into_compile_error().into())
}

/// Tries to retrieve KMS key information from your AWS environment (the ARN).
/// This ensures that the key actually exists in your account and that your deployment will not fail.
///
/// You should pass on a unique id, as well as the kms key id, separated by a comma: `lookup_kms_key_ref!("SomeId","SomeKeyId")`
#[proc_macro]
pub fn lookup_kms_key_ref(input: TokenStream) -> TokenStream {
    let input: GenericInput = parse_macro_input!(input);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(find_kms_ref(&input.resource_id, &input.identifier))
        .unwrap_or_else(|e| e.into_compile_error().into())
}

/// Tries to retrieve Secret information from your AWS environment.
/// This ensures that the secret actually exists in your account and that your deployment will not fail.
///
/// You should pass on a unique id, as well as the secret arn, separated by a comma: `lookup_secret_ref!("SomeId","arn:aws:secretsmanager:...")`
#[proc_macro]
pub fn lookup_secret_ref(input: TokenStream) -> TokenStream {
    let input: GenericInput = parse_macro_input!(input);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(find_secret_ref(&input.resource_id, &input.identifier))
        .unwrap_or_else(|e| e.into_compile_error().into())
}
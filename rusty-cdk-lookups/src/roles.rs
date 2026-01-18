use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::Error;
use crate::cloudcontrol::{lookup, ResourceInfo};

pub(crate) async fn lookup_role_ref(resource_id: &str, role_name: &str) -> Result<TokenStream, Error> {
    let ResourceInfo { identifier, arn } = lookup(role_name, "AWS::IAM::Role").await.map_err(|e| Error::new(Span::call_site(), e))?;
    
    Ok(quote!(
        RoleRef::new(#resource_id, #identifier, #arn)
    ).into())
}

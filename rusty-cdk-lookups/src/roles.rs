use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::Error;
use crate::cloudcontrol::{lookup, ResourceInfo};

pub(crate) async fn find_role_ref(resource_id: &str, role_name: &str) -> Result<TokenStream, Error> {
    // TODO would be nice to additionally check that the permissions are correct
    //  this would require context or the user passing in additional info though
    let ResourceInfo { identifier, arn } = lookup(role_name, "AWS::IAM::Role").await.map_err(|e| Error::new(Span::call_site(), e))?;
    
    Ok(quote!(
        RoleRef::new(#resource_id, #identifier, #arn)
    ).into())
}

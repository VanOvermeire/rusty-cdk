use crate::cloudcontrol::{ResourceInfo, ResourceInfoWithArn, lookup, lookup_arn};
use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::Error;

pub(crate) async fn find_role_ref(resource_id: &str, role_name: &str) -> Result<TokenStream, Error> {
    // TODO would be nice to additionally check that the permissions are correct
    //  this would require context or the user passing in additional info though
    let ResourceInfoWithArn { identifier, arn } = lookup_arn(role_name, "AWS::IAM::Role")
        .await
        .map_err(|e| Error::new(Span::call_site(), e))?;

    Ok(quote!(
        RoleRef::new(#resource_id, #identifier, #arn)
    )
    .into())
}

pub(crate) async fn find_kms_ref(resource_id: &str, key_id: &str) -> Result<TokenStream, Error> {
    let ResourceInfoWithArn { identifier, arn } = lookup_arn(key_id, "AWS::KMS::Key")
        .await
        .map_err(|e| Error::new(Span::call_site(), e))?;

    Ok(quote!(
        KeyRef::new(#resource_id, #identifier, #arn)
    )
    .into())
}

pub(crate) async fn find_secret_ref(resource_id: &str, secret_arn: &str) -> Result<TokenStream, Error> {
    let ResourceInfo { identifier } = lookup(secret_arn, "AWS::SecretsManager::Secret")
        .await
        .map_err(|e| Error::new(Span::call_site(), e))?;

    Ok(quote!(
        SecretRef::new(#resource_id, #identifier, #identifier)
    )
    .into())
}

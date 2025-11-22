use serde::Serialize;
use serde_json::Value;
use crate::{dto_methods, ref_struct};
use crate::shared::Id;

ref_struct!(SecretRef);

#[derive(Debug, Serialize)]
pub struct Secret {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: SecretProperties,
}
dto_methods!(Secret);

#[derive(Debug, Serialize)]
pub struct SecretProperties {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub(super) name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub(super) description: Option<String>,
    #[serde(rename = "GenerateSecretString", skip_serializing_if = "Option::is_none")]
    pub(super) generate_secret_string: Option<GenerateSecretString>,
    #[serde(rename = "SecretString", skip_serializing_if = "Option::is_none")]
    pub(super) secret_string: Option<String>,
    // #[serde(rename = "KmsKeyId", skip_serializing_if = "Option::is_none")]
    // pub(super) kms_key_id: Option<String>,
    // #[serde(rename = "ReplicaRegions", skip_serializing_if = "Option::is_none")]
    // pub(super) replica_regions: Option<Vec<ReplicaRegion>>,
}

#[derive(Debug, Serialize)]
pub struct GenerateSecretString {
    #[serde(rename = "ExcludeCharacters", skip_serializing_if = "Option::is_none")]
    pub(super) exclude_characters: Option<String>,
    #[serde(rename = "ExcludeLowercase", skip_serializing_if = "Option::is_none")]
    pub(super) exclude_lowercase: Option<bool>,
    #[serde(rename = "ExcludeNumbers", skip_serializing_if = "Option::is_none")]
    pub(super) exclude_numbers: Option<bool>,
    #[serde(rename = "ExcludePunctuation", skip_serializing_if = "Option::is_none")]
    pub(super) exclude_punctuation: Option<bool>,
    #[serde(rename = "ExcludeUppercase", skip_serializing_if = "Option::is_none")]
    pub(super) exclude_uppercase: Option<bool>,
    #[serde(rename = "GenerateStringKey", skip_serializing_if = "Option::is_none")]
    pub(super) generate_string_key: Option<String>,
    #[serde(rename = "IncludeSpace", skip_serializing_if = "Option::is_none")]
    pub(super) include_space: Option<bool>,
    #[serde(rename = "PasswordLength", skip_serializing_if = "Option::is_none")]
    pub(super) password_length: Option<u32>,
    #[serde(rename = "RequireEachIncludedType", skip_serializing_if = "Option::is_none")]
    pub(super) require_each_included_type: Option<bool>,
    #[serde(rename = "SecretStringTemplate", skip_serializing_if = "Option::is_none")]
    pub(super) secret_string_template: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReplicaRegion {
    #[serde(rename = "Region")]
    pub(super) region: String,
    #[serde(rename = "KmsKeyId", skip_serializing_if = "Option::is_none")]
    pub(super) kms_key_id: Option<String>,
}
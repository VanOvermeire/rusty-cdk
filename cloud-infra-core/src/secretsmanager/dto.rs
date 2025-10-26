use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;
use crate::shared::Id;

// TODO builder

#[derive(Debug, Serialize)]
pub struct SecretsManagerSecret {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: SecretsManagerSecretProperties,
}

impl SecretsManagerSecret {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }
}

#[derive(Debug, Serialize)]
pub struct SecretsManagerSecretProperties {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(rename = "GenerateSecretString", skip_serializing_if = "Option::is_none")]
    pub(crate) generate_secret_string: Option<GenerateSecretString>,
    #[serde(rename = "SecretString", skip_serializing_if = "Option::is_none")]
    pub(crate) secret_string: Option<String>,
    // #[serde(rename = "KmsKeyId", skip_serializing_if = "Option::is_none")]
    // pub(crate) kms_key_id: Option<String>,
    // #[serde(rename = "ReplicaRegions", skip_serializing_if = "Option::is_none")]
    // pub(crate) replica_regions: Option<Vec<ReplicaRegion>>,
}

#[derive(Debug, Serialize)]
pub struct GenerateSecretString {
    #[serde(rename = "ExcludeCharacters", skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_characters: Option<String>,
    #[serde(rename = "ExcludeLowercase", skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_lowercase: Option<bool>,
    #[serde(rename = "ExcludeNumbers", skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_numbers: Option<bool>,
    #[serde(rename = "ExcludePunctuation", skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_punctuation: Option<bool>,
    #[serde(rename = "ExcludeUppercase", skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_uppercase: Option<bool>,
    #[serde(rename = "GenerateStringKey", skip_serializing_if = "Option::is_none")]
    pub(crate) generate_string_key: Option<String>,
    #[serde(rename = "IncludeSpace", skip_serializing_if = "Option::is_none")]
    pub(crate) include_space: Option<bool>,
    #[serde(rename = "PasswordLength", skip_serializing_if = "Option::is_none")]
    pub(crate) password_length: Option<u32>,
    #[serde(rename = "RequireEachIncludedType", skip_serializing_if = "Option::is_none")]
    pub(crate) require_each_included_type: Option<bool>,
    #[serde(rename = "SecretStringTemplate", skip_serializing_if = "Option::is_none")]
    pub(crate) secret_string_template: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReplicaRegion {
    #[serde(rename = "Region")]
    pub(crate) region: String,
    #[serde(rename = "KmsKeyId", skip_serializing_if = "Option::is_none")]
    pub(crate) kms_key_id: Option<String>,
}
use serde_json::Value;
use serde::Serialize;
use crate::{dto_methods, ref_struct};
use crate::shared::Id;

ref_struct!(ApplicationRef);

#[derive(Debug, Serialize)]
pub struct Application {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: ApplicationProperties
}
dto_methods!(Application);

#[derive(Debug, Serialize)]
pub struct ApplicationProperties {
    #[serde(rename = "Name")]
    pub(super) name: String,
}

ref_struct!(ConfigurationProfileRef);

#[derive(Debug, Serialize)]
pub struct ConfigurationProfile {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: ConfigurationProfileProperties
}
dto_methods!(ConfigurationProfile);

#[derive(Debug, Serialize)]
pub struct ConfigurationProfileProperties {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "ApplicationId")]
    pub(super) application_id: Value,
    #[serde(rename = "DeletionProtectionCheck", skip_serializing_if = "Option::is_none")]
    pub(super) deletion_protection_check: Option<String>,
    #[serde(rename = "LocationUri")]
    pub(super) location_uri: String,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub(super) config_type: Option<String>,
    #[serde(rename = "Validators", skip_serializing_if = "Option::is_none")]
    pub(super) validators: Option<Vec<Validator>>
    // "KmsKeyIdentifier" : String,
    // "RetrievalRoleArn" : String,
}

#[derive(Debug, Serialize)]
pub struct Validator {
    #[serde(rename = "Content")]
    pub(super) content: String,
    #[serde(rename = "Type")]
    pub(super) validator_type: String,
}

ref_struct!(DeploymentStrategyRef);

#[derive(Debug, Serialize)]
pub struct DeploymentStrategy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: DeploymentStrategyProperties
}
dto_methods!(DeploymentStrategy);

#[derive(Debug, Serialize)]
pub struct DeploymentStrategyProperties {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "DeploymentDurationInMinutes")]
    pub(super) deployment_duration_in_minutes: u16,
    #[serde(rename = "GrowthFactor")]
    pub(super) growth_factor: u16,
    #[serde(rename = "ReplicateTo")]
    pub(super) replicate_to: String,
    #[serde(rename = "GrowthType", skip_serializing_if = "Option::is_none")]
    pub(super) growth_type: Option<String>,
    // #[serde(rename = "FinalBakeTimeInMinutes", skip_serializing_if = "Option::is_none")]
    // pub(super) final_bake_time_in_minutes: u16, // 0 - 1440; requires additional permissions
}

ref_struct!(EnvironmentRef);

#[derive(Debug, Serialize)]
pub struct Environment {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: EnvironmentProperties,
}
dto_methods!(Environment);

#[derive(Debug, Serialize)]
pub struct EnvironmentProperties {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "ApplicationId")]
    pub(super) application_id: Value,
    #[serde(rename = "DeletionProtectionCheck", skip_serializing_if = "Option::is_none")]
    pub(super) deletion_protection_check: Option<String>
    // "Monitors" : [ Monitor, ... ],
}

use crate::shared::Id;
use crate::{dto_methods, ref_struct};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ApplicationType {
    #[serde(rename = "AWS::AppConfig::Application")]
    ApplicationType,
}

ref_struct!(ApplicationRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: ApplicationType,
    #[serde(rename = "Properties")]
    pub(super) properties: ApplicationProperties,
}
dto_methods!(Application);

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename = "Name")]
    pub(super) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ConfigurationProfileType {
    #[serde(rename = "AWS::AppConfig::ConfigurationProfile")]
    ConfigurationProfileType,
}

ref_struct!(ConfigurationProfileRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationProfile {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: ConfigurationProfileType,
    #[serde(rename = "Properties")]
    pub(super) properties: ConfigurationProfileProperties,
}
dto_methods!(ConfigurationProfile);

#[derive(Debug, Serialize, Deserialize)]
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
    pub(super) validators: Option<Vec<Validator>>, // "KmsKeyIdentifier" : String,
                                                   // "RetrievalRoleArn" : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Validator {
    #[serde(rename = "Content")]
    pub(super) content: Value,
    #[serde(rename = "Type")]
    pub(super) validator_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DeploymentStrategyType {
    #[serde(rename = "AWS::AppConfig::DeploymentStrategy")]
    DeploymentStrategyType,
}

ref_struct!(DeploymentStrategyRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentStrategy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: DeploymentStrategyType,
    #[serde(rename = "Properties")]
    pub(super) properties: DeploymentStrategyProperties,
}
dto_methods!(DeploymentStrategy);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentStrategyProperties {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "DeploymentDurationInMinutes")]
    pub(super) deployment_duration_in_minutes: u16,
    #[serde(rename = "GrowthFactor")]
    pub(super) growth_factor: u8,
    #[serde(rename = "ReplicateTo")]
    pub(super) replicate_to: String,
    #[serde(rename = "GrowthType", skip_serializing_if = "Option::is_none")]
    pub(super) growth_type: Option<String>,
    // #[serde(rename = "FinalBakeTimeInMinutes", skip_serializing_if = "Option::is_none")]
    // pub(super) final_bake_time_in_minutes: u16, // 0 - 1440; requires additional permissions
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum EnvironmentType {
    #[serde(rename = "AWS::AppConfig::Environment")]
    EnvironmentType,
}

ref_struct!(EnvironmentRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: EnvironmentType,
    #[serde(rename = "Properties")]
    pub(super) properties: EnvironmentProperties,
}
dto_methods!(Environment);

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentProperties {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "ApplicationId")]
    pub(super) application_id: Value,
    #[serde(rename = "DeletionProtectionCheck", skip_serializing_if = "Option::is_none")]
    pub(super) deletion_protection_check: Option<String>, // "Monitors" : [ Monitor, ... ],
}

use serde::Serialize;
use crate::shared::Id;

#[derive(Debug, Serialize)]
pub enum GrowthType {
    Linear,
    Exponential,
}

impl From<GrowthType> for String {
    fn from(value: GrowthType) -> Self {
        match value {
            GrowthType::Linear => "LINEAR".to_string(),
            GrowthType::Exponential => "EXPONENTIAL".to_string()
        }
    }
}

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

#[derive(Debug, Serialize)]
pub struct DeploymentStrategyProperties {
    #[serde(rename = "Name")]
    pub(super) name: String, // 1 - 64 chars
    #[serde(rename = "DeploymentDurationInMinutes")]
    pub(super) deployment_duration_in_minutes: u16, // 0 - 1440
    #[serde(rename = "FinalBakeTimeInMinutes")]
    pub(super) final_bake_time_in_minutes: u16, // 0 - 1440
    #[serde(rename = "GrowthFactor")]
    pub(super) growth_factor: u16, // 0 - 100 ?
    #[serde(rename = "GrowthType", skip_serializing_if = "Option::is_none")]
    pub(super) growth_type: Option<GrowthType>,
    // #[serde(rename = "ReplicateTo")]
}

#[derive(Debug, Serialize)]
pub enum DeletionProtectionCheck {
    AccountDefault,
    Apply,
    Bypass,
}

impl From<DeletionProtectionCheck> for String {
    fn from(value: DeletionProtectionCheck) -> Self {
        match value {
            DeletionProtectionCheck::AccountDefault => "ACCOUNT_DEFAULT".to_string(),
            DeletionProtectionCheck::Apply => "APPLY".to_string(),
            DeletionProtectionCheck::Bypass => "BYPASS".to_string(),
        }
    }
}

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

#[derive(Debug, Serialize)]
pub struct EnvironmentProperties {
    #[serde(rename = "Name")]
    pub(super) name: String, // 1 - 64 chars
    #[serde(rename = "ApplicationId")]
    pub(super) application_id: String,
    #[serde(rename = "DeletionProtectionCheck", skip_serializing_if = "Option::is_none")]
    pub(super) deletion_protection_check: Option<DeletionProtectionCheck>
    // "Monitors" : [ Monitor, ... ],
}

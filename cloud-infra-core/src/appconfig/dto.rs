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
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: DeploymentStrategyProperties
}

#[derive(Debug, Serialize)]
pub struct DeploymentStrategyProperties {
    #[serde(rename = "Name")]
    pub(crate) name: String, // 1 - 64 chars
    #[serde(rename = "DeploymentDurationInMinutes")]
    pub(crate) deployment_duration_in_minutes: u16, // 0 - 1440
    #[serde(rename = "FinalBakeTimeInMinutes")]
    pub(crate) final_bake_time_in_minutes: u16, // 0 - 1440
    #[serde(rename = "GrowthFactor")]
    pub(crate) growth_factor: u16, // 0 - 100 ?
    #[serde(rename = "GrowthType", skip_serializing_if = "Option::is_none")]
    pub(crate) growth_type: Option<GrowthType>,
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
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: EnvironmentProperties,
}

#[derive(Debug, Serialize)]
pub struct EnvironmentProperties {
    #[serde(rename = "Name")]
    pub(crate) name: String, // 1 - 64 chars
    #[serde(rename = "ApplicationId")]
    pub(crate) application_id: String,
    #[serde(rename = "DeletionProtectionCheck", skip_serializing_if = "Option::is_none")]
    pub(crate) deletion_protection_check: Option<DeletionProtectionCheck>
    // "Monitors" : [ Monitor, ... ],
}

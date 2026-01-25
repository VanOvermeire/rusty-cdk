use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{dto_methods, ref_struct};
use crate::shared::Id;

ref_struct!(ScheduleRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: ScheduleProperties,
}

dto_methods!(Schedule);

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleProperties {
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub(super) start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub(super) end_date: Option<String>,
    #[serde(rename = "FlexibleTimeWindow")]
    pub(super) flexible_time_window: FlexibleTimeWindow,
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub(super) group_name: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub(super) name: Option<String>,
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub(super) state: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    pub(super) schedule_expression: String,
    #[serde(rename = "Target")]
    pub(super) target: Target
    // "ScheduleExpressionTimezone" : String,
    // "KmsKeyArn" : String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlexibleTimeWindow {
    #[serde(rename = "MaximumWindowInMinutes", skip_serializing_if = "Option::is_none")]
    pub(super) maximum_window_in_minutes: Option<u16>,
    #[serde(rename = "Mode")]
    pub(super) mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "Arn")]
    pub(super) arn: Value,
    #[serde(rename = "RoleArn")]
    pub(super) role_arn: Value,
    #[serde(rename = "Input", skip_serializing_if = "Option::is_none")]
    pub(super) input: Option<String>,
    #[serde(rename = "RetryPolicy", skip_serializing_if = "Option::is_none")]
    pub(super) retry_policy: Option<RetryPolicy>,
    // DeadLetterConfig: DeadLetterConfig
    // EcsParameters: EcsParameters
    // EventBridgeParameters: EventBridgeParameters
    // KinesisParameters: KinesisParameters
    // SageMakerPipelineParameters: SageMakerPipelineParameters
    // SqsParameters: SqsParameters
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetryPolicy {
    #[serde(rename = "MaximumEventAgeInSeconds", skip_serializing_if = "Option::is_none")]
    pub(super) maximum_event_age_in_seconds: Option<u32>,
    #[serde(rename = "MaximumRetryAttempts", skip_serializing_if = "Option::is_none")]
    pub(super) maximum_retry_attempts: Option<u8>
}
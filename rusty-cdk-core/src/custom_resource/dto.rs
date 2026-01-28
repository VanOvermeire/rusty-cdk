use crate::shared::Id;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{dto_methods, ref_struct};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum BucketNotificationType {
    #[serde(rename = "Custom::S3BucketNotifications")]
    BucketNotificationType
}

ref_struct!(BucketNotificationRef);

/// The code for bucket notifications is *heavily* inspired by the AWS CDK
/// And will be needed until this is supported natively in CloudFormation (https://github.com/aws-cloudformation/cloudformation-coverage-roadmap/issues/79)
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketNotification {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: BucketNotificationType,
    #[serde(rename = "Properties")]
    pub(super) properties: BucketNotificationProperties,
}

dto_methods!(BucketNotification);

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketNotificationProperties {
    #[serde(rename = "ServiceToken")]
    pub(super) service_token: Value,
    #[serde(rename = "BucketName")]
    pub(super) bucket_name: Value,
    #[serde(rename = "Managed")]
    pub(super) managed: bool,
    #[serde(rename = "SkipDestinationValidation")]
    pub(super) skip_destination_validation: bool,
    #[serde(rename = "NotificationConfiguration")]
    pub(super) notification_configuration: NotificationConfiguration,
    #[serde(rename = "DependsOn", skip_serializing_if = "Option::is_none")]
    pub(super) depends_on: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfiguration {
    #[serde(rename = "LambdaFunctionConfigurations", skip_serializing_if = "Option::is_none")]
    pub(super) lambda_configs: Option<Vec<LambdaFunctionConfiguration>>,
    #[serde(rename = "TopicConfigurations", skip_serializing_if = "Option::is_none")]
    pub(super) topic_configs: Option<Vec<TopicConfiguration>>,
    #[serde(rename = "QueueConfigurations", skip_serializing_if = "Option::is_none")]
    pub(super) queue_configs: Option<Vec<QueueConfiguration>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LambdaFunctionConfiguration {
    #[serde(rename = "Events")]
    pub(super) events: Vec<String>,
    #[serde(rename = "LambdaFunctionArn")]
    pub(super) arn: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicConfiguration {
    #[serde(rename = "Events")]
    pub(super) events: Vec<String>,
    #[serde(rename = "TopicArn")]
    pub(super) arn: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueConfiguration {
    #[serde(rename = "Events")]
    pub(super) events: Vec<String>,
    #[serde(rename = "QueueArn")]
    pub(super) arn: Value,
}

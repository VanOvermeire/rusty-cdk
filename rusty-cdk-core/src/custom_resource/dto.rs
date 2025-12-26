use crate::shared::Id;
use serde::Serialize;
use serde_json::Value;
use crate::{dto_methods, ref_struct};

ref_struct!(BucketNotificationRef);

/// The code for bucket notifications is *heavily* inspired by the AWS CDK
/// And will be needed until this is supported natively in CloudFormation (https://github.com/aws-cloudformation/cloudformation-coverage-roadmap/issues/79)
#[derive(Debug, Serialize)]
pub struct BucketNotification {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: BucketNotificationProperties,
}

dto_methods!(BucketNotification);

#[derive(Debug, Serialize)]
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
    #[serde(rename = "DependsOn")]
    pub(super) depends_on: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct NotificationConfiguration {
    #[serde(rename = "LambdaFunctionConfigurations")]
    pub(super) lambda_configs: Option<Vec<LambdaFunctionConfiguration>>
}

#[derive(Debug, Serialize)]
pub struct LambdaFunctionConfiguration {
    #[serde(rename = "Events")]
    pub(super) events: Vec<String>,
    #[serde(rename = "LambdaFunctionArn")]
    pub(super) arn: Value,
}
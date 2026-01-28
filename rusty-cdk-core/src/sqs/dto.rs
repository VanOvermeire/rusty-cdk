use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{dto_methods, ref_struct_with_id_methods};
use crate::iam::PolicyDocument;
use crate::shared::Id;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum QueueType {
    #[serde(rename = "AWS::SQS::Queue")]
    QueueType
}

ref_struct_with_id_methods!(QueueRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: QueueType,
    #[serde(rename = "Properties")]
    pub(super) properties: QueueProperties,
}

dto_methods!(Queue);

ref_struct_with_id_methods!(QueuePolicyRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueProperties {
    #[serde(rename = "ContentBasedDeduplication", skip_serializing_if = "Option::is_none")]
    pub(super) content_based_deduplication: Option<bool>,
    #[serde(rename = "DeduplicationScope", skip_serializing_if = "Option::is_none")]
    pub(super) deduplication_scope: Option<String>,
    #[serde(rename = "DelaySeconds", skip_serializing_if = "Option::is_none")]
    pub(super) delay_seconds: Option<u32>,
    #[serde(rename = "FifoThroughputLimit", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_throughput_limit: Option<String>,
    #[serde(rename = "FifoQueue", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_queue: Option<bool>,
    #[serde(rename = "KmsDataKeyReusePeriodSeconds", skip_serializing_if = "Option::is_none")]
    pub(super) kms_data_key_reuse_period_seconds: Option<u32>,
    #[serde(rename = "KmsMasterKeyId", skip_serializing_if = "Option::is_none")]
    pub(super) kms_master_key_id: Option<Value>,
    #[serde(rename = "MaximumMessageSize", skip_serializing_if = "Option::is_none")]
    pub(super) maximum_message_size: Option<u32>,
    #[serde(rename = "MessageRetentionPeriod", skip_serializing_if = "Option::is_none")]
    pub(super) message_retention_period: Option<u32>,
    #[serde(rename = "QueueName", skip_serializing_if = "Option::is_none")]
    pub(super) queue_name: Option<String>,
    #[serde(rename = "ReceiveMessageWaitTimeSeconds", skip_serializing_if = "Option::is_none")]
    pub(super) receive_message_wait_time_seconds: Option<u32>,
    #[serde(rename = "RedrivePolicy", skip_serializing_if = "Option::is_none")]
    pub(super) redrive_policy: Option<RedrivePolicy>,
    #[serde(rename = "RedriveAllowPolicy", skip_serializing_if = "Option::is_none")]
    pub(super) redrive_allow_policy: Option<Value>,
    #[serde(rename = "SqsManagedSseEnabled", skip_serializing_if = "Option::is_none")]
    pub(super) sqs_managed_sse_enabled: Option<bool>,
    #[serde(rename = "VisibilityTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) visibility_timeout: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum QueuePolicyType {
    #[serde(rename = "AWS::SQS::QueuePolicy")]
    QueuePolicyType
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueuePolicy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: QueuePolicyType,
    #[serde(rename = "Properties")]
    pub(crate) properties: QueuePolicyProperties,
}
dto_methods!(QueuePolicy);

#[derive(Debug, Serialize, Deserialize)]
pub struct QueuePolicyProperties {
    #[serde(rename = "PolicyDocument")]
    pub(crate) doc: PolicyDocument,
    #[serde(rename = "Queues")]
    pub(super) queues: Vec<Value>,   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedrivePolicy {
    #[serde(rename = "deadLetterTargetArn")]
    pub(super) dead_letter_target_arn: String,
    #[serde(rename = "maxReceiveCount")]
    pub(super) max_receive_count: u32,
}

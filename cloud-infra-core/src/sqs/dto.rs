use serde::Serialize;
use serde_json::Value;
use crate::ref_struct;
use crate::shared::Id;

ref_struct!(QueueRef);

#[derive(Debug, Serialize)]
pub struct Queue {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: QueueProperties,
}

impl Queue {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct QueueProperties {
    #[serde(rename = "QueueName", skip_serializing_if = "Option::is_none")]
    pub(super) queue_name: Option<String>,
    #[serde(rename = "DelaySeconds", skip_serializing_if = "Option::is_none")]
    pub(super) delay_seconds: Option<u32>,
    #[serde(rename = "MaximumMessageSize", skip_serializing_if = "Option::is_none")]
    pub(super) maximum_message_size: Option<u32>,
    #[serde(rename = "MessageRetentionPeriod", skip_serializing_if = "Option::is_none")]
    pub(super) message_retention_period: Option<u32>,
    #[serde(rename = "ReceiveMessageWaitTimeSeconds", skip_serializing_if = "Option::is_none")]
    pub(super) receive_message_wait_time_seconds: Option<u32>,
    #[serde(rename = "VisibilityTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) visibility_timeout: Option<u32>,
    #[serde(rename = "FifoQueue", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_queue: Option<bool>,
    #[serde(rename = "ContentBasedDeduplication", skip_serializing_if = "Option::is_none")]
    pub(super) content_based_deduplication: Option<bool>,
    #[serde(rename = "DeduplicationScope", skip_serializing_if = "Option::is_none")]
    pub(super) deduplication_scope: Option<String>,
    #[serde(rename = "FifoThroughputLimit", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_throughput_limit: Option<String>,
    #[serde(rename = "SqsManagedSseEnabled", skip_serializing_if = "Option::is_none")]
    pub(super) sqs_managed_sse_enabled: Option<bool>,
    #[serde(rename = "RedrivePolicy", skip_serializing_if = "Option::is_none")]
    pub(super) redrive_policy: Option<RedrivePolicy>,
    #[serde(rename = "RedriveAllowPolicy", skip_serializing_if = "Option::is_none")]
    pub(super) redrive_allow_policy: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct RedrivePolicy {
    #[serde(rename = "deadLetterTargetArn")]
    pub(super) dead_letter_target_arn: String,
    #[serde(rename = "maxReceiveCount")]
    pub(super) max_receive_count: u32,
}
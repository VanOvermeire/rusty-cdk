use serde::Serialize;
use serde_json::Value;
use crate::{dto_methods, ref_struct_with_id_methods};
use crate::iam::PolicyDocument;
use crate::shared::Id;

ref_struct_with_id_methods!(TopicRef);

#[derive(Debug, Serialize)]
pub struct Topic {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: TopicProperties,
}
dto_methods!(Topic);

#[derive(Debug, Serialize)]
pub struct TopicProperties {
    #[serde(rename = "TopicName", skip_serializing_if = "Option::is_none")]
    pub(super) topic_name: Option<String>,
    #[serde(rename = "FifoTopic", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_topic: Option<bool>,
    #[serde(rename = "ContentBasedDeduplication", skip_serializing_if = "Option::is_none")]
    pub(super) content_based_deduplication: Option<bool>,
    #[serde(rename = "FifoThroughputScope", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_throughput_scope: Option<String>,
}

ref_struct_with_id_methods!(TopicPolicyRef);

#[derive(Debug, Serialize)]
pub struct TopicPolicy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: TopicPolicyProperties,
}
dto_methods!(TopicPolicy);

#[derive(Debug, Serialize)]
pub struct TopicPolicyProperties {
    #[serde(rename = "PolicyDocument")]
    pub(crate) doc: PolicyDocument,
    #[serde(rename = "Topics")]
    pub(super) topics: Vec<Value>,
}

#[derive(Debug, Serialize)]
pub struct Subscription {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: SnsSubscriptionProperties,
}
dto_methods!(Subscription);

#[derive(Debug, Serialize)]
pub struct SnsSubscriptionProperties {
    #[serde(rename = "Protocol")]
    pub(super) protocol: String,
    #[serde(rename = "Endpoint")]
    pub(super) endpoint: Value,
    #[serde(rename = "TopicArn")]
    pub(super) topic_arn: Value
}

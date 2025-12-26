use serde::Serialize;
use serde_json::Value;
use crate::{dto_methods, ref_struct};
use crate::iam::PolicyDocument;
use crate::intrinsic::{get_arn, get_att, get_ref};
use crate::shared::Id;

ref_struct!(TopicRef);

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

pub struct TopicPolicyRef {
    id: Id,
    resource_id: String,
}

impl TopicPolicyRef {
    pub fn new(id: Id, resource_id: String) -> Self {
        Self {
            id,
            resource_id
        }
    }

    pub fn get_id(&self) -> Id {
        self.id.clone()
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }

    pub fn get_arn(&self) -> Value {
        get_arn(self.get_resource_id())
    }

    pub fn get_att(&self, id: &str) -> Value {
        get_att(self.get_resource_id(), id)
    }
}

#[derive(Debug, Serialize)]
pub struct TopicPolicy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: TopicPolicyProperties,
}
dto_methods!(TopicPolicy);

#[derive(Debug, Serialize)]
pub struct TopicPolicyProperties {
    #[serde(rename = "PolicyDocument")]
    pub(super) doc: PolicyDocument,
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

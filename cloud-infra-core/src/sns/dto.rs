use serde::Serialize;
use serde_json::Value;
use crate::ref_struct;
use crate::shared::Id;

ref_struct!(TopicRef);

#[derive(Debug, Serialize)]
pub struct Topic {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: TopicProperties,
}

impl Topic {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct TopicProperties {
    #[serde(rename = "TopicName", skip_serializing_if = "Option::is_none")]
    pub(crate) topic_name: Option<String>,
    #[serde(rename = "FifoTopic", skip_serializing_if = "Option::is_none")]
    pub(crate) fifo_topic: Option<bool>,
    #[serde(rename = "ContentBasedDeduplication", skip_serializing_if = "Option::is_none")]
    pub(crate) content_based_deduplication: Option<bool>,
    #[serde(rename = "FifoThroughputScope", skip_serializing_if = "Option::is_none")]
    pub(crate) fifo_throughput_scope: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Subscription {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: SnsSubscriptionProperties,
}

impl Subscription {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct SnsSubscriptionProperties {
    #[serde(rename = "Protocol")]
    pub(crate) protocol: String,
    #[serde(rename = "Endpoint")]
    pub(crate) endpoint: Value,
    #[serde(rename = "TopicArn")]
    pub(crate) topic_arn: Value
}

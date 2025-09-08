use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;
use crate::shared::Id;

#[derive(Debug, Serialize)]
pub struct SnsTopic {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: SnsTopicProperties,
}

impl SnsTopic {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }
}

#[derive(Debug, Serialize)]
pub struct SnsTopicProperties {
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
pub struct SnsSubscription {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(skip)]
    pub(crate) referenced_ids: Vec<String>,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: SnsSubscriptionProperties,
}

impl SnsSubscription {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_referenced_ids(&self) -> Vec<&str> {
        self.referenced_ids.iter().map(|r| r.as_str()).collect()
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

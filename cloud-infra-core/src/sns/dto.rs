use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;

#[derive(Debug, Serialize)]
pub struct SnsTopic {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: SnsTopicProperties,
}

impl SnsTopic {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_id())
    }
}

#[derive(Debug, Serialize)]
pub struct SnsTopicProperties {
    #[serde(rename = "TopicName", skip_serializing_if = "Option::is_none")]
    pub(crate) topic_name: Option<String>,
    #[serde(rename = "DisplayName", skip_serializing_if = "Option::is_none")]
    pub(crate) display_name: Option<String>,
    #[serde(rename = "FifoTopic", skip_serializing_if = "Option::is_none")]
    pub(crate) fifo_topic: Option<bool>,
    #[serde(rename = "ContentBasedDeduplication", skip_serializing_if = "Option::is_none")]
    pub(crate) content_based_deduplication: Option<bool>,
    #[serde(rename = "Subscription", skip_serializing_if = "Option::is_none")]
    pub(crate) subscription: Option<Vec<SnsSubscription>>,
}

#[derive(Debug, Serialize)]
pub struct SnsSubscription {
    #[serde(rename = "Protocol")]
    pub(crate) protocol: String,
    #[serde(rename = "Endpoint")]
    pub(crate) endpoint: String,
}
use crate::iam::PolicyDocument;
use crate::intrinsic::{get_att, get_ref};
use crate::shared::Id;
use crate::{dto_methods, ref_struct_with_id_methods};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum TopicType {
    #[serde(rename = "AWS::SNS::Topic")]
    TopicType,
}

// custom because get_arn is different
pub struct TopicRef {
    id: Id,
    resource_id: String,
    ref_name: Option<String>,
    arn_value: Option<String>,
}

impl TopicRef {
    pub(crate) fn internal_new(id: Id, resource_id: String) -> Self {
        Self {
            id,
            resource_id,
            ref_name: None,
            arn_value: None,
        }
    }

    pub fn new(id: &str, resource_id: &str, ref_name: &str, arn_value: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            resource_id: resource_id.to_string(),
            ref_name: Some(ref_name.to_string()),
            arn_value: Some(arn_value.to_string()),
        }
    }

    pub(crate) fn get_id(&self) -> &Id {
        &self.id
    }

    pub(crate) fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_ref(&self) -> Value {
        if let Some(val) = &self.ref_name {
            Value::String(val.to_string())
        } else {
            get_ref(self.get_resource_id())
        }
    }

    pub fn get_arn(&self) -> Value {
        if let Some(val) = &self.arn_value {
            Value::String(val.to_string())
        } else {
            self.get_ref()
        }
    }

    pub fn get_att(&self, id: &str) -> Value {
        if self.ref_name.is_some() && self.arn_value.is_some() {
            unimplemented!("get att is not supported for an imported RoleRef")
        } else {
            get_att(self.get_resource_id(), id)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: TopicType,
    #[serde(rename = "Properties")]
    pub(crate) properties: TopicProperties,
}
dto_methods!(Topic);

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicProperties {
    #[serde(rename = "ArchivePolicy", skip_serializing_if = "Option::is_none")]
    pub(crate) archive_policy: Option<Value>,
    #[serde(rename = "ContentBasedDeduplication", skip_serializing_if = "Option::is_none")]
    pub(super) content_based_deduplication: Option<bool>,
    #[serde(rename = "DeliveryStatusLogging", skip_serializing_if = "Option::is_none")]
    pub(super) delivery_status_logging: Option<LoggingConfig>,
    #[serde(rename = "DisplayName", skip_serializing_if = "Option::is_none")]
    pub(super) display_name: Option<String>,
    #[serde(rename = "FifoTopic", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_topic: Option<bool>,
    #[serde(rename = "FifoThroughputScope", skip_serializing_if = "Option::is_none")]
    pub(super) fifo_throughput_scope: Option<String>,
    #[serde(rename = "KmsMasterKeyId", skip_serializing_if = "Option::is_none")]
    pub(super) kms_master_key_id: Option<Value>,
    #[serde(rename = "TopicName", skip_serializing_if = "Option::is_none")]
    pub(super) topic_name: Option<String>,
    #[serde(rename = "TracingConfig", skip_serializing_if = "Option::is_none")]
    pub(super) tracing_config: Option<String>,
    // DataProtectionPolicy
}

ref_struct_with_id_methods!(TopicPolicyRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(rename = "FailureFeedbackRoleArn", skip_serializing_if = "Option::is_none")]
    pub(super) failure_feedback_role_arn: Option<Value>,
    #[serde(rename = "Protocol")]
    pub(super) protocol: String,
    #[serde(rename = "SuccessFeedbackRoleArn", skip_serializing_if = "Option::is_none")]
    pub(super) success_feedback_role_arn: Option<Value>,
    #[serde(rename = "SuccessFeedbackSampleRate", skip_serializing_if = "Option::is_none")]
    pub(super) success_feedback_sample_rate: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum TopicPolicyType {
    #[serde(rename = "AWS::SNS::TopicPolicy")]
    TopicPolicyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicPolicy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: TopicPolicyType,
    #[serde(rename = "Properties")]
    pub(crate) properties: TopicPolicyProperties,
}
dto_methods!(TopicPolicy);

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicPolicyProperties {
    #[serde(rename = "PolicyDocument")]
    pub(crate) doc: PolicyDocument,
    #[serde(rename = "Topics")]
    pub(super) topics: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SubscriptionDtoType {
    #[serde(rename = "AWS::SNS::Subscription")]
    SubscriptionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: SubscriptionDtoType,
    #[serde(rename = "Properties")]
    pub(super) properties: SnsSubscriptionProperties,
}
dto_methods!(Subscription);

#[derive(Debug, Serialize, Deserialize)]
pub struct SnsSubscriptionProperties {
    #[serde(rename = "Endpoint")]
    pub(super) endpoint: Value,
    #[serde(rename = "Protocol")]
    pub(super) protocol: String,
    #[serde(rename = "TopicArn")]
    pub(super) topic_arn: Value,
}

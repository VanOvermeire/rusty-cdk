use serde::Serialize;
use serde_json::Value;
use crate::ref_struct;
use crate::shared::Id;

ref_struct!(TableRef);

#[derive(Debug, Serialize)]
pub struct Table {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: TableProperties,
}

impl Table {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct TableProperties {
    #[serde(rename = "KeySchema")]
    pub(super) key_schema: Vec<KeySchema>,
    #[serde(rename = "AttributeDefinitions")]
    pub(super) attribute_definitions: Vec<AttributeDefinition>,
    #[serde(rename = "BillingMode")]
    pub(super) billing_mode: String,
    #[serde(rename = "ProvisionedThroughput", skip_serializing_if = "Option::is_none")]
    pub(super) provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "OnDemandThroughput", skip_serializing_if = "Option::is_none")]
    pub(super) on_demand_throughput: Option<OnDemandThroughput>,
    // "GlobalSecondaryIndexes" : [ GlobalSecondaryIndex, ... ],
    // "LocalSecondaryIndexes" : [ LocalSecondaryIndex, ... ],
    // "PointInTimeRecoverySpecification" : PointInTimeRecoverySpecification,
    // "ResourcePolicy" : ResourcePolicy,
    // "SSESpecification" : SSESpecification,
    // "StreamSpecification" : StreamSpecification,
    // "TimeToLiveSpecification" : TimeToLiveSpecification,
}

#[derive(Debug, Serialize)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    pub(super) attribute_name: String,
    #[serde(rename = "AttributeType")]
    pub(super) attribute_type: String,
}

#[derive(Debug, Serialize)]
pub struct KeySchema {
    #[serde(rename = "AttributeName")]
    pub(super) attribute_name: String,
    #[serde(rename = "KeyType")]
    pub(super) key_type: String,
}

#[derive(Debug, Serialize)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    pub(super) read_capacity: u32,
    #[serde(rename = "WriteCapacityUnits")]
    pub(super) write_capacity: u32,
}

#[derive(Debug, Serialize)]
pub struct OnDemandThroughput {
    #[serde(rename = "MaxReadRequestUnits", skip_serializing_if = "Option::is_none")]
    pub(super) max_read_capacity: Option<u32>,
    #[serde(rename = "MaxWriteRequestUnits", skip_serializing_if = "Option::is_none")]
    pub(super) max_write_capacity: Option<u32>,
}

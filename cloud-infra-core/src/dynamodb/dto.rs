use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;
use crate::shared::Id;

#[derive(Debug, Serialize)]
pub struct DynamoDBTable {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: DynamoDBTableProperties,
}

impl DynamoDBTable {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    // TODO decide whether we want this indirection (currently sometimes used, sometimes not)
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }
}

#[derive(Debug, Serialize)]
pub struct DynamoDBTableProperties {
    #[serde(rename = "KeySchema")]
    pub(crate) key_schema: Vec<KeySchema>,
    #[serde(rename = "AttributeDefinitions")]
    pub(crate) attribute_definitions: Vec<AttributeDefinition>,
    #[serde(rename = "BillingMode")]
    pub(crate) billing_mode: String,
    #[serde(rename = "ProvisionedThroughput", skip_serializing_if = "Option::is_none")]
    pub(crate) provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "OnDemandThroughput", skip_serializing_if = "Option::is_none")]
    pub(crate) on_demand_throughput: Option<OnDemandThroughput>,
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
    pub(crate) attribute_name: String,
    #[serde(rename = "AttributeType")]
    pub(crate) attribute_type: String,
}

#[derive(Debug, Serialize)]
pub struct KeySchema {
    #[serde(rename = "AttributeName")]
    pub(crate) attribute_name: String,
    #[serde(rename = "KeyType")]
    pub(crate) key_type: String,
}

#[derive(Debug, Serialize)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    pub(crate) read_capacity: u32,
    #[serde(rename = "WriteCapacityUnits")]
    pub(crate) write_capacity: u32,
}

#[derive(Debug, Serialize)]
pub struct OnDemandThroughput {
    #[serde(rename = "MaxReadRequestUnits", skip_serializing_if = "Option::is_none")]
    pub(crate) max_read_capacity: Option<u32>,
    #[serde(rename = "MaxWriteRequestUnits", skip_serializing_if = "Option::is_none")]
    pub(crate) max_write_capacity: Option<u32>,
}

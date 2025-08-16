use serde::Serialize;

#[derive(Serialize)]
pub struct DynamoDBTable {
    #[serde(skip)]
    id: String,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(rename = "Properties")]
    properties: DynamoDBTableProperties,
}

impl DynamoDBTable {
    pub(crate) fn new(id: String, properties: DynamoDBTableProperties) -> Self {
        Self {
            id,
            r#type: "AWS::DynamoDB::Table".to_string(),
            properties,
        }
    }

    pub(crate) fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    pub(crate) attribute_name: String,
    #[serde(rename = "AttributeType")]
    pub(crate) attribute_type: String,
}

#[derive(Serialize)]
pub struct KeySchema {
    #[serde(rename = "AttributeName")]
    pub(crate) attribute_name: String,
    #[serde(rename = "KeyType")]
    pub(crate) key_type: String,
}

#[derive(Serialize)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    pub(crate) read_capacity: u32,
    #[serde(rename = "WriteCapacityUnits")]
    pub(crate) write_capacity: u32,
}

#[derive(Serialize)]
pub struct OnDemandThroughput {
    #[serde(rename = "MaxReadRequestUnits", skip_serializing_if = "Option::is_none")]
    pub(crate) max_read_capacity: Option<u32>,
    #[serde(rename = "MaxWriteRequestUnits", skip_serializing_if = "Option::is_none")]
    pub(crate) max_write_capacity: Option<u32>,
}

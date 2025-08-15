use serde::Serialize;

#[derive(Serialize)]
pub struct DynamoDBTable {
    #[serde(skip)]
    pub id: String,
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
    
    pub(crate) fn get_id(&self) -> String {
        self.id.to_string()
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
    #[serde(rename = "ReadCapacityUnits")]
    pub(crate) read_capacity: Option<u32>,
    #[serde(rename = "WriteCapacityUnits")]
    pub(crate) write_capacity: Option<u32>,
    #[serde(rename = "MaxReadRequestUnits")]
    pub(crate) max_read_capacity: Option<u32>,
    #[serde(rename = "MaxWriteRequestUnits")]
    pub(crate) max_write_capacity: Option<u32>,
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
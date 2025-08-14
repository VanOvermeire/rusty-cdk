use serde::Serialize;
use std::collections::HashMap;
use crate::dynamodb::DynamoDBTable;

#[derive(Serialize)]
pub struct Stack {
    #[serde(rename = "Resources")]
    resources: HashMap<String, Resource>,
}

impl Stack {
    pub fn new(resources: Vec<Resource>) -> Self {
        let resources = resources.into_iter().map(|r| (r.get_id(), r)).collect();
        Self {
            resources
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Resource {
    DynamoDBTable(DynamoDBTable),
}

impl Resource {
    fn get_id(&self) -> String {
        match self {
            // TODO real id
            Resource::DynamoDBTable(table) => "dynamoTableId".to_string()
        }
    }
}

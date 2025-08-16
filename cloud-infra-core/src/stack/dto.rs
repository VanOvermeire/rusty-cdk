use serde::Serialize;
use std::collections::HashMap;
use rand::Rng;
use crate::dynamodb::DynamoDBTable;

#[derive(Serialize)]
pub struct Stack {
    #[serde(rename = "Resources")]
    resources: HashMap<String, Resource>,
}

impl Stack {
    pub fn new(resources: Vec<Resource>) -> Self {
        let resources = resources.into_iter().map(|r| (r.get_id().to_string(), r)).collect();
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
    fn get_id(&self) -> &str {
        match self {
            Resource::DynamoDBTable(table) => table.get_id()
        }
    }
    
    pub fn generate_id(resource_name: &str) -> String {
        let mut rng = rand::rng();
        let random_suffix: u32 = rng.random();
        format!("{resource_name}{random_suffix}")
    }
}

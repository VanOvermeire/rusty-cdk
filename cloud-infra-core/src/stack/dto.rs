use serde::Serialize;
use std::collections::HashMap;
use rand::Rng;
use crate::dynamodb::DynamoDBTable;
use crate::iam::IamRole;
use crate::lambda::LambdaFunction;

#[derive(Clone)]
pub struct Asset {
    pub s3_bucket: String,
    pub s3_key: String,
    pub path: String
}

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
    
    pub fn get_assets(&self) -> Vec<Asset> {
        self.resources.iter().flat_map(|r| match r.1 {
            Resource::DynamoDBTable(_) => vec![],
            Resource::IamRole(_) => vec![],
            Resource::LambdaFunction(l) => vec![l.asset.clone()] // see if we can avoid the clone
        }).collect()
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Resource {
    DynamoDBTable(DynamoDBTable),
    LambdaFunction(LambdaFunction),
    IamRole(IamRole),
}

impl Resource {
    fn get_id(&self) -> &str {
        match self {
            Resource::DynamoDBTable(t) => t.get_id(),
            Resource::LambdaFunction(f) => f.get_id(),
            Resource::IamRole(r) => r.get_id(),
        }
    }
    
    pub fn generate_id(resource_name: &str) -> String {
        let mut rng = rand::rng();
        let random_suffix: u32 = rng.random();
        format!("{resource_name}{random_suffix}")
    }
}

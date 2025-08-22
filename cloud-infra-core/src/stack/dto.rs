use serde::Serialize;
use std::collections::HashMap;
use rand::Rng;
use crate::dynamodb::DynamoDBTable;
use crate::iam::IamRole;
use crate::lambda::LambdaFunction;

#[derive(Debug, Clone)]
pub struct Asset {
    pub s3_bucket: String,
    pub s3_key: String,
    pub path: String
}

#[derive(Debug, Serialize)]
pub struct Stack {
    #[serde(rename = "Resources")]
    pub(crate) resources: HashMap<String, Resource>,
}

impl Stack {
    pub fn get_assets(&self) -> Vec<Asset> {
        self.resources.values().flat_map(|r| match r {
            Resource::DynamoDBTable(_) => vec![],
            Resource::IamRole(_) => vec![],
            Resource::LambdaFunction(l) => vec![l.asset.clone()] // see if we can avoid the clone
        }).collect()
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Resource {
    DynamoDBTable(DynamoDBTable),
    LambdaFunction(LambdaFunction),
    IamRole(IamRole),
}

impl Resource {
    pub(crate) fn get_id(&self) -> &str {
        match self {
            Resource::DynamoDBTable(t) => t.get_id(),
            Resource::LambdaFunction(f) => f.get_id(),
            Resource::IamRole(r) => r.get_id(),
        }
    }
    
    pub(crate) fn generate_id(resource_name: &str) -> String {
        let mut rng = rand::rng();
        let random_suffix: u32 = rng.random();
        format!("{resource_name}{random_suffix}")
    }
}

impl From<DynamoDBTable> for Resource {
    fn from(value: DynamoDBTable) -> Self {
        Resource::DynamoDBTable(value)
    }
}

impl From<LambdaFunction> for Resource {
    fn from(value: LambdaFunction) -> Self {
        Resource::LambdaFunction(value)
    }
}

impl From<IamRole> for Resource {
    fn from(value: IamRole) -> Self {
        Resource::IamRole(value)
    }
}

use crate::dynamodb::DynamoDBTable;
use crate::iam::IamRole;
use crate::lambda::{EventSourceMapping, LambdaFunction};
use crate::sqs::SqsQueue;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Asset {
    pub s3_bucket: String,
    pub s3_key: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct Stack {
    #[serde(rename = "Resources")]
    pub(crate) resources: HashMap<String, Resource>,
}

impl Stack {
    pub fn get_assets(&self) -> Vec<Asset> {
        self.resources
            .values()
            .flat_map(|r| match r {
                Resource::LambdaFunction(l) => vec![l.asset.clone()], // see if we can avoid the clone
                Resource::DynamoDBTable(_) => vec![],
                Resource::IamRole(_) => vec![],
                Resource::SqsQueue(_) => vec![],
                Resource::EventSourceMapping(_) => vec![],
            })
            .collect()
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Resource {
    DynamoDBTable(DynamoDBTable),
    LambdaFunction(LambdaFunction),
    SqsQueue(SqsQueue),
    EventSourceMapping(EventSourceMapping),
    IamRole(IamRole),
}

impl Resource {
    pub fn get_id(&self) -> &str {
        match self {
            Resource::DynamoDBTable(t) => t.get_id(),
            Resource::LambdaFunction(f) => f.get_id(),
            Resource::IamRole(r) => r.get_id(),
            Resource::SqsQueue(q) => q.get_id(),
            Resource::EventSourceMapping(m) => m.get_id(), 
        }
    }
    
    pub fn get_ref_ids(&self) -> Vec<&str> {
        match self {
            Resource::LambdaFunction(f) => f.get_referenced_ids(),
            _ => vec![] // TODO
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

impl From<SqsQueue> for Resource {
    fn from(value: SqsQueue) -> Self {
        Resource::SqsQueue(value)
    }
}
impl From<EventSourceMapping> for Resource {
    fn from(value: EventSourceMapping) -> Self {
        Resource::EventSourceMapping(value)
    }
}

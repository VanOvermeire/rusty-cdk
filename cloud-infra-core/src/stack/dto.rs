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
            Resource::DynamoDBTable(_) => vec![],
            Resource::SqsQueue(_) => vec![],
            Resource::EventSourceMapping(_) => vec![],
            Resource::IamRole(_) => vec![],
        }
    }

    pub(crate) fn generate_id(resource_name: &str) -> String {
        let mut rng = rand::rng();
        let random_suffix: u32 = rng.random();
        format!("{resource_name}{random_suffix}")
    }
}

macro_rules! from_resource {
    ($name:ident) => {
        impl From<$name> for Resource {
            fn from(value: $name) -> Self {
                Resource::$name(value)
            }
        }
    };
}

from_resource!(DynamoDBTable);
from_resource!(LambdaFunction);
from_resource!(IamRole);
from_resource!(SqsQueue);
from_resource!(EventSourceMapping);

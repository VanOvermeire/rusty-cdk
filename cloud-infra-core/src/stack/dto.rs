use crate::apigateway::dto::{ApiGatewayV2Api, ApiGatewayV2Integration, ApiGatewayV2Route, ApiGatewayV2Stage};
use crate::cloudwatch::LogGroup;
use crate::dynamodb::DynamoDBTable;
use crate::iam::IamRole;
use crate::lambda::{EventSourceMapping, LambdaFunction, LambdaPermission};
use crate::s3::dto::S3Bucket;
use crate::shared::Id;
use crate::sns::dto::{SnsSubscription, SnsTopic};
use crate::sqs::SqsQueue;
use crate::stack::StackBuilder;
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
    #[serde(rename = "Metadata")]
    pub(crate) metadata: HashMap<String, String>,
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
                Resource::LogGroup(_) => vec![],
                Resource::SnsTopic(_) => vec![],
                Resource::SnsSubscription(_) => vec![],
                Resource::LambdaPermission(_) => vec![],
                Resource::ApiGatewayV2Api(_) => vec![],
                Resource::ApiGatewayV2Stage(_) => vec![],
                Resource::ApiGatewayV2Route(_) => vec![],
                Resource::ApiGatewayV2Integration(_) => vec![],
                Resource::S3Bucket(_) => vec![],
            })
            .collect()
    }

    pub fn synth(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Could not serialize stack: {e:?}"))
    }

    pub fn update_resource_ids_for_existing_stack(&mut self, existing_ids_with_resource_ids: HashMap<String, String>) {
        let current_ids: HashMap<String, String> = self
            .resources
            .iter()
            .map(|(resource_id, resource)| (resource.get_id().0, resource_id.to_string()))
            .collect();

        existing_ids_with_resource_ids
            .into_iter()
            .filter(|(existing_id, _)| current_ids.contains_key(existing_id))
            .for_each(|(existing_id, existing_resource_id)| {
                let current_stack_resource_id = current_ids.get(&existing_id).expect("existence to be checked by filter");
                let removed = self.resources.remove(current_stack_resource_id).expect("resource to exist in stack resources");
                self.resources.insert(existing_resource_id.clone(), removed);
                self.metadata.insert(existing_id, existing_resource_id);
            });
    }
}

impl TryFrom<Vec<Resource>> for Stack {
    type Error = String;

    fn try_from(resources: Vec<Resource>) -> Result<Self, Self::Error> {
        let stack_builder = StackBuilder::new().add_resources(resources);
        let stack = stack_builder.build().map_err(|e| e.to_string())?;
        Ok(stack)
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Resource {
    S3Bucket(S3Bucket),
    DynamoDBTable(DynamoDBTable),
    LambdaFunction(LambdaFunction),
    LogGroup(LogGroup),
    SqsQueue(SqsQueue),
    SnsTopic(SnsTopic),
    SnsSubscription(SnsSubscription),
    LambdaPermission(LambdaPermission),
    EventSourceMapping(EventSourceMapping),
    IamRole(IamRole),
    ApiGatewayV2Api(ApiGatewayV2Api),
    ApiGatewayV2Stage(ApiGatewayV2Stage),
    ApiGatewayV2Route(ApiGatewayV2Route),
    ApiGatewayV2Integration(ApiGatewayV2Integration),
}

impl Resource {
    pub fn get_id(&self) -> Id {
        match self {
            Resource::S3Bucket(s) => s.get_id().clone(),
            _ => todo!(),
        }
    }

    pub fn get_resource_id(&self) -> &str {
        match self {
            Resource::DynamoDBTable(t) => t.get_resource_id(),
            Resource::LambdaFunction(f) => f.get_resource_id(),
            Resource::IamRole(r) => r.get_resource_id(),
            Resource::SqsQueue(q) => q.get_resource_id(),
            Resource::EventSourceMapping(m) => m.get_resource_id(),
            Resource::LogGroup(l) => l.get_resource_id(),
            Resource::SnsTopic(s) => s.get_resource_id(),
            Resource::SnsSubscription(s) => s.get_resource_id(),
            Resource::LambdaPermission(l) => l.get_resource_id(),
            Resource::ApiGatewayV2Api(a) => a.get_resource_id(),
            Resource::ApiGatewayV2Stage(s) => s.get_resource_id(),
            Resource::ApiGatewayV2Route(r) => r.get_resource_id(),
            Resource::ApiGatewayV2Integration(i) => i.get_resource_id(),
            Resource::S3Bucket(s) => s.get_resource_id(),
        }
    }

    pub fn get_refenced_ids(&self) -> Vec<&str> {
        match self {
            // TODO the other resources (except when references are impossible)
            Resource::LambdaFunction(f) => f.get_referenced_ids(),
            Resource::SnsSubscription(s) => s.get_referenced_ids(),
            Resource::LambdaPermission(l) => l.get_referenced_ids(),
            Resource::DynamoDBTable(_) => vec![],
            Resource::SqsQueue(_) => vec![],
            Resource::EventSourceMapping(_) => vec![],
            Resource::IamRole(_) => vec![],
            Resource::LogGroup(_) => vec![],
            Resource::SnsTopic(_) => vec![],
            Resource::ApiGatewayV2Api(_) => vec![],
            Resource::ApiGatewayV2Stage(_) => vec![],
            Resource::ApiGatewayV2Route(r) => r.get_referenced_ids(),
            Resource::ApiGatewayV2Integration(i) => i.get_referenced_ids(),
            Resource::S3Bucket(_) => vec![],
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

from_resource!(S3Bucket);
from_resource!(DynamoDBTable);
from_resource!(LambdaFunction);
from_resource!(IamRole);
from_resource!(LogGroup);
from_resource!(SqsQueue);
from_resource!(SnsTopic);
from_resource!(EventSourceMapping);
from_resource!(LambdaPermission);
from_resource!(SnsSubscription);
from_resource!(ApiGatewayV2Api);
from_resource!(ApiGatewayV2Stage);
from_resource!(ApiGatewayV2Route);
from_resource!(ApiGatewayV2Integration);

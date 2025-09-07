use crate::dynamodb::DynamoDBTable;
use crate::iam::IamRole;
use crate::lambda::{EventSourceMapping, LambdaFunction, LambdaPermission};
use crate::sqs::SqsQueue;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use crate::apigateway::dto::{ApiGatewayV2Api, ApiGatewayV2Integration, ApiGatewayV2Route, ApiGatewayV2Stage};
use crate::cloudwatch::LogGroup;
use crate::s3::dto::S3Bucket;
use crate::sns::dto::{SnsSubscription, SnsTopic};
use crate::stack::StackBuilder;

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
            Resource::S3Bucket(_) => vec![]
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

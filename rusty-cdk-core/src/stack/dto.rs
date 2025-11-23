use crate::apigateway::dto::{ApiGatewayV2Api, ApiGatewayV2Integration, ApiGatewayV2Route, ApiGatewayV2Stage};
use crate::cloudwatch::LogGroup;
use crate::dynamodb::Table;
use crate::iam::Role;
use crate::lambda::{EventSourceMapping, Function, Permission};
use crate::s3::dto::{Bucket, BucketPolicy};
use crate::secretsmanager::dto::Secret;
use crate::shared::Id;
use crate::sns::dto::{Subscription, Topic};
use crate::sqs::Queue;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use crate::appconfig::dto::{Application, ConfigurationProfile, DeploymentStrategy, Environment};
use crate::cloudfront::{CachePolicy, Distribution, OriginAccessControl};

#[derive(Debug, Clone)]
pub struct Asset {
    pub s3_bucket: String,
    pub s3_key: String,
    pub path: String,
}

/// Represents a CloudFormation stack containing AWS resources and their configurations.
///
/// A `Stack` is the core abstraction for defining and managing AWS infrastructure.
/// It contains a collection of AWS resources (such as Lambda functions, S3 buckets, DynamoDB tables, etc.) 
/// that are deployed together as a single unit in AWS CloudFormation.
///
/// # Usage
///
/// Stacks are created using the [`StackBuilder`](crate::stack::StackBuilder), which provides a fluent interface for adding resources. 
/// Once built, a stack can be:
/// - Synthesized into a CloudFormation template JSON using [`synth()`](Stack::synth)
/// - Deployed to AWS using the deployment utilities (`deploy`)
///
/// # Example
///
/// ```
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::sqs::QueueBuilder;
///
/// let mut stack_builder = StackBuilder::new();
///
/// // Add resources to the stack
/// QueueBuilder::new("my-queue")
///     .standard_queue()
///     .build(&mut stack_builder);
///
/// // Build the stack
/// let stack = stack_builder.build().unwrap();
///
/// // Synthesize to CloudFormation template
/// let template_json = stack.synth().unwrap();
/// ```
///
/// # Serialization
///
/// The stack is serialized to CloudFormation-compatible JSON format, with:
/// - `Resources`: The AWS resources map
/// - `Metadata`: Additional metadata for resource management
/// - Tags are *not* serialized directly
#[derive(Debug, Serialize)]
pub struct Stack {
    #[serde(skip)]
    pub(crate) to_replace: Vec<(String, String)>,
    #[serde(skip)]
    pub(crate) tags: Vec<(String, String)>,
    #[serde(rename = "Resources")]
    pub(crate) resources: HashMap<String, Resource>,
    #[serde(rename = "Metadata")]
    pub(crate) metadata: HashMap<String, String>,
}

impl Stack {
    pub fn get_tags(&self) -> Vec<(String, String)> {
        self.tags.clone()
    }
    
    pub fn get_assets(&self) -> Vec<Asset> {
        self.resources
            .values()
            .flat_map(|r| match r {
                Resource::Function(l) => vec![l.asset.clone()], // see if we can avoid the clone
                _ => vec![],
            })
            .collect()
    }

    /// Synthesizes the stack into a CloudFormation template JSON string.
    ///
    /// This method converts the stack and all its resources into a JSON-formatted
    /// CloudFormation template that can be deployed to AWS using the AWS CLI, SDKs,
    /// or the AWS Console.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - A JSON-formatted CloudFormation template string
    /// * `Err(String)` - An error message if serialization fails
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_cdk_core::stack::StackBuilder;
    /// use rusty_cdk_core::sqs::QueueBuilder;
    ///
    /// let mut stack_builder = StackBuilder::new();
    ///
    /// // Add resources to the stack
    /// QueueBuilder::new("my-queue")
    ///     .standard_queue()
    ///     .build(&mut stack_builder);
    ///
    /// // Build the stack
    /// let stack = stack_builder.build().unwrap();
    ///
    /// // Synthesize to CloudFormation template
    /// let template_json = stack.synth().unwrap();
    /// ```
    ///
    /// # Usage with AWS Tools
    ///
    /// The synthesized template can be used with:
    /// - AWS CLI: `aws cloudformation create-stack --template-body file://template.json`
    /// - AWS SDKs: Pass the template string to the CloudFormation client
    /// - AWS Console: Upload the template file directly
    pub fn synth(&self) -> Result<String, String> {
        let mut naive_synth = serde_json::to_string(self).map_err(|e| format!("Could not serialize stack: {e:#?}"))?;
        // nicer way to do this? for example, a method on each DTO to look for possible arns/refs (`Value`) and replace them if needed. referenced ids should help a bit
        self.to_replace.iter().for_each(|(current, new)| {
            naive_synth = naive_synth.replace(current, new);
        });

        Ok(naive_synth)
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
                let removed = self
                    .resources
                    .remove(current_stack_resource_id)
                    .expect("resource to exist in stack resources");
                self.resources.insert(existing_resource_id.clone(), removed);
                self.metadata.insert(existing_id, existing_resource_id.clone());
                self.to_replace.push((current_stack_resource_id.to_string(), existing_resource_id));
            });
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Resource {
    Application(Application),
    Bucket(Bucket),
    BucketPolicy(BucketPolicy),
    ConfigurationProfile(ConfigurationProfile),
    DeploymentStrategy(DeploymentStrategy),
    Environment(Environment),
    Table(Table),
    Function(Function),
    LogGroup(LogGroup),
    Queue(Queue),
    Topic(Topic),
    Subscription(Subscription),
    Permission(Permission),
    EventSourceMapping(EventSourceMapping),
    Role(Role),
    ApiGatewayV2Api(ApiGatewayV2Api),
    ApiGatewayV2Stage(ApiGatewayV2Stage),
    ApiGatewayV2Route(ApiGatewayV2Route),
    ApiGatewayV2Integration(ApiGatewayV2Integration),
    Secret(Secret),
    Distribution(Distribution),
    CachePolicy(CachePolicy),
    OriginAccessControl(OriginAccessControl),
}

impl Resource {
    pub fn get_id(&self) -> Id {
        let id = match self {
            Resource::Bucket(r) => r.get_id(),
            Resource::BucketPolicy(r) => r.get_id(),
            Resource::Table(r) => r.get_id(),
            Resource::Function(r) => r.get_id(),
            Resource::LogGroup(r) => r.get_id(),
            Resource::Queue(r) => r.get_id(),
            Resource::Topic(r) => r.get_id(),
            Resource::Subscription(r) => r.get_id(),
            Resource::Permission(r) => r.get_id(),
            Resource::EventSourceMapping(r) => r.get_id(),
            Resource::Role(r) => r.get_id(),
            Resource::ApiGatewayV2Api(r) => r.get_id(),
            Resource::ApiGatewayV2Stage(r) => r.get_id(),
            Resource::ApiGatewayV2Route(r) => r.get_id(),
            Resource::ApiGatewayV2Integration(r) => r.get_id(),
            Resource::Secret(r) => r.get_id(),
            Resource::Distribution(r) => r.get_id(),
            Resource::CachePolicy(r) => r.get_id(),
            Resource::OriginAccessControl(r) => r.get_id(),
            Resource::Application(r) => r.get_id(),
            Resource::ConfigurationProfile(r) => r.get_id(),
            Resource::DeploymentStrategy(r) => r.get_id(),
            Resource::Environment(r) => r.get_id(),
        };
        id.clone()
    }

    pub fn get_resource_id(&self) -> &str {
        match self {
            Resource::Bucket(r) => r.get_resource_id(),
            Resource::BucketPolicy(r) => r.get_resource_id(),
            Resource::Table(t) => t.get_resource_id(),
            Resource::Function(r) => r.get_resource_id(),
            Resource::Role(r) => r.get_resource_id(),
            Resource::Queue(r) => r.get_resource_id(),
            Resource::EventSourceMapping(r) => r.get_resource_id(),
            Resource::LogGroup(r) => r.get_resource_id(),
            Resource::Topic(r) => r.get_resource_id(),
            Resource::Subscription(r) => r.get_resource_id(),
            Resource::Permission(r) => r.get_resource_id(),
            Resource::ApiGatewayV2Api(r) => r.get_resource_id(),
            Resource::ApiGatewayV2Stage(r) => r.get_resource_id(),
            Resource::ApiGatewayV2Route(r) => r.get_resource_id(),
            Resource::ApiGatewayV2Integration(r) => r.get_resource_id(),
            Resource::Secret(r) => r.get_resource_id(),
            Resource::Distribution(r) => r.get_resource_id(),
            Resource::CachePolicy(r) => r.get_resource_id(),
            Resource::OriginAccessControl(r) => r.get_resource_id(),
            Resource::Application(r) => r.get_resource_id(),
            Resource::ConfigurationProfile(r) => r.get_resource_id(),
            Resource::DeploymentStrategy(r) => r.get_resource_id(),
            Resource::Environment(r) => r.get_resource_id(),
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

from_resource!(Application);
from_resource!(Bucket);
from_resource!(BucketPolicy);
from_resource!(ConfigurationProfile);
from_resource!(DeploymentStrategy);
from_resource!(Environment);
from_resource!(Table);
from_resource!(Function);
from_resource!(Role);
from_resource!(LogGroup);
from_resource!(Queue);
from_resource!(Topic);
from_resource!(EventSourceMapping);
from_resource!(Permission);
from_resource!(Subscription);
from_resource!(ApiGatewayV2Api);
from_resource!(ApiGatewayV2Stage);
from_resource!(ApiGatewayV2Route);
from_resource!(ApiGatewayV2Integration);
from_resource!(Secret);
from_resource!(Distribution);
from_resource!(CachePolicy);
from_resource!(OriginAccessControl);

#[cfg(test)]
mod tests {
    use crate::sns::builder::TopicBuilder;
    use crate::sqs::QueueBuilder;
    use crate::stack::StackBuilder;
    use std::collections::HashMap;

    #[test]
    fn should_do_nothing_for_empty_stack_and_empty_existing_ids() {
        let mut stack_builder = StackBuilder::new().build().unwrap();
        let existing_ids = HashMap::new();

        stack_builder.update_resource_ids_for_existing_stack(existing_ids);

        assert_eq!(stack_builder.resources.len(), 0);
        assert_eq!(stack_builder.metadata.len(), 0);
        assert_eq!(stack_builder.to_replace.len(), 0);
    }

    #[test]
    fn should_do_nothing_for_empty_stack() {
        let mut stack_builder = StackBuilder::new().build().unwrap();
        let mut existing_ids = HashMap::new();
        existing_ids.insert("fun".to_string(), "abc123".to_string());

        stack_builder.update_resource_ids_for_existing_stack(existing_ids);

        assert_eq!(stack_builder.resources.len(), 0);
        assert_eq!(stack_builder.metadata.len(), 0);
        assert_eq!(stack_builder.to_replace.len(), 0);
    }

    #[test]
    fn should_replace_topic_resource_id_with_the_existing_id() {
        let mut stack_builder = StackBuilder::new();
        TopicBuilder::new("topic").build(&mut stack_builder);
        let mut existing_ids = HashMap::new();
        existing_ids.insert("topic".to_string(), "abc123".to_string());
        let mut stack = stack_builder.build().unwrap();

        stack.update_resource_ids_for_existing_stack(existing_ids);

        assert_eq!(stack.resources.len(), 1);
        assert_eq!(stack.to_replace.len(), 1);
        assert_eq!(stack.metadata.len(), 1);
        assert_eq!(stack.metadata.get("topic").unwrap(), &"abc123".to_string());
    }

    #[test]
    fn should_replace_topic_resource_id_with_the_existing_id_keeping_new_queue_id() {
        let mut stack_builder = StackBuilder::new();
        TopicBuilder::new("topic").build(&mut stack_builder);
        QueueBuilder::new("queue").standard_queue().build(&mut stack_builder);
        let mut existing_ids = HashMap::new();
        existing_ids.insert("topic".to_string(), "abc123".to_string());
        let mut stack = stack_builder.build().unwrap();

        stack.update_resource_ids_for_existing_stack(existing_ids);

        assert_eq!(stack.resources.len(), 2);
        assert_eq!(stack.to_replace.len(), 1);
        assert_eq!(stack.metadata.len(), 2);
        assert_eq!(stack.metadata.get("topic").unwrap(), &"abc123".to_string());
    }
}

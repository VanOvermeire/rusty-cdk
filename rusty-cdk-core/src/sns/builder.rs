use crate::iam::PolicyDocument;
use crate::intrinsic::{get_arn, get_ref};
use crate::lambda::{FunctionRef, PermissionBuilder};
use crate::shared::{Id, TOPIC_POLICY_ID_SUFFIX};
use crate::sns::{SnsSubscriptionProperties, Subscription, Topic, TopicPolicy, TopicPolicyProperties, TopicPolicyRef, TopicProperties, TopicRef};
use crate::stack::{Resource, StackBuilder};
use crate::type_state;
use crate::wrappers::{LambdaPermissionAction, StringWithOnlyAlphaNumericsUnderscoresAndHyphens};
use serde_json::Value;
use std::marker::PhantomData;

const FIFO_SUFFIX: &str = ".fifo";

#[derive(Debug, Clone)]
pub enum FifoThroughputScope {
    Topic,
    MessageGroup
}

pub enum SubscriptionType<'a> {
    Lambda(&'a FunctionRef)
}

impl From<FifoThroughputScope> for String {
    fn from(value: FifoThroughputScope) -> Self {
        match value {
            FifoThroughputScope::Topic => "Topic".to_string(),
            FifoThroughputScope::MessageGroup => "MessageGroup".to_string(),
        }
    }
}

type_state!(
    TopicBuilderState,
    StartState,
    StandardStateWithSubscriptions,
    FifoState,
    FifoStateWithSubscriptions,
);

/// Builder for SNS topics.
///
/// Supports both standard and FIFO topics with Lambda subscriptions.
/// FIFO topics have additional configuration for deduplication and throughput.
///
/// # Example
///
/// ```rust,no_run
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::sns::{TopicBuilder, SubscriptionType};
/// # use rusty_cdk_core::lambda::{FunctionBuilder, Architecture, Runtime, Zip};
/// # use rusty_cdk_core::wrappers::*;
/// # use rusty_cdk_macros::{memory, timeout, zip_file};
///
/// let mut stack_builder = StackBuilder::new();
///
/// // Create a simple topic without subscriptions
/// let simple_topic = TopicBuilder::new("simple-topic")
///     .build(&mut stack_builder);
/// 
/// let function = unimplemented!("create a function");
///
/// // Create a topic with a Lambda subscription
/// let topic = TopicBuilder::new("my-topic")
///     .add_subscription(SubscriptionType::Lambda(&function))
///     .build(&mut stack_builder);
///
/// ```
pub struct TopicBuilder<T: TopicBuilderState> {
    state: PhantomData<T>,
    id: Id,
    topic_name: Option<String>,
    content_based_deduplication: Option<bool>,
    fifo_throughput_scope: Option<FifoThroughputScope>,
    topic_policy_doc: Option<PolicyDocument>,
    lambda_subscription_ids: Vec<(Id, String)>,
}

impl TopicBuilder<StartState> {
    /// Creates a new SNS topic builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the topic
    pub fn new(id: &str) -> Self {
        Self {
            state: Default::default(),
            id: Id(id.to_string()),
            topic_name: None,
            content_based_deduplication: None,
            fifo_throughput_scope: None,
            topic_policy_doc: None,
            lambda_subscription_ids: vec![],
        }
    }

    /// Adds a subscription to the topic.
    ///
    /// For Lambda subscriptions, automatically creates the necessary permission.
    pub fn add_subscription(mut self, subscription: SubscriptionType) -> TopicBuilder<StandardStateWithSubscriptions> {
        self.add_subscription_internal(subscription);

        TopicBuilder {
            state: Default::default(),
            id: self.id,
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            topic_policy_doc: self.topic_policy_doc,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> TopicRef {
        self.build_internal(false, stack_builder)
    }
}

impl TopicBuilder<StandardStateWithSubscriptions> {
    pub fn add_subscription(mut self, subscription: SubscriptionType) -> TopicBuilder<StandardStateWithSubscriptions> {
        self.add_subscription_internal(subscription);

        TopicBuilder {
            state: Default::default(),
            id: self.id,
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            topic_policy_doc: self.topic_policy_doc,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> TopicRef {
        self.build_internal(false, stack_builder)
    }
}

impl<T: TopicBuilderState> TopicBuilder<T> {
    pub fn topic_name(self, topic_name: StringWithOnlyAlphaNumericsUnderscoresAndHyphens) -> TopicBuilder<T> {
        TopicBuilder {
            topic_name: Some(topic_name.0),
            id: self.id,
            state: Default::default(),
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            topic_policy_doc: self.topic_policy_doc,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    pub fn fifo(self) -> TopicBuilder<FifoState> {
        TopicBuilder {
            state: Default::default(),
            id: self.id,
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            topic_policy_doc: self.topic_policy_doc,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    /// Adds an SNS Topic Policy for this topic.
    /// The code will automatically set the `resources` section of the `PolicyDocument` to the ARN of this queue, so there's no need to pass that in.
    pub fn topic_policy(self, doc: PolicyDocument) -> TopicBuilder<T> {
        TopicBuilder {
            topic_policy_doc: Some(doc),
            topic_name: self.topic_name,
            id: self.id,
            state: Default::default(),
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }
    
    fn add_subscription_internal(&mut self, subscription: SubscriptionType) {
        match subscription {
            SubscriptionType::Lambda(l) => self.lambda_subscription_ids.push((l.get_id().clone(), l.get_resource_id().to_string()))
        };
    }
    
    fn build_internal(self, fifo: bool, stack_builder: &mut StackBuilder) -> TopicRef {
        let topic_resource_id = Resource::generate_id("SnsTopic");
        
        self.lambda_subscription_ids.iter().for_each(|(to_subscribe_id, to_subscribe_resource_id)| {
            let subscription_id = Id::combine_ids(&self.id, to_subscribe_id);
            let subscription_resource_id = Resource::generate_id("SnsSubscription");
            
            PermissionBuilder::new(&Id::generate_id(&subscription_id, "Permission"), LambdaPermissionAction("lambda:InvokeFunction".to_string()), get_arn(to_subscribe_resource_id), "sns.amazonaws.com")
                .source_arn(get_ref(&topic_resource_id))
                .build(stack_builder);

            let subscription = Subscription {
                id: subscription_id,
                resource_id: subscription_resource_id,
                r#type: "AWS::SNS::Subscription".to_string(),
                properties: SnsSubscriptionProperties {
                    protocol: "lambda".to_string(),
                    endpoint: get_arn(to_subscribe_resource_id),
                    topic_arn: get_ref(&topic_resource_id),
                },
            };

            stack_builder.add_resource(subscription);
        });
        
        let properties = TopicProperties {
            topic_name: self.topic_name,
            fifo_topic: Some(fifo),
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope.map(Into::into),
        };

        let topic_ref = TopicRef::new(self.id.clone(), topic_resource_id.to_string());
        
        if let Some(mut policy) = self.topic_policy_doc {
            for statement in &mut policy.statements {
                // point the statements of this policy to the queue
                statement.resource = Some(vec![topic_ref.get_ref()]);
            }
            TopicPolicyBuilder::new(Id::generate_id(&self.id, TOPIC_POLICY_ID_SUFFIX), policy, vec![&topic_ref]).build(stack_builder);
        }

        stack_builder.add_resource(Topic {
            id: self.id,
            resource_id: topic_resource_id,
            r#type: "AWS::SNS::Topic".to_string(),
            properties,
        });
        
        topic_ref
    }
}

impl TopicBuilder<FifoState> {
    pub fn fifo_throughput_scope(self, scope: FifoThroughputScope) -> TopicBuilder<FifoState> {
        Self {
            fifo_throughput_scope: Some(scope),
            ..self
        }
    }

    pub fn content_based_deduplication(self, content_based_deduplication: bool) -> TopicBuilder<FifoState> {
        Self {
            content_based_deduplication: Some(content_based_deduplication),
            ..self
        }
    }

    pub fn add_subscription(mut self, subscription: SubscriptionType) -> TopicBuilder<FifoStateWithSubscriptions> {
        self.add_subscription_internal(subscription);

        TopicBuilder {
            state: Default::default(),
            id: self.id,
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            topic_policy_doc: self.topic_policy_doc,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    /// Builds the FIFO topic and adds it to the stack.
    ///
    /// Automatically appends the required ".fifo" suffix to the topic name if not already present.
    pub fn build(mut self, stack_builder: &mut StackBuilder) -> TopicRef {
        if let Some(ref name) = self.topic_name
            && !name.ends_with(FIFO_SUFFIX) {
                self.topic_name = Some(format!("{}{}", name, FIFO_SUFFIX));
            }
        self.build_internal(true, stack_builder)
    }
}

impl TopicBuilder<FifoStateWithSubscriptions> {
    pub fn fifo_throughput_scope(self, scope: FifoThroughputScope) -> TopicBuilder<FifoStateWithSubscriptions> {
        Self {
            fifo_throughput_scope: Some(scope),
            ..self
        }
    }

    pub fn content_based_deduplication(self, content_based_deduplication: bool) -> TopicBuilder<FifoStateWithSubscriptions> {
        Self {
            content_based_deduplication: Some(content_based_deduplication),
            ..self
        }
    }

    pub fn add_subscription(mut self, subscription: SubscriptionType) -> TopicBuilder<FifoStateWithSubscriptions> {
        self.add_subscription_internal(subscription);

        TopicBuilder {
            state: Default::default(),
            id: self.id,
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            topic_policy_doc: self.topic_policy_doc,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }
    
    /// Builds the FIFO topic with subscriptions and adds it to the stack.
    ///
    /// Automatically appends the required ".fifo" suffix to the topic name if not already present.
    /// Creates Lambda permissions for all subscriptions.
    pub fn build(mut self, stack_builder: &mut StackBuilder) -> TopicRef {
        if let Some(ref name) = self.topic_name
            && !name.ends_with(FIFO_SUFFIX) {
                self.topic_name = Some(format!("{}{}", name, FIFO_SUFFIX));
            }
        self.build_internal(true, stack_builder)
    }
}

pub(crate) struct TopicPolicyBuilder {
    id: Id,
    doc: PolicyDocument,
    topics: Vec<Value>
}

impl TopicPolicyBuilder {
    /// Use the `topic_policy` method of `TopicBuilder` to add a topic policy to a topic
    pub(crate) fn new(id: Id, doc: PolicyDocument, topics: Vec<&TopicRef>) -> Self {
        Self::new_with_values(id, doc, topics.into_iter().map(|v| v.get_ref()).collect())
    }
    
    pub(crate) fn new_with_values(id: Id, doc: PolicyDocument, topics: Vec<Value>) -> Self {
        Self {
            id,
            doc,
            topics,
        }
    }
    
    pub(crate) fn build(self, stack_builder: &mut StackBuilder) -> TopicPolicyRef {
        let resource_id = Resource::generate_id("TopicPolicy");
        stack_builder.add_resource(TopicPolicy {
            id: self.id.clone(),
            resource_id: resource_id.clone(),
            r#type: "AWS::SNS::TopicPolicy".to_string(),
            properties: TopicPolicyProperties {
                doc: self.doc,
                topics: self.topics,
            },
        });
        
        TopicPolicyRef::new(self.id, resource_id)
    }
}


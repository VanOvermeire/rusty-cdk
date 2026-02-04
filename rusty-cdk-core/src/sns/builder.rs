use crate::iam::{PolicyDocument, RoleRef};
use crate::intrinsic::{get_arn, get_ref};
use crate::kms::KeyRef;
use crate::lambda::{FunctionRef, PermissionBuilder};
use crate::shared::{Id, TOPIC_POLICY_ID_SUFFIX};
use crate::sns::{
    LoggingConfig, SnsSubscriptionProperties, Subscription, SubscriptionDtoType, Topic, TopicPolicy, TopicPolicyProperties, TopicPolicyRef,
    TopicPolicyType, TopicProperties, TopicRef, TopicType,
};
use crate::stack::{Resource, StackBuilder};
use crate::type_state;
use crate::wrappers::{
    ArchivePolicy, LambdaPermissionAction, StringWithOnlyAlphaNumericsUnderscoresAndHyphens, SuccessFeedbackSampleRate, TopicDisplayName,
};
use serde_json::{Value, json};
use std::marker::PhantomData;

const FIFO_SUFFIX: &str = ".fifo";

#[derive(Debug, Clone)]
pub enum FifoThroughputScope {
    Topic,
    MessageGroup,
}

pub enum SubscriptionType<'a> {
    Lambda(&'a FunctionRef),
}

impl From<FifoThroughputScope> for String {
    fn from(value: FifoThroughputScope) -> Self {
        match value {
            FifoThroughputScope::Topic => "Topic".to_string(),
            FifoThroughputScope::MessageGroup => "MessageGroup".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TracingConfig {
    PassThrough,
    Active,
}

impl From<TracingConfig> for String {
    fn from(value: TracingConfig) -> Self {
        match value {
            TracingConfig::PassThrough => "PassThrough".to_string(),
            TracingConfig::Active => "Active".to_string(),
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
    archive_policy: Option<String>,
    display_name: Option<String>,
    kms_master_key_id: Option<Value>,
    tracing_config: Option<String>,
    logging_config: Option<LoggingConfig>,
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
            archive_policy: None,
            display_name: None,
            kms_master_key_id: None,
            tracing_config: None,
            logging_config: None,
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
            archive_policy: self.archive_policy,
            display_name: self.display_name,
            kms_master_key_id: self.kms_master_key_id,
            tracing_config: self.tracing_config,
            logging_config: self.logging_config,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> TopicRef {
        self.build_internal(false, stack_builder)
    }
}

impl TopicBuilder<StandardStateWithSubscriptions> {
    pub fn add_subscription(mut self, subscription: SubscriptionType) -> Self {
        self.add_subscription_internal(subscription);
        self
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> TopicRef {
        self.build_internal(false, stack_builder)
    }
}

impl<T: TopicBuilderState> TopicBuilder<T> {
    pub fn display_name(self, display_name: TopicDisplayName) -> Self {
        Self {
            display_name: Some(display_name.0),
            ..self
        }
    }

    pub fn logging_config(self, logging_config: LoggingConfig) -> Self {
        Self {
            logging_config: Some(logging_config),
            ..self
        }
    }

    pub fn kms_master_key(self, kms_key: &KeyRef) -> Self {
        Self {
            kms_master_key_id: Some(kms_key.get_ref()),
            ..self
        }
    }

    pub fn tracing_config(self, tracing_config: TracingConfig) -> Self {
        Self {
            tracing_config: Some(tracing_config.into()),
            ..self
        }
    }

    pub fn topic_name(self, topic_name: StringWithOnlyAlphaNumericsUnderscoresAndHyphens) -> Self {
        Self {
            topic_name: Some(topic_name.0),
            ..self
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
            display_name: self.display_name,
            kms_master_key_id: self.kms_master_key_id,
            archive_policy: self.archive_policy,
            tracing_config: self.tracing_config,
            logging_config: self.logging_config,
        }
    }

    /// Adds an SNS Topic Policy for this topic.
    /// The code will automatically set the `resources` section of the `PolicyDocument` to the ARN of this queue, so there's no need to pass that in.
    pub fn topic_policy(self, doc: PolicyDocument) -> Self {
        Self {
            topic_policy_doc: Some(doc),
            ..self
        }
    }

    fn add_subscription_internal(&mut self, subscription: SubscriptionType) {
        match subscription {
            SubscriptionType::Lambda(l) => self
                .lambda_subscription_ids
                .push((l.get_id().clone(), l.get_resource_id().to_string())),
        };
    }

    fn build_internal(self, fifo: bool, stack_builder: &mut StackBuilder) -> TopicRef {
        let topic_resource_id = Resource::generate_id("SnsTopic");

        self.lambda_subscription_ids
            .iter()
            .for_each(|(to_subscribe_id, to_subscribe_resource_id)| {
                let subscription_id = Id::combine_ids(&self.id, to_subscribe_id);
                let subscription_resource_id = Resource::generate_id("SnsSubscription");

                PermissionBuilder::new(
                    &Id::generate_id(&subscription_id, "Permission"),
                    LambdaPermissionAction("lambda:InvokeFunction".to_string()),
                    get_arn(to_subscribe_resource_id),
                    "sns.amazonaws.com",
                )
                .source_arn(get_ref(&topic_resource_id))
                .build(stack_builder);

                let subscription = Subscription {
                    id: subscription_id,
                    resource_id: subscription_resource_id,
                    r#type: SubscriptionDtoType::SubscriptionType,
                    properties: SnsSubscriptionProperties {
                        protocol: "lambda".to_string(),
                        endpoint: get_arn(to_subscribe_resource_id),
                        topic_arn: get_ref(&topic_resource_id),
                    },
                };

                stack_builder.add_resource(subscription);
            });

        let archive_policy = if let Some(policy_retention_time) = self.archive_policy {
            Some(json!({ "MessageRetentionPeriod": policy_retention_time }))
        } else {
            None
        };

        let properties = TopicProperties {
            topic_name: self.topic_name,
            fifo_topic: Some(fifo),
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope.map(Into::into),
            archive_policy,
            display_name: self.display_name,
            kms_master_key_id: self.kms_master_key_id,
            tracing_config: self.tracing_config,
            delivery_status_logging: self.logging_config,
        };

        let topic_ref = TopicRef::internal_new(self.id.clone(), topic_resource_id.to_string());

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
            r#type: TopicType::TopicType,
            properties,
        });

        topic_ref
    }
}

impl TopicBuilder<FifoState> {
    pub fn archive_policy(self, archive_policy: ArchivePolicy) -> Self {
        Self {
            archive_policy: Some(archive_policy.0.to_string()),
            ..self
        }
    }

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
            id: self.id,
            state: Default::default(),
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            topic_policy_doc: self.topic_policy_doc,
            lambda_subscription_ids: self.lambda_subscription_ids,
            display_name: self.display_name,
            kms_master_key_id: self.kms_master_key_id,
            archive_policy: self.archive_policy,
            tracing_config: self.tracing_config,
            logging_config: self.logging_config,
        }
    }

    /// Builds the FIFO topic and adds it to the stack.
    ///
    /// Automatically appends the required ".fifo" suffix to the topic name if not already present.
    pub fn build(mut self, stack_builder: &mut StackBuilder) -> TopicRef {
        if let Some(ref name) = self.topic_name
            && !name.ends_with(FIFO_SUFFIX)
        {
            self.topic_name = Some(format!("{}{}", name, FIFO_SUFFIX));
        }
        self.build_internal(true, stack_builder)
    }
}

impl TopicBuilder<FifoStateWithSubscriptions> {
    pub fn fifo_throughput_scope(self, scope: FifoThroughputScope) -> Self {
        Self {
            fifo_throughput_scope: Some(scope),
            ..self
        }
    }

    pub fn content_based_deduplication(self, content_based_deduplication: bool) -> Self {
        Self {
            content_based_deduplication: Some(content_based_deduplication),
            ..self
        }
    }

    pub fn add_subscription(mut self, subscription: SubscriptionType) -> TopicBuilder<FifoStateWithSubscriptions> {
        self.add_subscription_internal(subscription);
        self
    }

    /// Builds the FIFO topic with subscriptions and adds it to the stack.
    ///
    /// Automatically appends the required ".fifo" suffix to the topic name if not already present.
    /// Creates Lambda permissions for all subscriptions.
    pub fn build(mut self, stack_builder: &mut StackBuilder) -> TopicRef {
        if let Some(ref name) = self.topic_name
            && !name.ends_with(FIFO_SUFFIX)
        {
            self.topic_name = Some(format!("{}{}", name, FIFO_SUFFIX));
        }
        self.build_internal(true, stack_builder)
    }
}

pub enum Protocol {
    HTTP,
    SQS,
    Lambda,
    Firehose,
    Application,
}

impl From<Protocol> for String {
    fn from(value: Protocol) -> String {
        match value {
            Protocol::HTTP => "http".to_string(),
            Protocol::SQS => "sqs".to_string(),
            Protocol::Lambda => "lambda".to_string(),
            Protocol::Firehose => "firehose".to_string(),
            Protocol::Application => "application".to_string(),
        }
    }
}

pub struct LoggingConfigBuilder {
    protocol: String,
    success_feedback_sample_rate: Option<u8>,
    failure_feedback_role: Option<Value>,
    success_feedback_role: Option<Value>,
}

impl LoggingConfigBuilder {
    pub fn new(protocol: Protocol) -> Self {
        Self {
            protocol: protocol.into(),
            success_feedback_sample_rate: None,
            failure_feedback_role: None,
            success_feedback_role: None,
        }
    }

    pub fn success_feedback_role(self, role: &RoleRef) -> Self {
        Self {
            success_feedback_role: Some(role.get_arn()),
            ..self
        }
    }

    pub fn failure_feedback_role(self, role: &RoleRef) -> Self {
        Self {
            failure_feedback_role: Some(role.get_arn()),
            ..self
        }
    }

    pub fn success_feedback_sample_rate(self, success_feedback_sample_rate: SuccessFeedbackSampleRate) -> Self {
        Self {
            success_feedback_sample_rate: Some(success_feedback_sample_rate.0),
            ..self
        }
    }

    pub fn build(self) -> LoggingConfig {
        LoggingConfig {
            failure_feedback_role_arn: self.failure_feedback_role,
            protocol: self.protocol,
            success_feedback_role_arn: self.success_feedback_role,
            success_feedback_sample_rate: self.success_feedback_sample_rate,
        }
    }
}

pub(crate) struct TopicPolicyBuilder {
    id: Id,
    doc: PolicyDocument,
    topics: Vec<Value>,
}

impl TopicPolicyBuilder {
    /// Use the `topic_policy` method of `TopicBuilder` to add a topic policy to a topic
    pub(crate) fn new(id: Id, doc: PolicyDocument, topics: Vec<&TopicRef>) -> Self {
        Self::new_with_values(id, doc, topics.into_iter().map(|v| v.get_ref()).collect())
    }

    pub(crate) fn new_with_values(id: Id, doc: PolicyDocument, topics: Vec<Value>) -> Self {
        Self { id, doc, topics }
    }

    pub(crate) fn build(self, stack_builder: &mut StackBuilder) -> TopicPolicyRef {
        let resource_id = Resource::generate_id("TopicPolicy");
        stack_builder.add_resource(TopicPolicy {
            id: self.id.clone(),
            resource_id: resource_id.clone(),
            r#type: TopicPolicyType::TopicPolicyType,
            properties: TopicPolicyProperties {
                doc: self.doc,
                topics: self.topics,
            },
        });

        TopicPolicyRef::internal_new(self.id, resource_id)
    }
}

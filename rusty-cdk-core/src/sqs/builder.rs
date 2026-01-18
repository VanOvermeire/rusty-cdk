use crate::shared::Id;
use crate::sqs::{Queue, QueuePolicy, QueuePolicyProperties, QueuePolicyRef, QueueProperties, RedrivePolicy};
use crate::sqs::QueueRef;
use crate::stack::{Resource, StackBuilder};
use crate::wrappers::{
    DelaySeconds, MaximumMessageSize, MessageRetentionPeriod, NonZeroNumber, ReceiveMessageWaitTime,
    StringWithOnlyAlphaNumericsAndUnderscores, VisibilityTimeout,
};
use serde_json::Value;
use std::marker::PhantomData;
use crate::iam::PolicyDocument;
use crate::shared::QUEUE_POLICY_ID_SUFFIX;
use crate::type_state;

const FIFO_SUFFIX: &str = ".fifo";

#[derive(Debug, Clone)]
pub enum DeduplicationScope {
    Queue,
    MessageGroup,
}

impl From<DeduplicationScope> for String {
    fn from(value: DeduplicationScope) -> Self {
        match value {
            DeduplicationScope::Queue => "queue".to_string(),
            DeduplicationScope::MessageGroup => "messageGroup".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FifoThroughputLimit {
    PerQueue,
    PerMessageGroupId,
}

impl From<FifoThroughputLimit> for String {
    fn from(value: FifoThroughputLimit) -> Self {
        match value {
            FifoThroughputLimit::PerQueue => "perQueue".to_string(),
            FifoThroughputLimit::PerMessageGroupId => "perMessageGroupId".to_string(),
        }
    }
}

type_state!(
    QueueBuilderState,
    StartState,
    StandardState,
    FifoState,
);

/// Builder for SQS queues.
///
/// Supports both standard and FIFO queues. FIFO queues have additional configuration
/// options for deduplication and throughput.
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::sqs::QueueBuilder;
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::{delay_seconds, message_retention_period, visibility_timeout};
///
/// let mut stack_builder = StackBuilder::new();
///
/// // Create a standard queue
/// let standard_queue = QueueBuilder::new("standard-queue")
///     .standard_queue()
///     .visibility_timeout(visibility_timeout!(60))
///     .build(&mut stack_builder);
/// 
/// // Create a FIFO queue
/// let queue = QueueBuilder::new("my-queue")
///     .fifo_queue()
///     .content_based_deduplication(true)
///     .delay_seconds(delay_seconds!(30))
///     .message_retention_period(message_retention_period!(600))
///     .build(&mut stack_builder);
/// ```
pub struct QueueBuilder<T: QueueBuilderState> {
    state: PhantomData<T>,
    id: Id,
    queue_name: Option<String>,
    delay_seconds: Option<u32>,
    maximum_message_size: Option<u32>,
    message_retention_period: Option<u32>,
    receive_message_wait_time_seconds: Option<u32>,
    visibility_timeout: Option<u32>,
    content_based_deduplication: Option<bool>,
    deduplication_scope: Option<String>,
    fifo_throughput_limit: Option<String>,
    sqs_managed_sse_enabled: Option<bool>,
    redrive_policy: Option<RedrivePolicy>,
    redrive_allow_policy: Option<Value>,
    queue_policy_doc: Option<PolicyDocument>,
}

impl QueueBuilder<StartState> {
    /// Creates a new SQS queue builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the queue
    pub fn new(id: &str) -> Self {
        Self {
            state: Default::default(),
            id: Id(id.to_string()),
            queue_name: None,
            delay_seconds: None,
            maximum_message_size: None,
            message_retention_period: None,
            receive_message_wait_time_seconds: None,
            visibility_timeout: None,
            content_based_deduplication: None,
            deduplication_scope: None,
            fifo_throughput_limit: None,
            sqs_managed_sse_enabled: None,
            redrive_policy: None,
            redrive_allow_policy: None,
            queue_policy_doc: None,
        }
    }

    pub fn standard_queue(self) -> QueueBuilder<StandardState> {
        QueueBuilder {
            state: Default::default(),
            id: self.id,
            queue_name: self.queue_name,
            delay_seconds: self.delay_seconds,
            maximum_message_size: self.maximum_message_size,
            message_retention_period: self.message_retention_period,
            receive_message_wait_time_seconds: self.receive_message_wait_time_seconds,
            visibility_timeout: self.visibility_timeout,
            content_based_deduplication: self.content_based_deduplication,
            deduplication_scope: self.deduplication_scope,
            fifo_throughput_limit: self.fifo_throughput_limit,
            sqs_managed_sse_enabled: self.sqs_managed_sse_enabled,
            redrive_policy: self.redrive_policy,
            redrive_allow_policy: self.redrive_allow_policy,
            queue_policy_doc: self.queue_policy_doc,
        }
    }

    pub fn fifo_queue(self) -> QueueBuilder<FifoState> {
        QueueBuilder {
            state: Default::default(),
            id: self.id,
            queue_name: self.queue_name,
            delay_seconds: self.delay_seconds,
            maximum_message_size: self.maximum_message_size,
            message_retention_period: self.message_retention_period,
            receive_message_wait_time_seconds: self.receive_message_wait_time_seconds,
            visibility_timeout: self.visibility_timeout,
            content_based_deduplication: self.content_based_deduplication,
            deduplication_scope: self.deduplication_scope,
            fifo_throughput_limit: self.fifo_throughput_limit,
            sqs_managed_sse_enabled: self.sqs_managed_sse_enabled,
            redrive_policy: self.redrive_policy,
            redrive_allow_policy: self.redrive_allow_policy,
            queue_policy_doc: self.queue_policy_doc,
        }
    }
}

impl<T: QueueBuilderState> QueueBuilder<T> {
    pub fn delay_seconds(self, delay: DelaySeconds) -> Self {
        Self {
            delay_seconds: Some(delay.0 as u32),
            ..self
        }
    }

    pub fn maximum_message_size(self, size: MaximumMessageSize) -> Self {
        Self {
            maximum_message_size: Some(size.0),
            ..self
        }
    }

    pub fn message_retention_period(self, period: MessageRetentionPeriod) -> Self {
        Self {
            message_retention_period: Some(period.0),
            ..self
        }
    }

    pub fn receive_message_wait_time_seconds(self, wait_time: ReceiveMessageWaitTime) -> Self {
        Self {
            receive_message_wait_time_seconds: Some(wait_time.0 as u32),
            ..self
        }
    }

    pub fn visibility_timeout(self, timeout: VisibilityTimeout) -> Self {
        Self {
            visibility_timeout: Some(timeout.0),
            ..self
        }
    }

    pub fn sqs_managed_sse_enabled(self, enabled: bool) -> Self {
        Self {
            sqs_managed_sse_enabled: Some(enabled),
            ..self
        }
    }

    pub fn dead_letter_queue<D: Into<String>>(self, dead_letter_target_arn: D, max_receive_count: NonZeroNumber) -> Self {
        Self {
            redrive_policy: Some(RedrivePolicy {
                dead_letter_target_arn: dead_letter_target_arn.into(),
                max_receive_count: max_receive_count.0,
            }),
            ..self
        }
    }

    pub fn redrive_allow_policy(self, policy: Value) -> Self {
        Self {
            redrive_allow_policy: Some(policy),
            ..self
        }
    }

    pub fn queue_name(self, name: StringWithOnlyAlphaNumericsAndUnderscores) -> Self {
        Self {
            queue_name: Some(name.0),
            ..self
        }
    }
    
    /// Adds an SQS Queue Policy for this queue.
    /// The code will automatically set the `resources` section of the `PolicyDocument` to the ARN of this queue, so there's no need to pass that in.
    pub fn queue_policy(self, doc: PolicyDocument) -> Self {
        Self {
            queue_policy_doc: Some(doc),
            ..self
        }
    }

    fn build_internal(self, fifo: bool, stack_builder: &mut StackBuilder) -> QueueRef {
        let properties = QueueProperties {
            queue_name: self.queue_name,
            delay_seconds: self.delay_seconds,
            maximum_message_size: self.maximum_message_size,
            message_retention_period: self.message_retention_period,
            receive_message_wait_time_seconds: self.receive_message_wait_time_seconds,
            visibility_timeout: self.visibility_timeout,
            fifo_queue: if fifo { Some(true) } else { None },
            content_based_deduplication: self.content_based_deduplication,
            deduplication_scope: self.deduplication_scope,
            fifo_throughput_limit: self.fifo_throughput_limit,
            sqs_managed_sse_enabled: self.sqs_managed_sse_enabled,
            redrive_policy: self.redrive_policy,
            redrive_allow_policy: self.redrive_allow_policy,
        };

        let resource_id = Resource::generate_id("SqsQueue");
        let queue_ref = QueueRef::internal_new(self.id.clone(), resource_id.clone());
        
        if let Some(mut policy) = self.queue_policy_doc {
            for statement in &mut policy.statements {
                // point the statements of this policy to the queue
                statement.resource = Some(vec![queue_ref.get_arn()]);
            }
            QueuePolicyBuilder::new(Id::generate_id(&self.id, QUEUE_POLICY_ID_SUFFIX), policy, vec![&queue_ref]).build(stack_builder);
        }

        stack_builder.add_resource(Queue {
            id: self.id,
            resource_id,
            r#type: "AWS::SQS::Queue".to_string(),
            properties,
        });

        queue_ref
    }
}

impl QueueBuilder<StandardState> {
    pub fn build(self, stack_builder: &mut StackBuilder) -> QueueRef {
        self.build_internal(false, stack_builder)
    }
}

impl QueueBuilder<FifoState> {
    pub fn content_based_deduplication(self, enabled: bool) -> Self {
        Self {
            content_based_deduplication: Some(enabled),
            ..self
        }
    }

    /// Enables high throughput mode for FIFO queues.
    ///
    /// Sets deduplication scope to MessageGroup and throughput limit to PerMessageGroupId.
    pub fn high_throughput_fifo(self) -> Self {
        Self {
            deduplication_scope: Some(DeduplicationScope::MessageGroup.into()),
            fifo_throughput_limit: Some(FifoThroughputLimit::PerMessageGroupId.into()),
            ..self
        }
    }

    pub fn deduplication_scope(self, scope: DeduplicationScope) -> Self {
        Self {
            deduplication_scope: Some(scope.into()),
            ..self
        }
    }

    pub fn fifo_throughput_limit(self, limit: FifoThroughputLimit) -> Self {
        Self {
            fifo_throughput_limit: Some(limit.into()),
            ..self
        }
    }

    /// Builds the FIFO queue and adds it to the stack.
    ///
    /// Automatically appends the required ".fifo" suffix to the queue name if not already present.
    pub fn build(mut self, stack_builder: &mut StackBuilder) -> QueueRef {
        if let Some(ref name) = self.queue_name
            && !name.ends_with(FIFO_SUFFIX) {
                self.queue_name = Some(format!("{}{}", name, FIFO_SUFFIX));
            }
        self.build_internal(true, stack_builder)
    }
}

pub(crate) struct QueuePolicyBuilder {
    id: Id,
    doc: PolicyDocument,
    queues: Vec<Value>
}

impl QueuePolicyBuilder {
    /// Use the `queue_policy` method of `QueueBuilder` to add a queue policy to a queue
    pub(crate) fn new(id: Id, doc: PolicyDocument, queues: Vec<&QueueRef>) -> Self {
        Self::new_with_values(id, doc, queues.into_iter().map(|v| v.get_ref()).collect())
    }

    pub(crate) fn new_with_values(id: Id, doc: PolicyDocument, queues: Vec<Value>) -> Self {
        Self {
            id,
            doc,
            queues,
        }
    }

    pub(crate) fn build(self, stack_builder: &mut StackBuilder) -> QueuePolicyRef {
        let resource_id = Resource::generate_id("QueuePolicy");
        stack_builder.add_resource(QueuePolicy {
            id: self.id.clone(),
            resource_id: resource_id.clone(),
            r#type: "AWS::SQS::QueuePolicy".to_string(),
            properties: QueuePolicyProperties {
                doc: self.doc,
                queues: self.queues,
            },
        });

        QueuePolicyRef::internal_new(self.id, resource_id)
    }
}

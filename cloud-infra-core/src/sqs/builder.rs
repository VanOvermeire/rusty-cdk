use crate::sqs::dto::{RedrivePolicy, SqsQueue, SqsQueueProperties};
use crate::stack::Resource;
use crate::wrappers::{
    DelaySeconds, MaximumMessageSize, MessageRetentionPeriod, NonZeroNumber,
    ReceiveMessageWaitTime, StringWithOnlyAlphaNumericsAndUnderscores, VisibilityTimeout,
};
use serde_json::Value;
use std::marker::PhantomData;

const FIFO_SUFFIX: &'static str = ".fifo";

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

pub trait SqsQueueBuilderState {}

pub struct StartState {}
impl SqsQueueBuilderState for StartState {}

pub struct StandardState {}
impl SqsQueueBuilderState for StandardState {}

pub struct FifoState {}
impl SqsQueueBuilderState for FifoState {}

pub struct SqsQueueBuilder<T: SqsQueueBuilderState> {
    state: PhantomData<T>,
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
}

impl SqsQueueBuilder<StartState> {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
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
        }
    }

    pub fn standard_queue(self) -> SqsQueueBuilder<StandardState> {
        SqsQueueBuilder {
            state: Default::default(),
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
        }
    }

    pub fn fifo_queue(self) -> SqsQueueBuilder<FifoState> {
        SqsQueueBuilder {
            state: Default::default(),
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
        }
    }
}

impl<T: SqsQueueBuilderState> SqsQueueBuilder<T> {
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

    pub fn dead_letter_queue(self, dead_letter_target_arn: String, max_receive_count: NonZeroNumber) -> Self {
        Self {
            redrive_policy: Some(RedrivePolicy {
                dead_letter_target_arn,
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

    fn build_internal(self, fifo: bool) -> SqsQueue {
        let properties = SqsQueueProperties {
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

        SqsQueue {
            resource_id: Resource::generate_id("SqsQueue"),
            r#type: "AWS::SQS::Queue".to_string(),
            properties,
        }
    }
}

impl SqsQueueBuilder<StandardState> {
    #[must_use]
    pub fn build(self) -> SqsQueue {
        self.build_internal(false)
    }
}

impl SqsQueueBuilder<FifoState> {
    pub fn content_based_deduplication(self, enabled: bool) -> Self {
        Self {
            content_based_deduplication: Some(enabled),
            ..self
        }
    }

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

    #[must_use]
    pub fn build(mut self) -> SqsQueue {
        if let Some(ref name) = self.queue_name {
            if !name.ends_with(FIFO_SUFFIX) {
                self.queue_name = Some(format!("{}{}", name, FIFO_SUFFIX));
            }
        }
        self.build_internal(true)
    }
}
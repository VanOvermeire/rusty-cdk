use std::marker::PhantomData;
use crate::intrinsic_functions::{get_arn, get_ref};
use crate::lambda::{LambdaFunction, LambdaPermission, LambdaPermissionProperties};
use crate::sns::dto::{SnsSubscription, SnsSubscriptionProperties, SnsTopic, SnsTopicProperties};
use crate::stack::Resource;

pub enum FifoThroughputScope {
    Topic,
    MessageGroup
}

pub enum Subscription<'a> {
    Lambda(&'a LambdaFunction)
}

impl From<FifoThroughputScope> for String {
    fn from(value: FifoThroughputScope) -> Self {
        match value {
            FifoThroughputScope::Topic => "Topic".to_string(),
            FifoThroughputScope::MessageGroup => "MessageGroup".to_string(),
        }
    }
}

pub trait SnsTopicBuilderState {}

pub struct StartState {}
impl SnsTopicBuilderState for StartState {}

pub struct StandardState {}
impl SnsTopicBuilderState for StandardState {}

pub struct FifoState {}
impl SnsTopicBuilderState for FifoState {}

// TODO maybe another state for subscriptions, that way you don't get an empty vec when there are no subscriptions
pub struct SnsTopicBuilder<T: SnsTopicBuilderState> {
    state: PhantomData<T>,
    topic_name: Option<String>,
    content_based_deduplication: Option<bool>,
    fifo_throughput_scope: Option<FifoThroughputScope>,
    lambda_subscription_ids: Vec<String>,
}

impl SnsTopicBuilder<StartState> {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
            topic_name: None,
            content_based_deduplication: None,
            fifo_throughput_scope: None,
            lambda_subscription_ids: vec![],
        }
    }

    #[must_use]
    pub fn build(self) -> (SnsTopic, Vec<(SnsSubscription, LambdaPermission)>) {
        self.build_internal(false)
    }
}

impl<T: SnsTopicBuilderState> SnsTopicBuilder<T> {
    // TODO wrapper
    pub fn topic_name(self, topic_name: String) -> SnsTopicBuilder<T> {
        SnsTopicBuilder {
            topic_name: Some(topic_name),
            state: Default::default(),
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    pub fn fifo(self) -> SnsTopicBuilder<FifoState> {
        SnsTopicBuilder {
            state: Default::default(),
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    // TODO add email and SNS subscriptions
    pub fn add_subscription(mut self, subscription: Subscription) -> SnsTopicBuilder<T> {
        match subscription {
            Subscription::Lambda(l) => self.lambda_subscription_ids.push(l.get_id().to_string())
        };
        
        SnsTopicBuilder {
            state: Default::default(),
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }
    
    fn build_internal(self, fifo: bool) -> (SnsTopic, Vec<(SnsSubscription, LambdaPermission)>) {
        let topic_id = Resource::generate_id("SnsTopic");
        
        let subscriptions: Vec<_> = self.lambda_subscription_ids.iter().map(|to_subscribe_id| {
            let subscription_id = Resource::generate_id("SnsSubscription");
            let properties = SnsSubscriptionProperties {
                protocol: "lambda".to_string(),
                endpoint: get_arn(to_subscribe_id),
                topic_arn: get_ref(&topic_id),    
            };
            let subscription = SnsSubscription {
                id: subscription_id,
                referenced_ids: vec![to_subscribe_id.to_string(), topic_id.to_string()],
                r#type: "AWS::SNS::Subscription".to_string(),
                properties,
            };
            
            let permission_id = Resource::generate_id("LambdaPermission");
            let properties = LambdaPermissionProperties {
                action: "lambda:InvokeFunction".to_string(),
                function_name: get_arn(to_subscribe_id),
                principal: "sns.amazonaws.com".to_string(),
                source_arn: Some(get_ref(&topic_id)),
            };
            let permission = LambdaPermission {
                id: permission_id,
                referenced_ids: vec![to_subscribe_id.to_string(), topic_id.to_string()],
                r#type: "AWS::Lambda::Permission".to_string(),
                properties,
            };

            (subscription, permission)
        }).collect();
        
        let properties = SnsTopicProperties {
            topic_name: self.topic_name,
            fifo_topic: Some(fifo),
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope.map(Into::into),
        };
        
        let topic = SnsTopic {
            id: topic_id,
            r#type: "AWS::SNS::Topic".to_string(),
            properties,
        };

        (topic, subscriptions)
    }
}

impl SnsTopicBuilder<FifoState> {
    pub fn fifo_throughput_scope(self, scope: FifoThroughputScope) -> SnsTopicBuilder<FifoState> {
        Self {
            fifo_throughput_scope: Some(scope),
            ..self
        }
    }

    pub fn content_based_deduplication(self, content_based_deduplication: bool) -> SnsTopicBuilder<FifoState> {
        Self {
            content_based_deduplication: Some(content_based_deduplication),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> (SnsTopic, Vec<(SnsSubscription, LambdaPermission)>) {
        self.build_internal(true)
    }
}
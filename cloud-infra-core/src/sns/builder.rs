use std::marker::PhantomData;
use crate::intrinsic_functions::{get_arn, get_ref};
use crate::lambda::{Function, Permission, PermissionBuilder};
use crate::shared::Id;
use crate::sns::dto::{Subscription, SnsSubscriptionProperties, Topic, TopicProperties};
use crate::stack::Resource;
use crate::wrappers::StringWithOnlyAlphaNumericsUnderscoresAndHyphens;

const FIFO_SUFFIX: &str = ".fifo";

pub enum FifoThroughputScope {
    Topic,
    MessageGroup
}

pub enum SubscriptionType<'a> {
    Lambda(&'a Function)
}

impl From<FifoThroughputScope> for String {
    fn from(value: FifoThroughputScope) -> Self {
        match value {
            FifoThroughputScope::Topic => "Topic".to_string(),
            FifoThroughputScope::MessageGroup => "MessageGroup".to_string(),
        }
    }
}

pub trait TopicBuilderState {}
pub struct StartState {}
impl TopicBuilderState for StartState {}
pub struct StandardStateWithSubscriptions {}
impl TopicBuilderState for StandardStateWithSubscriptions {}
pub struct FifoState {}
impl TopicBuilderState for FifoState {}
pub struct FifoStateWithSubscriptions {}
impl TopicBuilderState for FifoStateWithSubscriptions {}

pub struct TopicBuilder<T: TopicBuilderState> {
    state: PhantomData<T>,
    id: Id,
    topic_name: Option<String>,
    content_based_deduplication: Option<bool>,
    fifo_throughput_scope: Option<FifoThroughputScope>,
    lambda_subscription_ids: Vec<(Id, String)>,
}

impl TopicBuilder<StartState> {
    pub fn new(id: &str) -> Self {
        Self {
            state: Default::default(),
            id: Id(id.to_string()),
            topic_name: None,
            content_based_deduplication: None,
            fifo_throughput_scope: None,
            lambda_subscription_ids: vec![],
        }
    }

    pub fn add_subscription(mut self, subscription: SubscriptionType) -> TopicBuilder<StandardStateWithSubscriptions> {
        self.add_subscription_internal(subscription);

        TopicBuilder {
            state: Default::default(),
            id: self.id,
            topic_name: self.topic_name,
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope,
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    #[must_use]
    pub fn build(self) -> Topic {
        let (topic, _) = self.build_internal(false);
        topic
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
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    #[must_use]
    pub fn build(self) -> (Topic, Vec<(Subscription, Permission)>) {
        self.build_internal(false)
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
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }
    
    fn add_subscription_internal(&mut self, subscription: SubscriptionType) {
        match subscription {
            SubscriptionType::Lambda(l) => self.lambda_subscription_ids.push((l.get_id().clone(), l.get_resource_id().to_string()))
        };
    }
    
    fn build_internal(self, fifo: bool) -> (Topic, Vec<(Subscription, Permission)>) {
        let topic_resource_id = Resource::generate_id("SnsTopic");
        
        let subscriptions: Vec<_> = self.lambda_subscription_ids.iter().map(|(to_subscribe_id, to_subscribe_resource_id)| {
            let subscription_id = Id::combine_ids(&self.id, to_subscribe_id);
            let subscription_resource_id = Resource::generate_id("SnsSubscription");
            
            let permission = PermissionBuilder::new(&Id::generate_id(&subscription_id, "Permission"), "lambda:InvokeFunction".to_string(), get_arn(to_subscribe_resource_id), "sns.amazonaws.com")
                .referenced_ids(vec![to_subscribe_resource_id.to_string(), topic_resource_id.to_string()])
                .source_arn(get_ref(&topic_resource_id))
                .build();

            let properties = SnsSubscriptionProperties {
                protocol: "lambda".to_string(),
                endpoint: get_arn(to_subscribe_resource_id),
                topic_arn: get_ref(&topic_resource_id),
            };
            let subscription = Subscription {
                id: subscription_id,
                resource_id: subscription_resource_id,
                referenced_ids: vec![to_subscribe_resource_id.to_string(), topic_resource_id.to_string()],
                r#type: "AWS::SNS::Subscription".to_string(),
                properties,
            };

            (subscription, permission)
        }).collect();
        
        let properties = TopicProperties {
            topic_name: self.topic_name,
            fifo_topic: Some(fifo),
            content_based_deduplication: self.content_based_deduplication,
            fifo_throughput_scope: self.fifo_throughput_scope.map(Into::into),
        };
        
        let topic = Topic {
            id: self.id,
            resource_id: topic_resource_id,
            r#type: "AWS::SNS::Topic".to_string(),
            properties,
        };

        (topic, subscriptions)
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
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    #[must_use]
    pub fn build(mut self) -> Topic {
        if let Some(ref name) = self.topic_name {
            if !name.ends_with(FIFO_SUFFIX) {
                self.topic_name = Some(format!("{}{}", name, FIFO_SUFFIX));
            }
        }
        let (topic, _) = self.build_internal(true);
        topic
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
            lambda_subscription_ids: self.lambda_subscription_ids,
        }
    }

    #[must_use]
    pub fn build(mut self) -> (Topic, Vec<(Subscription, Permission)>) {
        if let Some(ref name) = self.topic_name {
            if !name.ends_with(FIFO_SUFFIX) {
                self.topic_name = Some(format!("{}{}", name, FIFO_SUFFIX));
            }
        }
        self.build_internal(true)
    }
}
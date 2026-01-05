use std::marker::PhantomData;
use serde_json::Value;
use crate::events::{FlexibleTimeWindow, RetryPolicy, Schedule, ScheduleProperties, Target};
use crate::lambda::FunctionRef;
use crate::shared::Id;
use crate::sns::TopicRef;
use crate::sqs::QueueRef;
use crate::stack::Resource;
use crate::type_state;
use crate::wrappers::{MaxFlexibleTimeWindow, RetryPolicyEventAge, RetryPolicyRetries, ScheduleAtExpression, ScheduleCronExpression, ScheduleName, ScheduleRateExpression};

type_state!(
    ScheduleBuilderState,
    StartState,
    OneTimeScheduleState,
    RepeatedScheduleState,
);

#[derive(Debug)]
pub enum State {
    Enabled,
    Disabled,
}

impl From<State> for String {
    fn from(value: State) -> String {
        match value {
            State::Enabled => "ENABLED".to_string(),
            State::Disabled => "DISABLED".to_string(),
        }
    }
}

pub struct ScheduleBuilder<T: ScheduleBuilderState> {
    phantom_data: PhantomData<T>,
    id: Id,
    start_date: Option<String>,
    end_date: Option<String>,
    flexible_time_window: FlexibleTimeWindow,
    group_name: Option<String>,
    name: Option<String>,
    state: Option<String>,
    schedule_expression: Option<String>,
    target: Target
}

impl<T: ScheduleBuilderState> ScheduleBuilder<T> {
    pub fn new(id: &str, target: Target, flexible_time_window: FlexibleTimeWindow) -> ScheduleBuilder<StartState> {
        ScheduleBuilder {
            phantom_data: Default::default(),
            id: Id(id.to_string()),
            flexible_time_window,
            target,
            start_date: None,
            end_date: None,
            group_name: None,
            name: None,
            state: None,
            schedule_expression: None,
        }
    }
    
    pub fn name(self, name: ScheduleName) -> Self {
        Self {
            name: Some(name.0),
            ..self
        }
    }
    
    pub fn group_name(self, group_name: ScheduleName) -> Self {
        Self {
            group_name: Some(group_name.0),
            ..self
        }
    }
    
    pub fn state(self, state: State) -> Self {
        Self {
            state: Some(state.into()),
            ..self
        }
    }
    
    fn build_internal(self) -> Schedule {
        let resource_id = Resource::generate_id("Schedule");
        Schedule {
            id: self.id,
            resource_id: resource_id,
            r#type: "AWS::Scheduler::Schedule".to_string(),
            properties: ScheduleProperties {
                start_date: self.start_date,
                end_date: self.end_date,
                flexible_time_window: self.flexible_time_window,
                group_name: self.group_name,
                name: self.name,
                state: self.state,
                schedule_expression: self.schedule_expression.expect("schedule expression to be present, enforced by builder"),
                target: self.target,
            },
        }
    }
}

impl ScheduleBuilder<StartState> {
    pub fn one_time_schedule(self, expression: ScheduleAtExpression) -> ScheduleBuilder<OneTimeScheduleState> {
        ScheduleBuilder {
            phantom_data: Default::default(),
            schedule_expression: Some(expression.0),
            id: self.id,
            flexible_time_window: self.flexible_time_window,
            group_name: self.group_name,
            name: self.name,
            state: self.state,
            target: self.target,
            start_date: None,
            end_date: None,
        }
    }
    
    pub fn rate_schedule(self, expression: ScheduleRateExpression) -> ScheduleBuilder<RepeatedScheduleState> {
        let rate = format!("rate({} {})", expression.0, expression.1);
        ScheduleBuilder {
            phantom_data: Default::default(),
            schedule_expression: Some(rate),
            id: self.id,
            flexible_time_window: self.flexible_time_window,
            group_name: self.group_name,
            name: self.name,
            state: self.state,
            target: self.target,
            start_date: self.start_date,
            end_date: self.end_date,
        }
    }
    
    pub fn cron_schedule(self, expression: ScheduleCronExpression) -> ScheduleBuilder<RepeatedScheduleState> {
        ScheduleBuilder {
            phantom_data: Default::default(),
            schedule_expression: Some(expression.0),
            id: self.id,
            flexible_time_window: self.flexible_time_window,
            group_name: self.group_name,
            name: self.name,
            state: self.state,
            target: self.target,
            start_date: self.start_date,
            end_date: self.end_date,
        }
    }
}

impl ScheduleBuilder<OneTimeScheduleState> {
    pub fn build(self) -> Schedule {
        self.build_internal()
    }
}

impl ScheduleBuilder<RepeatedScheduleState> {
    // TODO better validation
    pub fn start_date(self, start_date: String) -> Self {
        Self {
            start_date: Some(start_date),
            ..self
        }
    }

    pub fn end_date(self, end_date: String) -> Self {
        Self {
            end_date: Some(end_date),
            ..self
        }
    }
    
    pub fn build(self) -> Schedule {
        self.build_internal()
    }
}

type_state!(
    TargetBuilderState,
    JsonTargetState,
    NormalTargetState,
);

pub struct TargetBuilder<T: TargetBuilderState> {
    phantom_data: PhantomData<T>,
    target_arn: Value,
    role_arn: String,
    input: Option<String>,
    retry_policy: Option<RetryPolicy>,
}

pub enum JsonTarget<'a> {
    Lambda(&'a FunctionRef)
    // AWS SF
    // EventBridge
}

pub enum NormalTarget<'a> {
    Sqs(&'a QueueRef),
    Sns(&'a TopicRef),
    Other(Value)
}

impl<T: TargetBuilderState> TargetBuilder<T> {
    /// Target that accepts any string input
    pub fn new_normal_target(target: NormalTarget, role_arn: String) -> TargetBuilder<NormalTargetState> {
        let arn = match target {
            NormalTarget::Sqs(r) => r.get_arn(),
            NormalTarget::Sns(r) => r.get_arn(),
            NormalTarget::Other(r) => r,
        };
        TargetBuilder {
            phantom_data: Default::default(),
            target_arn: arn,
            role_arn,
            input: None,
            retry_policy: None,
        }
    }
    
    /// Target that requires the input to be valid JSON
    pub fn new_json_target(target: JsonTarget, role_arn: String) -> TargetBuilder<JsonTargetState> {
        let arn = match target {
            JsonTarget::Lambda(l) => l.get_arn(),
        };
        TargetBuilder {
            phantom_data: Default::default(),
            target_arn: arn,
            role_arn,
            input: None,
            retry_policy: None,
        }
    }
    
    pub fn retry_policy(self, retry_policy: RetryPolicy) -> TargetBuilder<T> {
        TargetBuilder {
            retry_policy: Some(retry_policy),
            phantom_data: Default::default(),
            target_arn: self.target_arn,
            role_arn: self.role_arn,
            input: self.input,
        }
    }

    pub fn build(self) -> Target {
        Target {
            arn: self.target_arn,
            role_arn: self.role_arn,
            input: self.input,
            retry_policy: self.retry_policy,
        }
    }
}

impl TargetBuilder<NormalTargetState> {
    pub fn input(self, input: String) -> Self {
        Self {
            input: Some(input),
            ..self
        }
    }
}

impl TargetBuilder<JsonTargetState> {
    pub fn input(self, input: Value) -> Self {
        Self {
            input: Some(input.to_string()),
            ..self
        }
    }
}

pub enum Mode {
    Off,
    Flexible(MaxFlexibleTimeWindow)
}

pub struct FlexibleTimeWindowBuilder {
    maximum_window_in_minutes: Option<u16>,
    mode: String,
}

impl FlexibleTimeWindowBuilder {
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::Off => {
                Self {
                    maximum_window_in_minutes: None,
                    mode: "OFF".to_string(),
                }
            }
            Mode::Flexible(max) => {
                Self {
                    maximum_window_in_minutes: Some(max.0),
                    mode: "FLEXIBLE".to_string(),
                }
            }
        }
    }
    
    pub fn build(self) -> FlexibleTimeWindow {
        FlexibleTimeWindow {
            maximum_window_in_minutes: self.maximum_window_in_minutes,
            mode: self.mode,
        }
    }
}

pub struct RetryPolicyBuilder {
    maximum_event_age_in_seconds: Option<u32>, 
    maximum_retry_attempts: Option<u8>
}

impl RetryPolicyBuilder {
    pub fn new() -> Self {
        Self {
            maximum_event_age_in_seconds: None,
            maximum_retry_attempts: None,
        }    
    }
    
    pub fn maximum_event_age_in_seconds(self, maximum_event_age_in_seconds: RetryPolicyEventAge) -> Self {
        Self {
            maximum_event_age_in_seconds: Some(maximum_event_age_in_seconds.0),
            ..self
        }
    }
    
    pub fn maximum_retry_attempts(self, maximum_retry_attempts: RetryPolicyRetries) -> Self {
        Self {
            maximum_retry_attempts: Some(maximum_retry_attempts.0),
            ..self
        }
    }
    
    pub fn build(self) -> RetryPolicy {
        RetryPolicy {
            maximum_event_age_in_seconds: self.maximum_event_age_in_seconds,
            maximum_retry_attempts: self.maximum_retry_attempts,
        }
    }
}
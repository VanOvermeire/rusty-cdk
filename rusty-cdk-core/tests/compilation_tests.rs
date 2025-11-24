use rusty_cdk_core::dynamodb::AttributeType;
use rusty_cdk_core::dynamodb::Key;
use rusty_cdk_core::dynamodb::TableBuilder;
use rusty_cdk_core::iam::{
    AssumeRolePolicyDocumentBuilder, Effect, PrincipalBuilder, RoleBuilder, RolePropertiesBuilder, StatementBuilder,
};
use rusty_cdk_core::s3::BucketBuilder;
use rusty_cdk_core::sqs::QueueBuilder;
use rusty_cdk_core::stack::StackBuilder;
use rusty_cdk_core::wrappers::IamAction;
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsUnderscoresAndHyphens;
use rusty_cdk_core::wrappers::*;
use rusty_cdk_macros::{
    delay_seconds, iam_action, maximum_message_size, message_retention_period, non_zero_number, receive_message_wait_time,
    string_with_only_alphanumerics_and_underscores, visibility_timeout,
};
use rusty_cdk_core::cloudwatch::{LogGroupBuilder, LogGroupClass};

#[test]
fn dynamodb_pay_per_request_billing_should_compile() {
    let mut stack_builder = StackBuilder::new();
    let key = string_with_only_alphanumerics_and_underscores!("test");
    let _ = TableBuilder::new("myTable", Key::new(key, AttributeType::String))
        .pay_per_request_billing()
        .build(&mut stack_builder);
}

#[test]
fn dynamodb_provisioned_billing_should_compile() {
    let mut stack_builder = StackBuilder::new();
    let key = string_with_only_alphanumerics_and_underscores!("id");
    let read_cap = non_zero_number!(5);
    let write_cap = non_zero_number!(5);

    TableBuilder::new("myTable", Key::new(key, AttributeType::String))
        .provisioned_billing()
        .read_capacity(read_cap)
        .write_capacity(write_cap)
        .build(&mut stack_builder);
}

#[test]
fn sqs_standard_queue_builder_should_compile() {
    let mut stack_builder = StackBuilder::new();
    let queue_name = string_with_only_alphanumerics_and_underscores!("test_queue");
    let delay = delay_seconds!(300);
    let max_size = maximum_message_size!(262144);
    let retention = message_retention_period!(345600);
    let timeout = visibility_timeout!(30);
    let wait_time = receive_message_wait_time!(10);
    let max_receive = non_zero_number!(3);

    let _ = QueueBuilder::new("myQueue")
        .standard_queue()
        .queue_name(queue_name)
        .delay_seconds(delay)
        .maximum_message_size(max_size)
        .message_retention_period(retention)
        .visibility_timeout(timeout)
        .receive_message_wait_time_seconds(wait_time)
        .dead_letter_queue("arn:aws:sqs:us-east-1:123456789012:dlq", max_receive)
        .sqs_managed_sse_enabled(true)
        .build(&mut stack_builder);
}

#[test]
fn sqs_fifo_queue_builder_should_compile() {
    let mut stack_builder = StackBuilder::new();
    let queue_name = string_with_only_alphanumerics_and_underscores!("test_fifo_queue");
    let delay = delay_seconds!(60);
    let timeout = visibility_timeout!(120);

    let _ = QueueBuilder::new("myQueue")
        .fifo_queue()
        .queue_name(queue_name)
        .delay_seconds(delay)
        .visibility_timeout(timeout)
        .content_based_deduplication(true)
        .build(&mut stack_builder);
}

#[test]
fn stack_with_bucket_website_should_compile() {
    let mut stack_builder = StackBuilder::new();
    BucketBuilder::new("website").website("index.com").build(&mut stack_builder);

    let stack = StackBuilder::new().build();

    assert!(stack.is_ok());
}

#[test]
fn sns_standard_topic_builder_should_compile() {
    use rusty_cdk_core::sns::TopicBuilder;
    use rusty_cdk_macros::string_with_only_alphanumerics_underscores_and_hyphens;

    let mut stack_builder = StackBuilder::new();
    let topic_name = string_with_only_alphanumerics_underscores_and_hyphens!("test_topic");

    TopicBuilder::new("myTopic").topic_name(topic_name).build(&mut stack_builder);
}

#[test]
fn sns_fifo_topic_builder_should_compile() {
    use rusty_cdk_core::sns::{FifoThroughputScope, TopicBuilder};
    use rusty_cdk_macros::string_with_only_alphanumerics_underscores_and_hyphens;

    let mut stack_builder = StackBuilder::new();
    let topic_name = string_with_only_alphanumerics_underscores_and_hyphens!("test_fifo_topic");

    TopicBuilder::new("myTopic")
        .fifo()
        .topic_name(topic_name)
        .content_based_deduplication(true)
        .fifo_throughput_scope(FifoThroughputScope::MessageGroup)
        .build(&mut stack_builder);
}

#[test]
fn cloudwatch_log_group_builder_should_compile() {
    let mut stack_builder = StackBuilder::new();
    let log_group_name = LogGroupName("/aws/lambda/my-function".to_string());

    LogGroupBuilder::new("myLogGroup")
        .log_group_name_string(log_group_name)
        .log_group_class(LogGroupClass::Standard)
        .log_group_retention(RetentionInDays(7))
        .build(&mut stack_builder);
}

#[test]
fn iam_principal_builder_should_compile() {
    let mut stack_builder = StackBuilder::new();

    let statement = StatementBuilder::new(vec![iam_action!("s3:*")], Effect::Allow)
        .principal(PrincipalBuilder::new().service("lambda.amazonaws.com").build())
        .build();

    let assume_role_policy = AssumeRolePolicyDocumentBuilder::new(vec![statement]).build();
    let properties = RolePropertiesBuilder::new(assume_role_policy, vec![]).build();

    RoleBuilder::new("myRole", properties).build(&mut stack_builder);
}

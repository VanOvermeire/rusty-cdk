use cloud_infra_core::dynamodb::DynamoDBKey;
use cloud_infra_core::dynamodb::DynamoDBTableBuilder;
use cloud_infra_core::dynamodb::AttributeType;
use cloud_infra_core::s3::builder::S3BucketBuilder;
use cloud_infra_core::sqs::{SqsQueueBuilder};
use cloud_infra_core::stack::StackBuilder;
use cloud_infra_core::wrappers::{
    StringWithOnlyAlphaNumericsAndUnderscores, DelaySeconds, MaximumMessageSize,
    MessageRetentionPeriod, VisibilityTimeout, ReceiveMessageWaitTime, NonZeroNumber
};
use cloud_infra_macros::{
    string_with_only_alpha_numerics_and_underscores, delay_seconds, maximum_message_size,
    message_retention_period, visibility_timeout, receive_message_wait_time, non_zero_number
};

#[test]
fn dynamodb_builder_should_compile() {
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let _ = DynamoDBTableBuilder::new("myTable", DynamoDBKey::new(key, AttributeType::String))
        .pay_per_request_billing()
        .build();
}

#[test]
fn sqs_standard_queue_builder_should_compile() {
    let queue_name = string_with_only_alpha_numerics_and_underscores!("test_queue");
    let delay = delay_seconds!(300);
    let max_size = maximum_message_size!(262144);
    let retention = message_retention_period!(345600);
    let timeout = visibility_timeout!(30);
    let wait_time = receive_message_wait_time!(10);
    let max_receive = non_zero_number!(3);
    
    let _ = SqsQueueBuilder::new("myQueue")
        .standard_queue()
        .queue_name(queue_name)
        .delay_seconds(delay)
        .maximum_message_size(max_size)
        .message_retention_period(retention)
        .visibility_timeout(timeout)
        .receive_message_wait_time_seconds(wait_time)
        .dead_letter_queue("arn:aws:sqs:us-east-1:123456789012:dlq", max_receive)
        .sqs_managed_sse_enabled(true)
        .build();
}

#[test]
fn sqs_fifo_queue_builder_should_compile() {
    let queue_name = string_with_only_alpha_numerics_and_underscores!("test_fifo_queue");
    let delay = delay_seconds!(60);
    let timeout = visibility_timeout!(120);
    
    let _ = SqsQueueBuilder::new("myQueue")
        .fifo_queue()
        .queue_name(queue_name)
        .delay_seconds(delay)
        .visibility_timeout(timeout)
        .content_based_deduplication(true)
        .build();
}

// TODO more of these tests
#[test]
fn stack_with_bucket_website_should_compile() {
    let (bucket, policy) = S3BucketBuilder::new("website")
        .website("index.com")
        .build();

    let stack = StackBuilder::new()
        .add_resource(bucket)
        .add_resource(policy)
        .build();

    assert!(stack.is_ok());
}

#[test]
fn stack_with_missing_bucket_should_err() {
    let (_, policy) = S3BucketBuilder::new("website")
        .website("index.com")
        .build();

    let stack = StackBuilder::new()
        // did not add bucket
        .add_resource(policy)
        .build();

    assert!(stack.is_err());
}

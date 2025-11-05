#![allow(dead_code)]

use cloud_infra_macros::{
    iam_action, non_zero_number, string_with_only_alpha_numerics_and_underscores,
    string_with_only_alpha_numerics_underscores_and_hyphens, env_var_key,
    memory, timeout, delay_seconds, maximum_message_size, message_retention_period,
    visibility_timeout, receive_message_wait_time, sqs_event_source_max_concurrency,
    log_retention, log_group_name
};

// placeholders for the wrapper structs that exist in the core package //
struct NonZeroNumber(u32);
struct IamAction(pub String);
struct StringWithOnlyAlphaNumericsAndUnderscores(String);
struct StringWithOnlyAlphaNumericsUnderscoresAndHyphens(String);
struct EnvVarKey(String);
struct Memory(u16);
struct Timeout(u16);
struct DelaySeconds(u16);
struct MaximumMessageSize(u32);
struct MessageRetentionPeriod(u32);
struct VisibilityTimeout(u32);
struct ReceiveMessageWaitTime(u16);
struct SqsEventSourceMaxConcurrency(u16);
struct RetentionInDays(u16);
struct LogGroupName(String);

#[test]
fn create_non_zero_number_should_compile_for_non_zero_number() {
    non_zero_number!(1);
}

#[test]
fn create_iam_action_with_wildcard() {
    iam_action!("s3:*");
}

#[test]
fn create_iam_action_with_specific_action() {
    iam_action!("dynamodb:BatchGetItem");
}

#[test]
fn create_string_with_only_alpha_numerics_and_underscores() {
    string_with_only_alpha_numerics_and_underscores!("valid_identifier_123");
}

#[test]
fn create_string_with_only_alpha_numerics_underscores_and_hyphens() {
    string_with_only_alpha_numerics_underscores_and_hyphens!("valid-identifier_123");
}

#[test]
fn create_env_var_key() {
    env_var_key!("MY_ENV_VAR");
}

#[test]
fn create_memory_with_valid_value() {
    memory!(512);
}

#[test]
fn create_timeout_with_valid_value() {
    timeout!(30);
}

#[test]
fn create_delay_seconds_with_valid_value() {
    delay_seconds!(60);
}

#[test]
fn create_maximum_message_size_with_valid_value() {
    maximum_message_size!(262144);
}

#[test]
fn create_message_retention_period_with_valid_value() {
    message_retention_period!(345600);
}

#[test]
fn create_visibility_timeout_with_valid_value() {
    visibility_timeout!(300);
}

#[test]
fn create_receive_message_wait_time_with_valid_value() {
    receive_message_wait_time!(10);
}

#[test]
fn create_sqs_event_source_max_concurrency_with_valid_value() {
    sqs_event_source_max_concurrency!(100);
}

#[test]
fn create_log_retention_with_valid_value() {
    log_retention!(7);
}

#[test]
fn create_log_group_name_with_valid_value() {
    log_group_name!("/aws/lambda/my-function");
}
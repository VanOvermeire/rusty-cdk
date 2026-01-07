#![allow(dead_code)]

use rusty_cdk_macros::{iam_action, non_zero_number, string_with_only_alphanumerics_and_underscores, string_with_only_alphanumerics_underscores_and_hyphens, env_var_key, memory, timeout, delay_seconds, maximum_message_size, message_retention_period, visibility_timeout, receive_message_wait_time, sqs_event_source_max_concurrency, log_retention, log_group_name, lifecycle_object_sizes, lambda_permission_action, lifecycle_transition_in_days, location_uri, app_sync_api_name, channel_namespace_name, bucket_tiering, retry_policy_event_age, retry_policy_retries, max_flexible_time_window, schedule_rate_expression, schedule_name, schedule_cron_expression, schedule_at_expression, policy_name};

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
struct ReceiveMessageWaitTime(u8);
struct SqsEventSourceMaxConcurrency(u16);
struct RetentionInDays(u16);
struct LogGroupName(String);
struct S3LifecycleObjectSizes(pub Option<u32>, pub Option<u32>);
struct LambdaPermissionAction(pub String);
struct LifecycleTransitionInDays(pub u16);
struct LocationUri(pub String);
struct AppSyncApiName(pub String);
struct ChannelNamespaceName(pub String);
struct BucketTiering(pub String, pub u16);
struct RetryPolicyEventAge(pub u32);
struct RetryPolicyRetries(pub u8);
struct MaxFlexibleTimeWindow(pub u16);
struct ScheduleRateExpression(pub u16, pub String);
struct ScheduleCronExpression(pub String);
struct ScheduleAtExpression(pub String);
struct ScheduleName(pub String);
struct PolicyName(pub String);

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
fn create_string_with_only_alphanumerics_and_underscores() {
    string_with_only_alphanumerics_and_underscores!("valid_identifier_123");
}

#[test]
fn create_string_with_only_alphanumerics_underscores_and_hyphens() {
    string_with_only_alphanumerics_underscores_and_hyphens!("valid-identifier_123");
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

#[test]
fn create_object_sizes_two_sizes() {
    let val = lifecycle_object_sizes!(5000,10000);

    assert_eq!(Some(5000), val.0);
    assert_eq!(Some(10000), val.1);
}

#[test]
fn create_object_sizes_first_size() {
    let val = lifecycle_object_sizes!(5000);

    assert_eq!(Some(5000), val.0);
    assert_eq!(None, val.1);
}

#[test]
fn create_object_sizes_second_size() {
    let val = lifecycle_object_sizes!(,10000);

    assert_eq!(None, val.0);
    assert_eq!(Some(10000), val.1);
}

#[test]
fn lambda_permission_action_with_right_prefix() {
    lambda_permission_action!("lambda:InvokeFunction");
}

#[test]
fn lifecycle_transition_in_days_more_than_30_days() {
    lifecycle_transition_in_days!(31,"StandardIA");
}

#[test]
fn lifecycle_transition_in_days() {
    lifecycle_transition_in_days!(3,"Glacier");
}

#[test]
fn location_uri_hosted() {
    location_uri!("hosted");
}

#[test]
fn location_uri_s3() {
    location_uri!("s3","s3://something");
}

#[test]
fn location_uri_secretsmanager() {
    location_uri!("secretsmanager","secretsmanager://something");
}

#[test]
fn location_uri_codepipeline() {
    location_uri!("codepipeline","codepipeline://something");
}

#[test]
fn app_sync_api_name() {
    app_sync_api_name!("some-API name");
}

#[test]
fn channel_namespace_name() {
    channel_namespace_name!("default");
}

#[test]
fn bucket_tiering_archive() {
    bucket_tiering!("ARCHIVE_ACCESS",100);
}

#[test]
fn bucket_tiering_deep_archive() {
    bucket_tiering!("DEEP_ARCHIVE_ACCESS",181);
}

#[test]
fn retry_policy_event_age() {
    retry_policy_event_age!(400);
}

#[test]
fn retry_policy_retries() {
    retry_policy_retries!(180);
}

#[test]
fn max_flexible_time_window() {
    max_flexible_time_window!(1220);
}

#[test]
fn schedule_rate_expression() {
    schedule_rate_expression!(1220,"days");
}

#[test]
fn schedule_cron_expression() {
    schedule_cron_expression!("2 8 * * *");
}

#[test]
fn schedule_at_expression() {
    schedule_at_expression!("2010-10-5T23:59:59");
}

#[test]
fn schedule_name() {
    schedule_name!("schedule-some_name");
}
#[test]
fn policy_name() {
    policy_name!("my-valid+policy_name");
}

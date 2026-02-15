#![allow(dead_code)]

use rusty_cdk_macros::{
    app_config_name, app_sync_api_name, bucket_tiering, channel_namespace_name, default_root_object, delay_seconds, doc_db_capacity_units, doc_db_instance_class, doc_db_master_pass, doc_db_master_username, ecr_repository_name, env_var_key, iam_action, image_tag_mutability_exclusion_filter_value, lambda_permission_action, lifecycle_object_sizes, lifecycle_transition_in_days, location_uri, log_group_name, log_retention, max_flexible_time_window, maximum_message_size, memory, message_retention_period, non_zero_number, origin_path, policy_name, receive_message_wait_time, repo_about_text, repo_description, repo_prefix, retry_policy_event_age, retry_policy_retries, schedule_at_expression, schedule_cron_expression, schedule_name, schedule_rate_expression, sqs_event_source_max_concurrency, string_for_secret, string_with_only_alphanumerics_and_hyphens, string_with_only_alphanumerics_and_underscores, string_with_only_alphanumerics_underscores_and_hyphens, timeout, toml_file, topic_display_name, url, visibility_timeout, zip_file
};

// placeholders for the wrapper structs that exist in the core package //
struct NonZeroNumber(u32);
struct IamAction(pub String);
struct StringWithOnlyAlphaNumericsAndUnderscores(String);
struct StringWithOnlyAlphaNumericsUnderscoresAndHyphens(String);
struct StringWithOnlyAlphaNumericsAndHyphens(String);
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
struct S3LifecycleObjectSizes(Option<u32>, Option<u32>);
struct LambdaPermissionAction(String);
struct LifecycleTransitionInDays(u16);
struct LocationUri(String);
struct AppSyncApiName(String);
struct TopicDisplayName(String);
struct AppConfigName(String);
struct ChannelNamespaceName(String);
struct BucketTiering(String, u16);
struct RetryPolicyEventAge(u32);
struct RetryPolicyRetries(u8);
struct MaxFlexibleTimeWindow(u16);
struct ScheduleRateExpression(u16, String);
struct ScheduleCronExpression(String);
struct ScheduleAtExpression(String);
struct ScheduleName(String);
struct PolicyName(String);
struct OriginPath(String);
struct ZipFile(String);
struct DefaultRootObject(String);
struct RepoAboutText(String);
struct RepoDescription(String);
struct RepoPrefix(String);
struct EcrRepositoryName(String);
struct URL(String);
struct ImageTagMutabilityExclusionFilterValue(String);
struct DocDbCapacityUnits(f32);
struct DocDbInstanceClass(String);
struct DocDbMasterUsername(String);
struct DocDbMasterPassword(String);

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
    let val = lifecycle_object_sizes!(5000, 10000);

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
    lifecycle_transition_in_days!(31, "StandardIA");
}

#[test]
fn lifecycle_transition_in_days() {
    lifecycle_transition_in_days!(3, "Glacier");
}

#[test]
fn location_uri_hosted() {
    location_uri!("hosted");
}

#[test]
fn location_uri_s3() {
    location_uri!("s3", "s3://something");
}

#[test]
fn location_uri_secretsmanager() {
    location_uri!("secretsmanager", "secretsmanager://something");
}

#[test]
fn location_uri_codepipeline() {
    location_uri!("codepipeline", "codepipeline://something");
}

#[test]
fn app_sync_api_name() {
    app_sync_api_name!("some-API name");
}

#[test]
fn app_config_name_valid() {
    app_config_name!("my-app-config");
}

#[test]
fn channel_namespace_name() {
    channel_namespace_name!("default");
}

#[test]
fn bucket_tiering_archive() {
    bucket_tiering!("ARCHIVE_ACCESS", 100);
}

#[test]
fn bucket_tiering_deep_archive() {
    bucket_tiering!("DEEP_ARCHIVE_ACCESS", 181);
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
    schedule_rate_expression!(1220, "days");
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

#[test]
fn create_string_with_only_alphanumerics_and_hyphens() {
    string_with_only_alphanumerics_and_hyphens!("valid-identifier-123");
}

struct StringForSecret(String);

#[test]
fn string_for_secret_valid() {
    string_for_secret!("a/b_c+d=e.f@g-h1");
}

struct TomlFile(String);

#[test]
fn toml_file_valid() {
    toml_file!("examples/apigateway_lambda_dynamodb/Cargo.toml");
}

#[test]
fn origin_path_valid() {
    origin_path!("/my-path");
}

#[test]
fn default_root_object_valid() {
    default_root_object!("index.html");
}

#[test]
fn zip_file_valid() {
    zip_file!("examples/apigateway_lambda_dynamodb/files/empty.zip");
}

#[test]
fn topic_display_name_valid() {
    topic_display_name!("my-topic_display name");
}

#[test]
fn create_string_with_only_alphanumerics_and_hyphens_alpha() {
    string_with_only_alphanumerics_and_hyphens!("validalpha");
}

#[test]
fn create_string_with_only_alphanumerics_and_hyphens_numeric() {
    string_with_only_alphanumerics_and_hyphens!("12345");
}

#[test]
fn create_string_with_only_alphanumerics_and_hyphens_mixed() {
    string_with_only_alphanumerics_and_hyphens!("another-valid-id-456");
}

#[test]
fn create_repo_about_text() {
    repo_about_text!("some description");
}

#[test]
fn create_repo_description() {
    repo_description!("some description");
}

#[test]
fn create_ecr_repository_name() {
    ecr_repository_name!("some-name");
}

#[test]
fn create_repo_prefix() {
    repo_prefix!("some-name");
}

#[test]
fn create_url() {
    url!("https://example.com");
}

#[test]
fn create_url_with_subdomain() {
    url!("https://example.com/subdomain");
}

#[test]
fn create_url_without_https() {
    url!("example.com");
}

#[test]
fn create_image_tag_mutability_exclusion_filter_value() {
    image_tag_mutability_exclusion_filter_value!("some-filter*");
}

#[test]
fn create_document_db_capacity_units() {
    doc_db_capacity_units!(32.0);
}

#[test]
fn create_document_db_capacity_units_half_unit() {
    doc_db_capacity_units!(32.5);
}

#[test]
fn document_db_instance_class() {
    doc_db_instance_class!("db.t4.2xlarge");
}

#[test]
fn document_db_username() {
    doc_db_master_username!("myusername");
}

#[test]
fn document_db_pass() {
    doc_db_master_pass!("some-password");
}

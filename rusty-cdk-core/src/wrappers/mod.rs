//! Type-safe wrapper types
//!
//! This module provides newtype wrappers that enforce type safety and validation
//! for various configuration values used in AWS resources. These wrappers help
//! prevent common mistakes like using invalid identifiers, zero values where
//! positive numbers are required, or invalid memory/timeout configurations.
//!
//! # Creating Wrappers
//! 
//! ** Recommended approach: ** Use the compile-time validated proc macros from the
//! `rusty-cdk-macros` crate for type safety and validation at compile time.
//! 
//! ** Direct creation: ** While these wrappers can be created directly by calling
//! their constructors, this bypasses compile-time validation and should only be
//! used as an override when you need runtime flexibility.
//!
//! # Example
//! ```rust
//! use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
//! use rusty_cdk_macros::string_with_only_alphanumerics_and_underscores;
//! use rusty_cdk_core::wrappers::{Memory};
//!
//! // Preferred: Use the macro for compile-time validation
//! let function_name = string_with_only_alphanumerics_and_underscores!("my_lambda_function");
//!
//! // Direct creation (use _sparingly_, as an override)
//! let memory = Memory(512);  // 512 MB
//! ```

/// A string wrapper that ensures the content contains only letters, numbers, and underscores.
///
/// This wrapper is designed to create safe identifiers for AWS resources that have
/// naming restrictions. It helps prevent runtime errors by enforcing valid character sets.
///
/// # Validation Rules
/// - Only alphanumeric characters (a-z, A-Z, 0-9) and underscores (_) are allowed
/// - Underscores can appear in any position (beginning, middle, or end)
///
/// # Recommended Usage
/// Use the `string_with_only_alphanumerics_and_underscores!` macro from `rusty-cdk-macros`
/// for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
/// use rusty_cdk_macros::string_with_only_alphanumerics_and_underscores;
///
/// let function_name = string_with_only_alphanumerics_and_underscores!("my_lambda_function");
/// ```
#[derive(Debug, Clone)]
pub struct StringWithOnlyAlphaNumericsAndUnderscores(pub String);

/// A string wrapper that ensures the content contains only letters, numbers, underscores, and hyphens.
///
/// # Validation Rules
/// - Only alphanumeric characters (a-z, A-Z, 0-9), underscores (_), and hyphens (-) are allowed
///
/// # Recommended Usage
/// Use the `string_with_only_alphanumerics_underscores_and_hyphens!` macro from `rusty-cdk-macros`
/// for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsUnderscoresAndHyphens;
/// use rusty_cdk_macros::string_with_only_alphanumerics_underscores_and_hyphens;
///
/// let stack_name = string_with_only_alphanumerics_underscores_and_hyphens!("my-function-name");
/// ```
#[derive(Debug, Clone)]
pub struct StringWithOnlyAlphaNumericsUnderscoresAndHyphens(pub String);

/// A string wrapper that ensures the content contains only letters, numbers, and hyphens.
///
/// This wrapper is similar to `StringWithOnlyAlphaNumericsAndUnderscores` but also allows hyphens,
/// making it suitable for AWS resources that accept hyphenated identifiers (like stack names).
///
/// # Validation Rules
/// - Only alphanumeric characters (a-z, A-Z, 0-9), underscores (_), and hyphens (-) are allowed
///
/// # Recommended Usage
/// Use the `string_with_only_alphanumerics_underscores_and_hyphens!` macro from `rusty-cdk-macros`
/// for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
/// use rusty_cdk_macros::string_with_only_alphanumerics_and_hyphens;
///
/// let stack_name = string_with_only_alphanumerics_and_hyphens!("my-stack-name");
/// ```
#[derive(Debug, Clone)]
pub struct StringWithOnlyAlphaNumericsAndHyphens(pub String);

/// A string wrapper for AWS Secrets Manager secret names.
///
/// This wrapper ensures type safety when defining secret names for AWS Secrets Manager.
///
/// # Validation Rules
/// - String must not be empty
/// - Only alphanumeric characters and the following special characters are allowed: / _ + = . @ -
/// - Maximum length of 512 characters (AWS Secrets Manager limit)
///
/// # Recommended Usage
/// Use the `string_for_secret!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::StringForSecret;
/// use rusty_cdk_macros::string_for_secret;
///
/// let secret_name = string_for_secret!("myapp/database/password");
/// ```
#[derive(Debug, Clone)]
pub struct StringForSecret(pub String);

/// A wrapper for positive integers that must be greater than zero.
///
/// This wrapper ensures that numeric configuration values are always positive,
/// preventing common configuration errors where zero values would cause
/// AWS resource creation to fail or behave unexpectedly.
///
/// # Use Cases
/// - Any configuration where zero would be invalid
///
/// # Recommended Usage
/// Use the `non_zero_number!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::NonZeroNumber;
/// use rusty_cdk_macros::non_zero_number;
/// 
/// let capacity = non_zero_number!(10);  // Compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct NonZeroNumber(pub u32);

/// Memory allocation configuration for AWS Lambda functions, specified in megabytes.
///
/// This wrapper ensures type safety when configuring Lambda function memory settings.
/// AWS Lambda has specific constraints on memory allocation that this wrapper helps enforce
/// through its type system.
///
/// # AWS Lambda Memory Constraints
/// - Minimum: 128 MB
/// - Maximum: 10,240 MB (10 GB)
/// - Memory allocation affects pricing and available CPU resources
///
/// # Recommended Usage
/// Use the `memory!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::Memory;
/// use rusty_cdk_macros::memory;
/// 
/// let mem = memory!(512);   // Compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Memory(pub u16);

/// Timeout configuration for AWS Lambda functions, specified in seconds.
///
/// This wrapper ensures type safety when configuring Lambda function timeout settings.
/// AWS Lambda has specific constraints on timeout duration that this wrapper helps enforce
/// through its type system.
///
/// # AWS Lambda Timeout Constraints
/// - Minimum: 1 second
/// - Maximum: 900 seconds (15 minutes)
/// - Timeout affects pricing and determines maximum execution duration
///
/// # Recommended Usage
/// Use the `timeout!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::Timeout;
/// use rusty_cdk_macros::timeout;
/// 
/// let timeout_val = timeout!(30);   // Compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Timeout(pub u16);

/// Environment variable key wrapper for AWS Lambda function configuration.
///
/// This wrapper ensures type safety when defining environment variable keys
/// for Lambda functions. It helps prevent typos and ensures consistent
/// naming of environment variables across your infrastructure.
///
/// # AWS Environment Variable Constraints
/// - Minimum length of 2
/// - Should start with a letter of number
/// - Should only contain letters, numbers and underscores
///
/// # Recommended Usage
/// Use the `env_var_key!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::EnvVarKey;
/// use rusty_cdk_macros::env_var_key;
/// 
/// let db_url = env_var_key!("DATABASE_URL");   // Compile-time validated
/// ```
#[derive(Debug, Clone)]
pub struct EnvVarKey(pub String);

/// File path wrapper for AWS Lambda deployment package ZIP files.
///
/// This wrapper ensures type safety when specifying the location of ZIP files
/// containing Lambda function code.
///
/// # Use Cases
/// - Lambda function deployment packages
/// - Any AWS resource requiring ZIP file uploads
///
/// # Path Requirements
/// - Should be a valid file path to a ZIP file
/// - Can be relative or absolute paths
/// - File should exist and be accessible at deployment time
///
/// # Recommended Usage
/// Use the `zip_file!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust,compile_fail
/// use rusty_cdk_core::wrappers::ZipFile;
/// use rusty_cdk_macros::zip_file;
/// 
/// let lambda_code = zip_file!("./target/lambda/function.zip");   // Compile-time validated
/// ```
#[derive(Debug, Clone)]
pub struct ZipFile(pub String);

/// Delay seconds wrapper for AWS SQS queue configuration.
///
/// This wrapper ensures type safety when configuring SQS queue delay settings.
/// AWS SQS has specific constraints on delay duration that this wrapper helps enforce.
///
/// # AWS SQS DelaySeconds Constraints
/// - Minimum: 0 seconds (no delay)
/// - Maximum: 900 seconds (15 minutes)
/// - Affects all messages sent to the queue
///
/// # Recommended Usage
/// Use the `delay_seconds!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::DelaySeconds;
/// use rusty_cdk_macros::delay_seconds;
/// 
/// let delay = delay_seconds!(300);   // Compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct DelaySeconds(pub u16);

/// Maximum message size wrapper for AWS SQS queue configuration.
///
/// This wrapper ensures type safety when configuring SQS queue message size limits.
/// AWS SQS has specific constraints on message size that this wrapper helps enforce.
///
/// # AWS SQS MaximumMessageSize Constraints
/// - Minimum: 1,024 bytes (1 KiB)
/// - Maximum: 1,048,576 bytes (1 MiB)
/// - Messages larger than this limit will be rejected
///
/// # Recommended Usage
/// Use the `maximum_message_size!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::MaximumMessageSize;
/// use rusty_cdk_macros::maximum_message_size;
/// 
/// let max_size = maximum_message_size!(262144);   // 256 KiB, compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MaximumMessageSize(pub u32);

/// Message retention period wrapper for AWS SQS queue configuration.
///
/// This wrapper ensures type safety when configuring SQS queue message retention settings.
/// AWS SQS has specific constraints on the retention period that this wrapper helps enforce.
///
/// # AWS SQS MessageRetentionPeriod Constraints
/// - Minimum: 60 seconds (1 minute)
/// - Maximum: 1,209,600 seconds (14 days)
/// - Determines how long messages are kept in the queue
///
/// # Recommended Usage
/// Use the `message_retention_period!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::MessageRetentionPeriod;
/// use rusty_cdk_macros::message_retention_period;
/// 
/// let retention = message_retention_period!(345600);   // 4 days, compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MessageRetentionPeriod(pub u32);

/// Visibility timeout wrapper for AWS SQS queue configuration.
///
/// This wrapper ensures type safety when configuring SQS queue visibility timeout settings.
/// AWS SQS has specific constraints on visibility timeout that this wrapper helps enforce.
///
/// # AWS SQS VisibilityTimeout Constraints
/// - Minimum: 0 seconds
/// - Maximum: 43,200 seconds (12 hours)
/// - Determines how long messages remain invisible after being received
///
/// # Recommended Usage
/// Use the `visibility_timeout!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::VisibilityTimeout;
/// use rusty_cdk_macros::visibility_timeout;
/// 
/// let timeout = visibility_timeout!(30);   // Compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct VisibilityTimeout(pub u32);

/// Receive message wait time wrapper for AWS SQS queue configuration.
///
/// This wrapper ensures type safety when configuring SQS queue long polling settings.
/// AWS SQS has specific constraints on receive message wait time that this wrapper helps enforce.
///
/// # AWS SQS ReceiveMessageWaitTimeSeconds Constraints
/// - Minimum: 0 seconds (short polling)
/// - Maximum: 20 seconds (long polling)
/// - Enables long polling when greater than 0
///
/// # Recommended Usage
/// Use the `receive_message_wait_time!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::ReceiveMessageWaitTime;
/// use rusty_cdk_macros::receive_message_wait_time;
/// 
/// let wait_time = receive_message_wait_time!(10);   // Compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct ReceiveMessageWaitTime(pub u16);

/// Maximum concurrency configuration for SQS event sources in AWS Lambda.
///
/// This wrapper ensures type safety when configuring the maximum number of concurrent Lambda function invocations.
///
/// # AWS Lambda SQS Event Source Concurrency Constraints
/// - Minimum: 2 concurrent invocations
/// - Maximum: 1,000 concurrent invocations
///
/// # Recommended Usage
/// Use the `sqs_event_source_max_concurrency!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::SqsEventSourceMaxConcurrency;
/// use rusty_cdk_macros::sqs_event_source_max_concurrency;
///
/// let max_concurrency = sqs_event_source_max_concurrency!(10);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct SqsEventSourceMaxConcurrency(pub u16);

/// A wrapper for referencing existing AWS S3 buckets.
///
/// This wrapper ensures type safety when referencing existing S3 buckets in your AWS account.
/// Use this when you need to reference a bucket that already exists, as opposed to creating
/// a new one (use `BucketName` for new buckets).
///
/// # Validation Rules (when using the macro)
/// - Value must not be an ARN (cannot start with "arn:")
/// - Value must not include the "s3:" prefix
/// - Bucket must exist in your AWS account (verified at compile time by the macro)
///
/// # Recommended Usage
/// Use the `bucket!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// # Note
/// The `bucket!` macro queries AWS at compile time to verify the bucket exists and caches
/// the result for faster subsequent compilations. Set `RUSTY_CDK_NO_REMOTE=true` to skip
/// remote checks during development.
#[derive(Debug, Clone)]
pub struct Bucket(pub String);

/// Retention period configuration for AWS CloudWatch Logs log groups, specified in days.
///
/// This wrapper ensures type safety when configuring log retention periods for CloudWatch Logs.
/// AWS CloudWatch only allows specific discrete values for retention periods, not arbitrary numbers.
///
/// # AWS CloudWatch Logs Retention Constraints
/// - Must be one of the following values (in days):
///   1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, 3653
///
/// # Recommended Usage
/// Use the `log_retention!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::RetentionInDays;
/// use rusty_cdk_macros::log_retention;
///
/// let retention = log_retention!(30);  // 30 days
/// ```
#[derive(Debug, Copy, Clone)]
pub struct RetentionInDays(pub u16);

/// A wrapper for AWS CloudWatch Logs log group names.
///
/// This wrapper ensures type safety when defining log group names for CloudWatch Logs.
///
/// # Validation Rules
/// - String must not be empty
/// - Maximum length of 512 characters
/// - Only alphanumeric characters and the following special characters are allowed: . - _ # / \
///
/// # Recommended Usage
/// Use the `log_group_name!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::LogGroupName;
/// use rusty_cdk_macros::log_group_name;
///
/// let log_group = log_group_name!("/aws/lambda/my-function");
/// ```
#[derive(Debug, Clone)]
pub struct LogGroupName(pub String);

/// A wrapper for creating new AWS S3 bucket names.
///
/// This wrapper ensures type safety when defining names for new S3 buckets, following AWS
/// naming requirements and verifying global uniqueness at compile time.
///
/// # Validation Rules (when using the macro)
/// - Must contain only lowercase letters, numbers, periods (.), and hyphens (-)
/// - No uppercase letters are allowed
/// - Bucket name must be globally unique and available (verified at compile time)
/// - Must be between 3 and 63 characters long
///
/// # Recommended Usage
/// Use the `bucket_name!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// # Note
/// The `bucket_name!` macro queries AWS at compile time to verify the bucket name is globally
/// available and caches the result for faster subsequent compilations. Set `RUSTY_CDK_NO_REMOTE=true`
/// to skip remote checks during development.
#[derive(Debug, Clone)]
pub struct BucketName(pub String);

/// A wrapper for AWS IAM action permissions.
///
/// This wrapper ensures type safety when defining IAM policy actions, helping prevent
/// runtime IAM policy errors by validating permissions at compile time against AWS's
/// official permission list.
///
/// # Validation Rules (when using the macro)
/// - String must not be empty
/// - Action must be a valid AWS IAM action (e.g., "s3:GetObject", "dynamodb:Query")
/// - Wildcards are supported (e.g., "s3:*", "dynamodb:Get*")
///
/// # Recommended Usage
/// Use the `iam_action!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::IamAction;
/// use rusty_cdk_macros::iam_action;
///
/// let action = iam_action!("s3:GetObject");
/// let wildcard_action = iam_action!("s3:Put*");
/// ```
#[derive(Debug, Clone)]
pub struct IamAction(pub String);

/// Configuration for object size constraints in S3 lifecycle rules, specified in bytes.
///
/// This wrapper defines minimum and maximum object sizes for S3 lifecycle transitions.
/// It allows you to apply lifecycle rules only to objects within a specific size range.
///
/// # Structure
/// - First value: Minimum object size (optional)
/// - Second value: Maximum object size (optional)
/// - Both values are in bytes
///
/// # Validation Rules (when using the macro)
/// - If both values are provided, the first must be smaller than the second
/// - Values represent object sizes in bytes
///
/// # Recommended Usage
/// Use the `lifecycle_object_sizes!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::S3LifecycleObjectSizes;
/// use rusty_cdk_macros::lifecycle_object_sizes;
///
/// let sizes = lifecycle_object_sizes!(1024,10485760);
///
/// let max_only = lifecycle_object_sizes!(5242880);
/// ```
#[derive(Debug, Clone)]
pub struct S3LifecycleObjectSizes(pub Option<u32>, pub Option<u32>);

/// A wrapper for TOML configuration file paths.
///
/// This wrapper ensures type safety when specifying paths to TOML configuration files
/// used in infrastructure definitions.
#[derive(Debug, Clone)]
pub struct TomlFile(pub String); // TODO add docs for macro

/// Number of connection attempts for CloudFront origin connections.
///
/// This wrapper configures how many times CloudFront will attempt to connect to an origin
/// before failing the request.
///
/// # AWS CloudFront Connection Attempts Constraints
/// - Minimum: 1 attempt
/// - Maximum: 3 attempts
/// - Determines retry behavior for origin connection failures
///
/// # Recommended Usage
/// Use the `connection_attempts!` macro from `rusty-cdk-macros` for compile-time validation:
#[derive(Debug, Clone)]
pub struct ConnectionAttempts(pub u16);

/// Connection timeout configuration for CloudFront origins, specified in seconds.
///
/// This wrapper defines timeout settings for CloudFront when connecting to origins.
/// It allows configuration of both connection timeout and response completion timeout.
///
/// # Structure
/// - First value: Connection timeout in seconds (optional, 1-10 seconds)
/// - Second value: Response completion timeout in seconds (optional, must be >= connection timeout)
///
/// # Recommended Usage
/// Use the `cf_connection_timeout!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::CfConnectionTimeout;
/// use rusty_cdk_core::wrappers::S3LifecycleObjectSizes;
/// use rusty_cdk_macros::cf_connection_timeout;
///
/// let timeouts = cf_connection_timeout!(5,30);
///
/// let conn_only = cf_connection_timeout!(3);
/// ```
#[derive(Debug, Clone)]
pub struct CfConnectionTimeout(pub Option<u16>, pub Option<u16>);

/// Path prefix for CloudFront origin requests.
///
/// This wrapper specifies a directory path that CloudFront appends to requests forwarded
/// to the origin. Useful when the origin content is located in a subdirectory.
///
/// # Recommended Usage
/// Use the `origin_path!` macro from `rusty-cdk-macros` for compile-time validation:
#[derive(Debug, Clone)]
pub struct OriginPath(pub String);

/// Default root object for CloudFront distributions.
///
/// This wrapper specifies the object that CloudFront returns when a viewer requests
/// the root URL of your distribution (e.g., http://example.com/ instead of
/// http://example.com/index.html).
/// # Recommended Usage
/// Use the `default_root_object!` macro from `rusty-cdk-macros` for compile-time validation:
#[derive(Debug, Clone)]
pub struct DefaultRootObject(pub String);

/// Read timeout for S3 origin in CloudFront, specified in seconds.
///
/// This wrapper configures how long CloudFront waits for a response from an S3 origin
/// before timing out the request.
///
/// # AWS CloudFront S3 Origin Read Timeout Constraints
/// - Minimum: 1 second
/// - Maximum: 120 seconds
/// - Determines maximum wait time for S3 to respond
///
/// # Recommended Usage
/// Use the `s3_origin_read_timeout!` macro from `rusty-cdk-macros` for compile-time validation:
#[derive(Debug, Clone)]
pub struct S3OriginReadTimeout(pub u16);

/// Action specification for AWS Lambda resource-based policy permissions.
///
/// This wrapper ensures type safety when defining actions for Lambda resource-based policies,
/// which control what AWS services and accounts can invoke your Lambda functions.
///
/// # Recommended Usage
/// Use the `lambda_permission_action!` macro from `rusty-cdk-macros` for compile-time validation:
#[derive(Debug, Clone)]
pub struct LambdaPermissionAction(pub String);

/// Name for AWS AppConfig applications, environments, or configuration profiles.
///
/// This wrapper ensures type safety when defining names for AppConfig resources,
/// following AWS AppConfig naming conventions and length restrictions.
///
/// # Recommended Usage
/// Use the `app_config_name!` macro from `rusty-cdk-macros` for compile-time validation:
#[derive(Debug, Clone)]
pub struct AppConfigName(pub String);

/// Deployment duration for AWS AppConfig deployments, specified in minutes.
///
/// This wrapper configures how long an AppConfig deployment takes to complete.
/// AppConfig gradually rolls out configuration changes over this period to minimize
/// the impact of potential issues.
///
/// # Recommended Usage
/// Use the `deployment_duration_in_minutes!` macro from `rusty-cdk-macros` for compile-time validation:
///
#[derive(Debug, Clone)]
pub struct DeploymentDurationInMinutes(pub u16);

/// Growth factor percentage for AWS AppConfig deployment strategies.
///
/// This wrapper configures the percentage of targets to receive the configuration
/// during each deployment interval. AppConfig uses this to gradually roll out changes.
///
/// # Recommended Usage
/// Use the `growth_factor!` macro from `rusty-cdk-macros` for compile-time validation:
///
#[derive(Debug, Clone)]
pub struct GrowthFactor(pub u16);

/// Number of days before S3 objects transition to a different storage class.
///
/// This wrapper configures S3 lifecycle rules to automatically transition objects
/// to different storage classes after a specified number of days, helping optimize
/// storage costs.
///
/// # Recommended Usage
/// Use the `lifecycle_transition_in_days!` macro from `rusty-cdk-macros` for compile-time validation:
///
/// ```rust
/// use rusty_cdk_core::wrappers::LifecycleTransitionInDays;
/// use rusty_cdk_macros::lifecycle_transition_in_days;
///
/// let transition = lifecycle_transition_in_days!(90,Glacier);
///
/// let ia_transition = lifecycle_transition_in_days!(31,StandardIA);
/// ```
#[derive(Debug, Clone)]
pub struct LifecycleTransitionInDays(pub u16);
//! Type-safe wrapper types for cloud infrastructure configuration.
//!
//! This module provides newtype wrappers that enforce type safety and validation
//! for various configuration values used in AWS resources. These wrappers help
//! prevent common mistakes like using invalid identifiers, zero values where
//! positive numbers are required, or invalid memory/timeout configurations.
//!
//! # Creating Wrappers
//! 
//! ** Recommended approach: ** Use the compile-time validated proc macros from the
//! `cloud-infra-macros` crate for type safety and validation at compile time.
//! 
//! ** Direct creation: ** While these wrappers can be created directly by calling
//! their constructors, this bypasses compile-time validation and should only be
//! used as an override when you need runtime flexibility.
//!
//! # Example
//! ```rust
//! use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
//! use cloud_infra_macros::string_with_only_alpha_numerics_and_underscores;
//! use cloud_infra_core::wrappers::{Memory};
//! 
//! // Preferred: Use the macro for compile-time validation
//! let function_name = string_with_only_alpha_numerics_and_underscores!("my_lambda_function");
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
/// Use the `string_with_only_alpha_numerics_and_underscores!` macro from `cloud-infra-macros` 
/// for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
/// use cloud_infra_macros::string_with_only_alpha_numerics_and_underscores;
/// 
/// let function_name = string_with_only_alpha_numerics_and_underscores!("my_lambda_function");
/// ```
#[derive(Debug, Clone)]
pub struct StringWithOnlyAlphaNumericsAndUnderscores(pub String);

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
/// Use the `non_zero_number!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::NonZeroNumber;
/// use cloud_infra_macros::non_zero_number;
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
/// Use the `memory!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::Memory;
/// use cloud_infra_macros::memory;
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
/// Use the `timeout!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::Timeout;
/// use cloud_infra_macros::timeout;
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
/// Use the `env_var_key!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::EnvVarKey;
/// use cloud_infra_macros::env_var_key;
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
/// Use the `zipfile!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust,compile_fail
/// use cloud_infra_core::wrappers::ZipFile;
/// use cloud_infra_macros::zipfile;
/// 
/// let lambda_code = zipfile!("./target/lambda/function.zip");   // Compile-time validated
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
/// Use the `delay_seconds!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::DelaySeconds;
/// use cloud_infra_macros::delay_seconds;
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
/// Use the `maximum_message_size!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::MaximumMessageSize;
/// use cloud_infra_macros::maximum_message_size;
/// 
/// let max_size = maximum_message_size!(262144);   // 256 KiB, compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MaximumMessageSize(pub u32);

/// Message retention period wrapper for AWS SQS queue configuration.
///
/// This wrapper ensures type safety when configuring SQS queue message retention settings.
/// AWS SQS has specific constraints on retention period that this wrapper helps enforce.
///
/// # AWS SQS MessageRetentionPeriod Constraints
/// - Minimum: 60 seconds (1 minute)
/// - Maximum: 1,209,600 seconds (14 days)
/// - Determines how long messages are kept in the queue
///
/// # Recommended Usage
/// Use the `message_retention_period!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::MessageRetentionPeriod;
/// use cloud_infra_macros::message_retention_period;
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
/// Use the `visibility_timeout!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::VisibilityTimeout;
/// use cloud_infra_macros::visibility_timeout;
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
/// Use the `receive_message_wait_time!` macro from `cloud-infra-macros` for compile-time validation:
///
/// ```rust
/// use cloud_infra_core::wrappers::ReceiveMessageWaitTime;
/// use cloud_infra_macros::receive_message_wait_time;
/// 
/// let wait_time = receive_message_wait_time!(10);   // Compile-time validated
/// ```
#[derive(Debug, Copy, Clone)]
pub struct ReceiveMessageWaitTime(pub u16);

#[derive(Debug, Copy, Clone)]
pub struct SqsEventSourceMaxConcurrency(pub u16);

#[derive(Debug, Clone)]
pub struct Bucket(pub String);

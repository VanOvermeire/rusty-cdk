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
//! use cloud_infra_macros::string_with_only_alpha_numerics_and_underscores;
//! use cloud_infra_core::wrappers::{Memory, Timeout};
//! 
//! // Preferred: Use the macro for compile-time validation
//! let function_name = string_with_only_alpha_numerics_and_underscores!("my_lambda_function");
//! 
//! // Direct creation (use _sparingly_, as an override)
//! let memory = Memory(512);  // 512 MB
//! let timeout = Timeout(30); // 30 seconds
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
/// ```rust
/// use cloud_infra_macros::zipfile;
/// 
/// let lambda_code = zipfile!("./target/lambda/function.zip");   // Compile-time validated
/// ```
#[derive(Debug, Clone)]
pub struct ZipFile(pub String);

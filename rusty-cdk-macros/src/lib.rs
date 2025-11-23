#![allow(unused_comparisons)]
//! This crate provides compile-time validation macros for AWS cloud infrastructure configuration.
//! These macros ensure type safety and enforce AWS service limits at build time, preventing
//! runtime errors from invalid configurations.
//!
//! ## Overview
//!
//! All macros perform validation at compile time and generate wrapper types that encapsulate
//! validated values. This approach provides:
//!
//! - **Compile-time safety**: Invalid values are caught during compilation
//! - **Zero runtime cost**: No performance overhead for validation
//! - **Type safety**: Wrapper types prevent mixing incompatible values
//! - **IDE support**: Better code completion and error messages
//!
//! The macros always return a newtype 'wrapper'.
//! You should import those from the rusty_cdk::wrappers directory, as seen in the below examples.
//!
//! ## Usage Examples
//!
//! ```rust,compile_fail
//! use rusty_cdk::wrappers::Memory; // import the wrapper
//! use rusty_cdk::memory;
//!
//! // Lambda memory configuration with validated limit
//! let mem = memory!(512);        // 512 MB (128-10240 range)
//! ```
//!
//! ## Available Macros
//!
//! ### String and Identifier Macros
//!
//! - [`string_with_only_alphanumerics_and_underscores!`] - AWS resource identifiers
//! - [`env_var_key!`] - Lambda environment variable keys
//!
//! ### File Path Macros
//!
//! - [`zip!`] - ZIP file paths for Lambda deployment packages
//!
//! ### Numeric Validation Macros
//!
//! - [`non_zero_number!`] - Positive integers (> 0)
//!
//! ### AWS Lambda Configuration Macros
//!
//! - [`memory!`] - Memory allocation (128-10,240 MB)
//! - [`timeout!`] - Function timeout (1-900 seconds)
//!
//! ### AWS SQS Configuration Macros
//!
//! - [`delay_seconds!`] - Message delay (0-900 seconds)
//! - [`maximum_message_size!`] - Max message size (1,024-1,048,576 bytes)
//! - [`message_retention_period!`] - Retention period (60-1,209,600 seconds)
//! - [`visibility_timeout!`] - Visibility timeout (0-43,200 seconds)
//! - [`receive_message_wait_time!`] - Long polling wait time (0-20 seconds)

mod bucket;
mod bucket_name;
mod file_util;
mod iam_validation;
mod object_sizes;
mod strings;
mod timeouts;
mod transition_in_days;

use crate::iam_validation::{PermissionValidator, ValidationResponse};
use crate::object_sizes::ObjectSizes;
use crate::strings::{check_string_requirements, StringRequirements};
use crate::timeouts::Timeouts;
use crate::transition_in_days::TransitionInfo;
use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use std::env;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, LitInt, LitStr};
use crate::file_util::get_absolute_file_path;

/// Creates a validated `StringWithOnlyAlphaNumericsAndUnderscores` wrapper at compile time.
///
/// # Validation Rules
///
/// - String must not be empty
/// - Only alphanumeric characters, and underscores are allowed
/// - Underscores can appear in any position (beginning, middle, or end)
#[proc_macro]
pub fn string_with_only_alphanumerics_and_underscores(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    let requirements = StringRequirements::not_empty_allowed_chars(vec!['_']);

    match check_string_requirements(&value, output.span(), requirements) {
        None => quote!(
            StringWithOnlyAlphaNumericsAndUnderscores(#value.to_string())
        )
        .into(),
        Some(e) => e.into_compile_error().into(),
    }
}

/// Creates a validated `StringWithOnlyAlphaNumericsUnderscoresAndHyphens` wrapper at compile time.
/// 
/// # Validation Rules
///
/// - String must not be empty
/// - Only alphanumeric characters, underscores, and hyphens are allowed
#[proc_macro]
pub fn string_with_only_alphanumerics_underscores_and_hyphens(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    let requirements = StringRequirements::not_empty_allowed_chars(vec!['_', '-']);

    match check_string_requirements(&value, output.span(), requirements) {
        None => quote!(
            StringWithOnlyAlphaNumericsUnderscoresAndHyphens(#value.to_string())
        )
        .into(),
        Some(e) => e.into_compile_error().into(),
    }
}

/// Creates a validated `StringWithOnlyAlphaNumericsUnderscoresAndHyphens` wrapper at compile time.
///
/// This macro ensures that the input string contains only alphanumeric characters (a-z, A-Z, 0-9),
/// underscores (_), and hyphens (-). It's designed for creating safe identifiers for AWS resources
/// that allow hyphens in their naming conventions.
///
/// # Validation Rules
///
/// - String must not be empty
/// - Only alphanumeric characters, underscores, and hyphens are allowed
#[proc_macro]
pub fn string_with_only_alphanumerics_and_hyphens(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    let requirements = StringRequirements::not_empty_allowed_chars(vec!['-']);

    match check_string_requirements(&value, output.span(), requirements) {
        None => quote!(
            StringWithOnlyAlphaNumericsAndHyphens(#value.to_string())
        )
        .into(),
        Some(e) => e.into_compile_error().into(),
    }
}

/// Creates a validated `StringForSecret` wrapper for AWS Secrets Manager secret names at compile time.
///
/// This macro ensures that the input string is a valid name for AWS Secrets Manager secrets,
/// following AWS naming conventions and character restrictions.
///
/// # Validation Rules
///
/// - String must not be empty
/// - Only alphanumeric characters, and the following special characters are allowed: /, _, +, =, ., @, -
#[proc_macro]
pub fn string_for_secret(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    let requirements = StringRequirements::not_empty_allowed_chars(vec!['/', '_', '+', '=', '.', '@', '-']);

    match check_string_requirements(&value, output.span(), requirements) {
        None => quote!(
            StringForSecret(#value.to_string())
        )
        .into(),
        Some(e) => e.into_compile_error().into(),
    }
}

/// Creates a validated `EnvVarKey` wrapper for AWS Lambda environment variable keys at compile time.
///
/// This macro ensures that the input string is a valid environment variable key for AWS Lambda
/// functions, following AWS naming conventions and restrictions.
///
/// # Validation Rules
///
/// - Key must be at least 2 characters long
/// - Cannot start with an underscore (_)
/// - Only alphanumeric characters and underscores are allowed
#[proc_macro]
pub fn env_var_key(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.len() < 2 {
        return Error::new(output.span(), "env var key should be at least two characters long".to_string())
            .into_compile_error()
            .into();
    }

    if value.get(0..1).expect("just checked that length is at least 2") == "_" {
        return Error::new(output.span(), "env var key should not start with an underscore".to_string())
            .into_compile_error()
            .into();
    }

    if value.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Error::new(
            output.span(),
            "env var key should only contain alphanumeric characters and underscores".to_string(),
        )
        .into_compile_error()
        .into();
    }

    quote!(
        EnvVarKey(#value.to_string())
    )
    .into()
}

/// Creates a validated `ZipFile` wrapper for AWS Lambda deployment packages at compile time.
///
/// This macro ensures that the input string refers to a valid ZIP file that exists on the
/// filesystem at compile time.
///
/// # Validation Rules
///
/// - Path must end with `.zip` extension
/// - File must exist at compile time
/// - Path must be valid Unicode
/// - Both relative and absolute paths are allowed
///
/// # Note
///
/// This macro performs filesystem checks at compile time, so the ZIP file must exist
/// when the code is compiled. This ensures deployment packages are available before
/// attempting to deploy infrastructure.
#[proc_macro]
pub fn zip_file(input: TokenStream) -> TokenStream {
    let output: syn::Result<LitStr> = syn::parse(input);

    let output = match output {
        Ok(output) => output,
        Err(_) => {
            return Error::new(Span::call_site(), "zip_file macro should contain value".to_string())
                .into_compile_error()
                .into();
        }
    };

    let value = output.value();

    if !value.ends_with(".zip") {
        return Error::new(output.span(), format!("zip should end with `.zip` (found `{value}`)"))
            .into_compile_error()
            .into();
    }

    let value = match get_absolute_file_path(&value) {
        Ok(v) => v,
        Err(e) => {
            return Error::new(output.span(), e)
                .into_compile_error()
                .into();
        }
    };

    quote!(
        ZipFile(#value.to_string())
    )
    .into()
}

/// Creates a validated `TomlFile` wrapper.
///
/// # Validation Rules
///
/// - Path must end with `.toml` extension
/// - File must exist at compile time
/// - Path must be valid Unicode
/// - Both relative and absolute paths are allowed
#[proc_macro]
pub fn toml_file(input: TokenStream) -> TokenStream {
    let output: syn::Result<LitStr> = syn::parse(input);

    let output = match output {
        Ok(output) => output,
        Err(_) => {
            return Error::new(Span::call_site(), "toml_file macro should contain value".to_string())
                .into_compile_error()
                .into();
        }
    };

    let value = output.value();

    if !value.ends_with(".toml") {
        return Error::new(output.span(), format!("toml file should end with `.toml` (found `{value}`)"))
            .into_compile_error()
            .into();
    }

    let value = match get_absolute_file_path(&value) {
        Ok(v) => v,
        Err(e) => {
            return Error::new(output.span(), e)
                .into_compile_error()
                .into();
        }
    };

    quote!(
        TomlFile(#value.to_string())
    )
    .into()
}

/// Creates a validated `NonZeroNumber` wrapper for positive integers at compile time.
///
/// This macro ensures that the input number is greater than zero, preventing common
/// configuration errors where zero values would cause AWS resource creation to fail
/// or behave unexpectedly.
#[proc_macro]
pub fn non_zero_number(input: TokenStream) -> TokenStream {
    let output = match syn::parse::<LitInt>(input) {
        Ok(v) => v,
        Err(_) => {
            return Error::new(Span::call_site(), "value is not a valid number".to_string())
                .into_compile_error()
                .into();
        }
    };

    let as_number: syn::Result<u32> = output.base10_parse();

    let num = if let Ok(num) = as_number {
        if num == 0 {
            return Error::new(output.span(), "value should not be null".to_string())
                .into_compile_error()
                .into();
        }
        num
    } else {
        return Error::new(output.span(), "value is not a valid u32 number".to_string())
            .into_compile_error()
            .into();
    };

    quote!(
        NonZeroNumber(#num)
    )
    .into()
}

macro_rules! number_check {
    ($name:ident,$min:literal,$max:literal,$output:ident,$type:ty) => {
        #[doc = "Checks whether the value that will be wrapped in the "]
		#[doc = stringify!($output)]
		#[doc = "struct is between "]
		#[doc = stringify!($min)]
		#[doc = "and "]
        #[doc = stringify!($max)]
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            let output: LitInt = syn::parse(input).unwrap();

            let as_number: syn::Result<$type> = output.base10_parse();

            if let Ok(num) = as_number {
                if num < $min {
                    Error::new(output.span(), format!("value should be at least {}", $min)).into_compile_error().into()
                } else if num > $max {
                    Error::new(output.span(), format!("value should be at most {}", $max)).into_compile_error().into()
                } else {
                    quote!(
                        $output(#num)
                    ).into()
                }
            } else {
                Error::new(output.span(), "value is not a valid number".to_string()).into_compile_error().into()
            }
        }
    }
}

number_check!(memory, 128, 10240, Memory, u16);
number_check!(timeout, 1, 900, Timeout, u16);
number_check!(delay_seconds, 0, 900, DelaySeconds, u16);
number_check!(maximum_message_size, 1024, 1048576, MaximumMessageSize, u32);
number_check!(message_retention_period, 60, 1209600, MessageRetentionPeriod, u32);
number_check!(visibility_timeout, 0, 43200, VisibilityTimeout, u32);
number_check!(receive_message_wait_time, 0, 20, ReceiveMessageWaitTime, u16);
number_check!(sqs_event_source_max_concurrency, 2, 1000, SqsEventSourceMaxConcurrency, u16);
number_check!(connection_attempts, 1, 3, ConnectionAttempts, u16);
number_check!(s3_origin_read_timeout, 1, 120, ConnectionAttempts, u16);
number_check!(deployment_duration_in_minutes, 0, 1440, DeploymentDurationInMinutes, u16);
number_check!(growth_factor, 0, 100, GrowthFactor, u16);

const NO_REMOTE_OVERRIDE_ENV_VAR_NAME: &str = "RUSTY_CDK_NO_REMOTE";
const RUSTY_CDK_RECHECK_ENV_VAR_NAME: &str = "RUSTY_CDK_RECHECK";

/// Creates a validated `Bucket` wrapper for existing AWS S3 bucket references at compile time.
///
/// This macro ensures that the input string refers to an existing S3 bucket in your AWS account.
/// It queries S3 to verify the bucket exists.
///
/// # Validation Rules
///
/// - Value must not be an ARN (cannot start with "arn:")
/// - Value must not include the "s3:" prefix
/// - Bucket must exist in your AWS account (verified at compile time)
///
/// # Environment Variables
///
/// - `rusty_cdk_NO_REMOTE`: Set to `true` to skip remote AWS checks (for offline development)
/// - `rusty_cdk_RECHECK`: Set to `true` to force revalidation of cached bucket names
///
/// # Note
///
/// This macro caches validation results to improve compile times. The first compilation will
/// query AWS to verify the bucket exists. Later compilations will use the cached result unless `rusty_cdk_RECHECK` is set to true.
///
/// As usual, you can also avoid this verification by using the wrapper directly, but you lose all the above compile time guarantees by doing so.
#[proc_macro]
pub fn bucket(input: TokenStream) -> TokenStream {
    let input: LitStr = syn::parse(input).unwrap();
    let value = input.value();

    if value.starts_with("arn:") {
        return Error::new(input.span(), "value is an arn, not a bucket name".to_string())
            .into_compile_error()
            .into();
    }

    if value.starts_with("s3:") {
        return Error::new(input.span(), "value has s3 prefix, should be plain bucket name".to_string())
            .into_compile_error()
            .into();
    }

    let no_remote_check_wanted = env::var(NO_REMOTE_OVERRIDE_ENV_VAR_NAME)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(false);

    if no_remote_check_wanted {
        return bucket::bucket_output(value);
    }

    let rechecked_wanted = env::var(RUSTY_CDK_RECHECK_ENV_VAR_NAME)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(false);

    if !rechecked_wanted {
        match bucket::valid_bucket_according_to_file_storage(&value) {
            bucket::FileStorageOutput::Valid => {
                return bucket::bucket_output(value)
            }
            bucket::FileStorageOutput::Invalid => {
                return Error::new(input.span(), format!("(cached) did not find bucket with name `{value}` in your account. You can rerun this check by adding setting the `{RUSTY_CDK_RECHECK_ENV_VAR_NAME}` env var to true")).into_compile_error().into()
            }
            bucket::FileStorageOutput::Unknown => {}
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(bucket::find_bucket(input.clone())) {
        Ok(_) => {
            bucket::update_file_storage(bucket::FileStorageInput::Valid(&value));
            bucket::bucket_output(value)
        }
        Err(e) => {
            bucket::update_file_storage(bucket::FileStorageInput::Invalid(&value));
            e.into_compile_error().into()
        }
    }
}

const ADDITIONAL_ALLOWED_FOR_BUCKET_NAME: [char; 2] = ['.', '-'];

/// Creates a validated `BucketName` wrapper for new AWS S3 bucket names at compile time.
///
/// This macro ensures that the input string is a valid S3 bucket name that follows AWS naming
/// requirements and verifies the name is available at compile time.
///
/// # Validation Rules
///
/// - Must contain only lowercase letters, numbers, periods (.), and hyphens (-)
/// - No uppercase letters are allowed
/// - Bucket name must be globally unique and available (verified at compile time)
///
/// # Environment Variables
///
/// - `RUSTY_CDK_NO_REMOTE`: Set to `true` to skip remote AWS checks (for offline development)
/// - `RUSTY_CDK_RECHECK`: Set to `true` to force revalidation of cached bucket name availability
///
/// # Note
///
/// This macro caches validation results to improve compile times. The first compilation will
/// query AWS to verify the bucket name is available. Later compilations will use the cached
/// result unless `RUSTY_CDK_RECHECK` is set to true.
///
/// As usual, you can also avoid this verification by using the wrapper directly, but you lose all the above compile time guarantees by doing so.
#[proc_macro]
pub fn bucket_name(input: TokenStream) -> TokenStream {
    let input: LitStr = syn::parse(input).unwrap();
    let value = input.value();

    if value.chars().any(|c| c.is_uppercase()) {
        return Error::new(input.span(), "value contains uppercase letters".to_string())
            .into_compile_error()
            .into();
    }

    if value
        .chars()
        .any(|c| !c.is_alphanumeric() && !ADDITIONAL_ALLOWED_FOR_BUCKET_NAME.contains(&c))
    {
        return Error::new(
            input.span(),
            "value should contain only letters, numbers, periods and dashes".to_string(),
        )
        .into_compile_error()
        .into();
    }

    let no_remote_check_wanted = env::var(NO_REMOTE_OVERRIDE_ENV_VAR_NAME)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(false);

    if no_remote_check_wanted {
        return bucket_name::bucket_name_output(value);
    }

    let rechecked_wanted = env::var(RUSTY_CDK_RECHECK_ENV_VAR_NAME)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(false);

    if !rechecked_wanted {
        match bucket_name::valid_bucket_name_according_to_file_storage(&value) {
            bucket_name::FileStorageOutput::Valid => {
                return bucket_name::bucket_name_output(value)
            }
            bucket_name::FileStorageOutput::Invalid => {
                return Error::new(input.span(), format!("(cached) bucket name is already taken. You can rerun this check by adding setting the `{RUSTY_CDK_RECHECK_ENV_VAR_NAME}` env var to true")).into_compile_error().into()
            }
            bucket_name::FileStorageOutput::Unknown => {}
        }
    }

    match bucket_name::check_bucket_name(input) {
        Ok(_) => {
            bucket_name::update_file_storage(bucket_name::FileStorageInput::Valid(&value));
            bucket_name::bucket_name_output(value)
        }
        Err(e) => {
            bucket_name::update_file_storage(bucket_name::FileStorageInput::Invalid(&value));
            e.into_compile_error().into()
        }
    }
}

const POSSIBLE_LOG_RETENTION_VALUES: [u16; 22] = [
    1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, 3653,
];

/// Creates a validated `RetentionInDays` wrapper for AWS CloudWatch Logs retention periods at compile time.
///
/// This macro ensures that the input number is one of the valid retention periods allowed by AWS
/// CloudWatch Logs. AWS only allows specific discrete values for log retention periods.
///
/// # Validation Rules
///
/// - Value must be a number, and of the AWS-approved retention periods (in days)
#[proc_macro]
pub fn log_retention(input: TokenStream) -> TokenStream {
    let output = match syn::parse::<LitInt>(input) {
        Ok(v) => v,
        Err(_) => {
            return Error::new(Span::call_site(), "value is not a valid number".to_string())
                .into_compile_error()
                .into();
        }
    };

    let as_number: syn::Result<u16> = output.base10_parse();

    if let Ok(num) = as_number {
        if POSSIBLE_LOG_RETENTION_VALUES.contains(&num) {
            quote! {
                RetentionInDays(#num)
            }
            .into()
        } else {
            Error::new(output.span(), format!("value should be one of {:?}", POSSIBLE_LOG_RETENTION_VALUES))
                .into_compile_error()
                .into()
        }
    } else {
        Error::new(output.span(), "value is not a valid u16 number".to_string())
            .into_compile_error()
            .into()
    }
}

const ADDITIONAL_ALLOWED_FOR_LOG_GROUP: [char; 6] = ['.', '-', '_', '#', '/', '\\'];

/// Creates a validated `LogGroupName` wrapper for AWS CloudWatch Logs log group names at compile time.
///
/// This macro ensures that the input string is a valid CloudWatch Logs log group name following
/// AWS naming conventions and length restrictions.
///
/// # Validation Rules
///
/// - String must not be empty
/// - The maximum length is 512 characters
/// - Only alphanumeric characters, and the following special characters are allowed: . - _ # / \
#[proc_macro]
pub fn log_group_name(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.is_empty() {
        return Error::new(output.span(), "value should not be blank".to_string())
            .into_compile_error()
            .into();
    }

    if value.len() > 512 {
        return Error::new(output.span(), "value should not be longer than 512 chars".to_string())
            .into_compile_error()
            .into();
    }

    if value
        .chars()
        .any(|c| !c.is_alphanumeric() && !ADDITIONAL_ALLOWED_FOR_LOG_GROUP.contains(&c))
    {
        return Error::new(
            output.span(),
            format!(
                "value should only contain alphanumeric characters and {:?}",
                ADDITIONAL_ALLOWED_FOR_LOG_GROUP
            ),
        )
        .into_compile_error()
        .into();
    }

    quote!(
        LogGroupName(#value.to_string())
    )
    .into()
}

/// Creates a validated `IamAction` wrapper for AWS IAM permissions at compile time.
///
/// This macro ensures that the input string represents a valid AWS IAM action permission.
/// It validates the action against a comprehensive list of AWS service permissions to catch
/// typos and invalid permissions at compile time.
///
/// # Validation Rules
///
/// - String must not be empty
/// - Action must be a valid AWS IAM action (e.g., "s3:GetObject", "s3:Put*")
/// - Action is validated against AWS's official permission list
/// - Wildcards are supported
///
/// # Note
///
/// This macro helps prevent runtime IAM policy errors by validating permissions at compile time.
/// Invalid or misspelled actions will cause compilation to fail with a helpful error message.
#[proc_macro]
pub fn iam_action(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.is_empty() {
        return Error::new(output.span(), "value should not be blank".to_string())
            .into_compile_error()
            .into();
    }

    let validator = PermissionValidator::new();
    match validator.is_valid_action(&value) {
        ValidationResponse::Valid => quote!(
            IamAction(#value.to_string())
        )
        .into(),
        ValidationResponse::Invalid(message) => Error::new(output.span(), message).into_compile_error().into(),
    }
}

/// Creates a validated `S3LifecycleObjectSizes` wrapper for S3 lifecycle rule object size constraints at compile time.
///
/// This macro defines minimum and maximum object sizes for S3 lifecycle transitions, allowing
/// lifecycle rules to apply only to objects within a specific size range.
///
/// # Validation Rules
///
/// - Both minimum and maximum sizes are optional
/// - If both are provided, the minimum must be smaller than the maximum
/// - Values are specified in bytes
#[proc_macro]
pub fn lifecycle_object_sizes(input: TokenStream) -> TokenStream {
    let ObjectSizes { first, second } = parse_macro_input!(input);

    // replace with if let Some
    if first.is_some() && second.is_some() && first.unwrap() > second.unwrap() {
        return Error::new(
            Span::call_site(),
            format!(
                "first number ({}) in `lifecycle_object_sizes` should be smaller than second ({})",
                first.unwrap(),
                second.unwrap()
            ),
        )
        .into_compile_error()
        .into();
    }

    let first_output = if let Some(first) = first {
        quote! {
            Some(#first)
        }
    } else {
        quote! { None }
    };

    let second_output = if let Some(second) = second {
        quote! {
            Some(#second)
        }
    } else {
        quote! { None }
    };

    quote! {
        S3LifecycleObjectSizes(#first_output, #second_output)
    }
    .into()
}

/// Creates a validated `OriginPath` wrapper for CloudFront origin path prefixes at compile time.
///
/// This macro ensures that the path string follows CloudFront's requirements for origin paths,
/// which are appended to requests forwarded to the origin.
///
/// # Validation Rules
///
/// - Must start with a forward slash (/)
/// - Must NOT end with a forward slash (/)
/// - Example: "/production" is valid, but "/production/" and "production" are not
#[proc_macro]
pub fn origin_path(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if !value.starts_with("/") || value.ends_with("/") {
        return Error::new(
            value.span(),
            format!("origin path should start with a / and should not end with / (but got {})", value),
        )
        .into_compile_error()
        .into();
    }

    quote! {
        OriginPath(#value)
    }
    .into()
}

/// Creates a validated `DefaultRootObject` wrapper for CloudFront default root objects at compile time.
///
/// This macro ensures that the object name follows CloudFront's requirements for default root
/// objects, which are returned when viewers request the root URL of a distribution.
///
/// # Validation Rules
///
/// - Must NOT start with a forward slash (/)
/// - Must NOT end with a forward slash (/)
/// - Example: "index.html" is valid, but "/index.html" and "index.html/" are not
#[proc_macro]
pub fn default_root_object(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.starts_with("/") || value.ends_with("/") {
        return Error::new(value.span(), "default root object should not start with /".to_string())
            .into_compile_error()
            .into();
    }

    quote! {
        DefaultRootObject(#value)
    }
    .into()
}

/// Creates a validated `CfConnectionTimeout` wrapper for CloudFront origin connection timeouts at compile time.
///
/// This macro configures timeout settings for CloudFront when connecting to origins, allowing specification of both connection timeout and response completion timeout.
///
/// # Validation Rules
///
/// - Connection timeout (first value) must be between 1 and 10 seconds (if provided)
/// - Response completion timeout (second value) must be greater than or equal to connection timeout (if both provided)
/// - Both values are optional
#[proc_macro]
pub fn cf_connection_timeout(input: TokenStream) -> TokenStream {
    let Timeouts { first, second } = parse_macro_input!(input);

    if let Some(first) = first {
        if first > 10 {
            return Error::new(
                Span::call_site(),
                format!("connection timeout was {} but should be between 1 and 10", first),
            )
            .into_compile_error()
            .into();
        } else if let Some(second) = second {
            if second < first {
                return Error::new(
                    Span::call_site(),
                    format!(
                        "response completion timeout was {} but should be larger than connection timeout ({})",
                        second, first
                    ),
                )
                .into_compile_error()
                .into();
            }
        }
    }

    let first_output = if let Some(first) = first {
        quote! {
            Some(#first)
        }
    } else {
        quote! { None }
    };

    let second_output = if let Some(second) = second {
        quote! {
            Some(#second)
        }
    } else {
        quote! { None }
    };

    quote! {
        CfConnectionTimeout(#first_output, #second_output)
    }
    .into()
}

/// Creates a validated `LambdaPermissionAction` wrapper for Lambda resource-based policy actions at compile time.
///
/// This macro ensures that the action string is properly formatted for Lambda resource-based
/// policies, which control what AWS services and accounts can invoke Lambda functions.
///
/// # Validation Rules
///
/// - String must not be empty
/// - Must start with "lambda:" prefix
/// - Common values include "lambda:InvokeFunction" and "lambda:GetFunction"
#[proc_macro]
pub fn lambda_permission_action(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    let requirements = StringRequirements::not_empty_prefix("lambda");

    match check_string_requirements(&value, output.span(), requirements) {
        None => quote!(
            LambdaPermissionAction(#value.to_string())
        )
        .into(),
        Some(e) => e.into_compile_error().into(),
    }
}

/// Creates a validated `AppConfigName` wrapper for AWS AppConfig resource names at compile time.
///
/// This macro ensures that the name string follows AWS AppConfig naming conventions and
/// length restrictions for applications, environments, and configuration profiles.
///
/// # Validation Rules
///
/// - String must not be empty
/// - Maximum length of 64 characters
/// - Used for AppConfig application names, environment names, and configuration profile names
#[proc_macro]
pub fn app_config_name(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.is_empty() || value.len() > 64 {
        return Error::new(
            Span::call_site(),
            "app config name should be between 1 and 64 chars in length".to_string(),
        )
        .into_compile_error()
        .into();
    }

    quote! {
        AppConfigName(#value.to_string())
    }
    .into()
}

const LIFECYCLE_STORAGE_TYPES: [&str; 6] = [
    "IntelligentTiering",
    "OneZoneIA",
    "StandardIA",
    "GlacierDeepArchive",
    "Glacier",
    "GlacierInstantRetrieval",
];
const LIFECYCLE_STORAGE_TYPES_MORE_THAN_THIRTY_DAYS: [&str; 2] = [
    "OneZoneIA",
    "StandardIA",
];

/// Creates a validated `LifecycleTransitionInDays` wrapper for S3 lifecycle transition rules at compile time.
///
/// This macro configures S3 lifecycle rules to automatically transition objects to different
/// storage classes after a specified number of days, with validation based on the target storage class.
///
/// # Validation Rules
///
/// - Days must be a positive number
/// - Storage class must be one of: IntelligentTiering, OneZoneIA, StandardIA, GlacierDeepArchive, Glacier, GlacierInstantRetrieval
/// - OneZoneIA and StandardIA storage classes require at least 30 days (cannot transition sooner)
#[proc_macro]
pub fn lifecycle_transition_in_days(input: TokenStream) -> TokenStream {
    let TransitionInfo { days, service } = parse_macro_input!(input);
    let service = service.trim();

    if !LIFECYCLE_STORAGE_TYPES.contains(&service) {
        return Error::new(Span::call_site(), format!("service should be one of {} (was {})", LIFECYCLE_STORAGE_TYPES.join(","), service))
            .into_compile_error()
            .into();
    } else if LIFECYCLE_STORAGE_TYPES_MORE_THAN_THIRTY_DAYS.contains(&service) && days <= 30 {
        return Error::new(Span::call_site(), format!("service of type {} cannot have transition under 30 days", LIFECYCLE_STORAGE_TYPES_MORE_THAN_THIRTY_DAYS.join(" or ")))
            .into_compile_error()
            .into();
    }

    quote! {
        LifecycleTransitionInDays(#days)
    }
    .into()
}

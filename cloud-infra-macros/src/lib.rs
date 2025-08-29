#![allow(unused_comparisons)]
//! # Cloud Infrastructure Macros
//!
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
//! You should import those from the cloud_infra::wrappers directory, as seen in the below example.
//!
//! ## Usage Examples
//!
//! ```rust,compile_fail
//! use cloud_infra::wrappers::Memory; // import the wrapper
//! use cloud_infra::memory;
//!
//! // Lambda memory configuration with validated limit
//! let mem = memory!(512);        // 512 MB (128-10240 range)
//! ```
//!
//! ## Available Macros
//!
//! ### String and Identifier Macros
//!
//! - [`string_with_only_alpha_numerics_and_underscores!`] - AWS resource identifiers
//! - [`env_var_key!`] - Lambda environment variable keys
//!
//! ### File Path Macros
//!
//! - [`zipfile!`] - ZIP file paths for Lambda deployment packages
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

use proc_macro::{TokenStream};
use std::env;
use quote::{quote};
use std::path::{absolute, Path};
use syn::{Error, LitInt, LitStr};
use crate::bucket::{bucket_output, find_bucket, update_file_storage, valid_bucket_according_to_file_storage, FileStorageInput, FileStorageOutput};

/// Creates a validated `StringWithOnlyAlphaNumericsAndUnderscores` wrapper at compile time.
///
/// This macro ensures that the input string contains only alphanumeric characters (a-z, A-Z, 0-9)
/// and underscores (_). It's designed for creating safe identifiers for AWS resources that have
/// naming restrictions.
///
/// # Validation Rules
///
/// - String must not be empty
/// - Only alphanumeric characters and underscores are allowed
/// - Underscores can appear in any position (beginning, middle, or end)
#[proc_macro]
pub fn string_with_only_alpha_numerics_and_underscores(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.is_empty() {
        return Error::new(output.span(), "value should not be blank".to_string()).into_compile_error().into()
    }
    
    if value.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Error::new(output.span(), "value should only contain alphanumeric characters and underscores".to_string()).into_compile_error().into()
    }
    
    quote!(
        StringWithOnlyAlphaNumericsAndUnderscores(#value.to_string())
    ).into()
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
        return Error::new(output.span(), "env var key should be at least two characters long".to_string()).into_compile_error().into()
    }

    if value.get(0..1).expect("just checked that length is at least 2") == "_" {
        return Error::new(output.span(), "env var key should not start with an underscore".to_string()).into_compile_error().into()
    }

    if value.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Error::new(output.span(), "env var key should only contain alphanumeric characters and underscores".to_string()).into_compile_error().into()
    }

    quote!(
        EnvVarKey(#value.to_string())
    ).into()
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
pub fn zipfile(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if !value.ends_with(".zip") {
        return Error::new(output.span(), format!("zip file should end with `.zip`, instead found `{value}`")).into_compile_error().into()
    }

    let path = Path::new(&value);

    if !path.exists() {
        return Error::new(output.span(), format!("did not find file `{value}`")).into_compile_error().into()
    }

    let value = if path.is_relative() {
        match absolute(path) {
            Ok(absolute_path) => absolute_path.to_str().expect("zip file path to be valid unicode").to_string(),
            Err(e) => {
                return Error::new(output.span(), format!("failed to convert zip file path to absolute path: {e}")).into_compile_error().into()
            }
        }
    } else {
        path.to_str().expect("zip file path to be valid unicode").to_string()
    };

    quote!(
        ZipFile(#value.to_string())
    ).into()
}

/// Creates a validated `NonZeroNumber` wrapper for positive integers at compile time.
///
/// This macro ensures that the input number is greater than zero, preventing common
/// configuration errors where zero values would cause AWS resource creation to fail
/// or behave unexpectedly.
#[proc_macro]
pub fn non_zero_number(input: TokenStream) -> TokenStream {
    let output: LitInt = syn::parse(input).unwrap();

    let as_number: syn::Result<u32> = output.base10_parse();

    let num = if let Ok(num) = as_number {
        if num == 0 {
            return Error::new(output.span(), "value should not be null".to_string()).into_compile_error().into()
        }
        num
    } else {
        return Error::new(output.span(), "value is not a valid number".to_string()).into_compile_error().into()
    };

    quote!(
        NonZeroNumber(#num)
    ).into()
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


const NO_REMOTE_OVERRIDE_ENV_VAR_NAME: &'static str = "CLOUD_INFRA_NO_REMOTE";
const CLOUD_INFRA_RECHECK_ENV_VAR_NAME: &'static str = "CLOUD_INFRA_RECHECK";

// TODO documentation
#[proc_macro]
pub fn bucket(input: TokenStream) -> TokenStream {
    let input: LitStr = syn::parse(input).unwrap();
    let value = input.value();

    if value.starts_with("arn:") {
        return Error::new(input.span(), "expected bucket name, not arn, as input".to_string()).into_compile_error().into()
    }

    if value.starts_with("s3:") {
        return Error::new(input.span(), "expected plain bucket name, remove the s3 prefix".to_string()).into_compile_error().into()
    }

    let no_remote_check_wanted = env::var(NO_REMOTE_OVERRIDE_ENV_VAR_NAME).ok().and_then(|v| v.parse().ok()).unwrap_or(false);

    if no_remote_check_wanted {
        return bucket_output(value)
    }

    let rechecked_wanted = env::var(CLOUD_INFRA_RECHECK_ENV_VAR_NAME).ok().and_then(|v| v.parse().ok()).unwrap_or(false);

    if !rechecked_wanted {
        match valid_bucket_according_to_file_storage(&value) {
            FileStorageOutput::Valid => {
                return bucket_output(value)
            }
            FileStorageOutput::Invalid => {
                return Error::new(input.span(), format!("(cached) did not find bucket with name `{value}`. You can rerun the check by adding setting the `{CLOUD_INFRA_RECHECK_ENV_VAR_NAME}` env var to true")).into_compile_error().into()
            }
            FileStorageOutput::Unknown => {}
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(find_bucket(input.clone(), &value)) {
        Ok(_) => {
            update_file_storage(FileStorageInput::Valid(&value));
            bucket_output(value)
        }
        Err(e) => {
            update_file_storage(FileStorageInput::Invalid(&value));
            e.into_compile_error().into()
        }
    }
}

const VALUES: [u16;22] = [1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, 2192, 2557, 2922, 3288, 3653];

#[proc_macro]
pub fn log_retention(input: TokenStream) -> TokenStream {
    let output: LitInt = syn::parse(input).unwrap();

    let as_number: syn::Result<u16> = output.base10_parse();

    if let Ok(num) = as_number {
        if VALUES.contains(&num) {
            quote! {
                RetentionInDays(#num)
            }.into()
        } else {
            Error::new(output.span(), format!("value should be one of {:?}", VALUES)).into_compile_error().into()    
        }
    } else {
        Error::new(output.span(), "value is not a valid number".to_string()).into_compile_error().into()
    }
}

const ADDITIONAL_ALLOWED: [char; 6] = ['.', '-', '_', '#', '/', '\\'];

#[proc_macro]
pub fn log_group_name(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.is_empty() {
        return Error::new(output.span(), "value should not be blank".to_string()).into_compile_error().into()
    }
    
    if value.len() > 512 {
        return Error::new(output.span(), "value should not be longer than 512 chars".to_string()).into_compile_error().into()
    }

    if value.chars().any(|c| !c.is_alphanumeric() && !ADDITIONAL_ALLOWED.contains(&c)) {
        return Error::new(output.span(), format!("value should only contain alphanumeric characters and {:?}", ADDITIONAL_ALLOWED)).into_compile_error().into()
    }
    
    quote!(
        LogGroupName(#value.to_string())
    ).into()
}

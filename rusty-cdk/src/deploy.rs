use crate::util::{get_existing_template, get_stack_status, load_config};
use aws_config::SdkConfig;
use aws_sdk_cloudformation::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_cloudformation::types::{Capability, StackStatus, Tag};
use aws_sdk_cloudformation::Client;
use rusty_cdk_core::stack::{Asset, Stack};
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub enum DeployError {
    SynthError(String),
    StackCreateError(String),
    StackUpdateError(String),
    AssetError(String),
    UnknownError(String),
}

impl Error for DeployError {}

impl Display for DeployError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeployError::SynthError(_) => f.write_str("unable to synth"),
            DeployError::StackCreateError(_) => f.write_str("unable to create stack"),
            DeployError::StackUpdateError(_) => f.write_str("unable to update stack"),
            DeployError::AssetError(_) => f.write_str("unable to handle asset"),
            DeployError::UnknownError(_) => f.write_str("unknown error"),
        }
    }
}

/// Deploys a stack to AWS using CloudFormation.
///
/// This function handles the complete deployment lifecycle:
/// - Uploading Lambda function assets to S3
/// - Creating or updating the CloudFormation stack
/// - Monitoring deployment progress with real-time status updates
///
/// It exits with code 0 on success, 1 on failure
/// 
/// For a deployment method that returns a Result, see `deploy_with_result`
///
/// # Parameters
///
/// * `name` - The CloudFormation stack name (alphanumeric characters and hyphens only)
/// * `stack` - The stack to deploy, created using `StackBuilder`
///
/// # Tags
///
/// If tags were added to the stack using `StackBuilder::add_tag()`, they will be
/// applied to the CloudFormation stack and propagated to resources where supported.
///
/// # Example
///
/// ```no_run
/// use rusty_cdk::deploy;
/// use rusty_cdk::stack::StackBuilder;
/// use rusty_cdk::sqs::QueueBuilder;
/// use rusty_cdk_macros::string_with_only_alphanumerics_and_hyphens;
/// use rusty_cdk::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
///
/// #[tokio::main]
/// async fn main() {
///
/// let mut stack_builder = StackBuilder::new();
///     QueueBuilder::new("my-queue")
///         .standard_queue()
///         .build(&mut stack_builder);
///
///     let stack = stack_builder.build().expect("Stack to build successfully");
///
///     deploy(string_with_only_alphanumerics_and_hyphens!("my-application-stack"), stack).await;
/// }
/// ```
///
/// # AWS Credentials
///
/// This function requires valid AWS credentials configured through:
/// - Environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
/// - AWS credentials file (`~/.aws/credentials`)
/// - IAM role (when running on EC2, ECS, Lambda, etc.)
/// - ...
///
/// The AWS credentials must have permissions for:
/// - `cloudformation:CreateStack`, `cloudformation:UpdateStack`, `cloudformation:DescribeStacks`, `cloudformation:GetTemplate`
/// - `s3:PutObject` (for Lambda asset uploads)
/// - IAM permissions if creating roles (`iam:CreateRole`, `iam:PutRolePolicy`, etc.)
/// - Service-specific permissions for resources being created
pub async fn deploy(name: StringWithOnlyAlphaNumericsAndHyphens, stack: Stack) {
    match deploy_with_result(name, stack, true).await {
        Ok(message) => println!("{message}"),
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}

/// Deploys a stack to AWS using CloudFormation.
///
/// This function handles the complete deployment lifecycle:
/// - Uploading Lambda function assets to S3
/// - Creating or updating the CloudFormation stack
/// - Monitoring deployment progress
///
/// It returns a `Result`. In case of error, a `DeployError` is returned.
///
/// For a deployment method that shows updates and exits on failure, see `deploy`
///
/// # Parameters
///
/// * `name` - The CloudFormation stack name (alphanumeric characters and hyphens only)
/// * `stack` - The stack to deploy, created using `StackBuilder`
///
/// # Tags
///
/// If tags were added to the stack using `StackBuilder::add_tag()`, they will be
/// applied to the CloudFormation stack and propagated to resources where supported.
///
/// # Example
///
/// ```no_run
/// use rusty_cdk::deploy;
/// use rusty_cdk::stack::StackBuilder;
/// use rusty_cdk::sqs::QueueBuilder;
/// use rusty_cdk_macros::string_with_only_alphanumerics_and_hyphens;
/// use rusty_cdk::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
///
/// #[tokio::main]
/// async fn main() {
///
/// use rusty_cdk::deploy_with_result;
/// let mut stack_builder = StackBuilder::new();
///     QueueBuilder::new("my-queue")
///         .standard_queue()
///         .build(&mut stack_builder);
///
///     let stack = stack_builder.build().expect("Stack to build successfully");
///
///     let result = deploy_with_result(string_with_only_alphanumerics_and_hyphens!("my-application-stack"), stack, false).await;
/// }
/// ```
///
/// # AWS Credentials
///
/// This function requires valid AWS credentials configured through:
/// - Environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
/// - AWS credentials file (`~/.aws/credentials`)
/// - IAM role (when running on EC2, ECS, Lambda, etc.)
/// - ...
///
/// The AWS credentials must have permissions for:
/// - `cloudformation:CreateStack`, `cloudformation:UpdateStack`, `cloudformation:DescribeStacks`, `cloudformation:GetTemplate`
/// - `s3:PutObject` (for Lambda asset uploads)
/// - IAM permissions if creating roles (`iam:CreateRole`, `iam:PutRolePolicy`, etc.)
/// - Service-specific permissions for resources being created
pub async fn deploy_with_result(name: StringWithOnlyAlphaNumericsAndHyphens, mut stack: Stack, print_progress: bool) -> Result<String, DeployError> {
    let name = name.0;
    let config = load_config(true).await;

    upload_assets(stack.get_assets(), &config).await?;

    let cloudformation_client = Client::new(&config);

    create_or_update_stack(&name, &mut stack, &cloudformation_client).await?;

    loop {
        let status = get_stack_status(&name, &cloudformation_client).await.expect("status to be available for stack");

        match status {
            StackStatus::CreateComplete => {
                return Ok("created successfully".to_string());
            }
            StackStatus::UpdateComplete | StackStatus::UpdateCompleteCleanupInProgress => {
                return Ok("updated successfully".to_string());
            }
            StackStatus::CreateInProgress => {
                if print_progress {
                    println!("creating...");
                }
            }
            StackStatus::UpdateInProgress => {
                if print_progress {
                    println!("updating...");
                }
            }
            StackStatus::CreateFailed => {
                return Err(DeployError::StackCreateError(format!("{status}")));
            }
            StackStatus::UpdateRollbackComplete
            | StackStatus::UpdateRollbackCompleteCleanupInProgress
            | StackStatus::UpdateRollbackFailed
            | StackStatus::UpdateRollbackInProgress
            | StackStatus::UpdateFailed => {
                return Err(DeployError::StackUpdateError(format!("{status}")));
            }
            _ => {
                return Err(DeployError::UnknownError(format!("{status}")));
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}

async fn create_or_update_stack(name: &String, stack: &mut Stack, cloudformation_client: &Client) -> Result<(), DeployError> {
    let existing_template = get_existing_template(cloudformation_client, name).await;
    let tags = stack.get_tags();
    let tags = if tags.is_empty() {
        None
    } else {
        Some(tags.into_iter().map(|v| Tag::builder().key(v.0).value(v.1).build()).collect())
    };

    match existing_template {
        Some(existing) => {
            let body = stack
                .synth_for_existing(&existing)
                .map_err(|e| DeployError::SynthError(format!("{e:?}")))?;

            return match cloudformation_client
                .update_stack()
                .stack_name(name)
                .template_body(body)
                .capabilities(Capability::CapabilityNamedIam)
                .set_tags(tags)
                .send()
                .await {
                Ok(_) => Ok(()),
                Err(e) => {
                    match e {
                        SdkError::ServiceError(ref s) => {
                            let update_stack_error = s.err();
                            if update_stack_error.message().map(|v| v.contains("No updates are to be performed")).unwrap_or(false) {
                                Ok(())   
                            } else {
                                Err(DeployError::StackUpdateError(format!("{e:?}")))
                            }
                        }
                        _ => {
                            Err(DeployError::StackUpdateError(format!("{e:?}")))
                        }
                    }
                }
            }
        }
        None => {
            let body = stack.synth().map_err(|e| DeployError::SynthError(format!("{e:?}")))?;

            cloudformation_client
                .create_stack()
                .stack_name(name)
                .template_body(body)
                .capabilities(Capability::CapabilityNamedIam)
                .set_tags(tags)
                .send()
                .await
                .map_err(|e| DeployError::StackCreateError(format!("{e:?}")))?;
        }
    }
    Ok(())
}

async fn upload_assets(assets: Vec<Asset>, config: &SdkConfig) -> Result<(), DeployError> {
    let s3_client = Arc::new(aws_sdk_s3::Client::new(config));

    let tasks: Vec<_> = assets
        .into_iter()
        .map(|a| {
            let s3_client = s3_client.clone();
            tokio::spawn(async move {
                let body = aws_sdk_s3::primitives::ByteStream::from_path(a.path).await;
                s3_client
                    .put_object()
                    .bucket(a.s3_bucket)
                    .key(a.s3_key)
                    .body(body.unwrap())
                    .send()
                    .await
                    .unwrap();
            })
        })
        .collect();

    for task in tasks {
        task.await.map_err(|e| DeployError::AssetError(format!("{e:?}")))?;
    }
    Ok(())
}

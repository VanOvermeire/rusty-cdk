use crate::util::{get_existing_template, get_stack_status, load_config};
use aws_sdk_cloudformation::types::StackStatus;
use aws_sdk_cloudformation::Client;
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::exit;
use std::time::Duration;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use tokio::time::sleep;
use rusty_cdk_core::stack::{Cleanable, Stack};

#[derive(Debug)]
pub enum DestroyError {
    EmptyError(String),
    StackDeleteError(String),
    UnknownStack(String),
    UnknownError(String),
}

impl Error for DestroyError {}

impl Display for DestroyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DestroyError::EmptyError(_) => f.write_str("could not empty bucket"),
            DestroyError::StackDeleteError(_) => f.write_str("unable to delete stack"),
            DestroyError::UnknownStack(_) => f.write_str("stack could not be found"),
            DestroyError::UnknownError(_) => f.write_str("unknown error"),
        }
    }
}

/// Destroy a deployed stack
pub async fn destroy(name: StringWithOnlyAlphaNumericsAndHyphens) {
    match destroy_with_result(name, true).await {
        Ok(()) => println!("destroy completed successfully!"),
        Err(e) => {
            eprintln!("{:?}", e);
            exit(1);
        }
    }
}

/// Destroy a deployed stack
///
/// It returns a `Result`. In case of error, a `DestroyError` is returned.
pub async fn destroy_with_result(name: StringWithOnlyAlphaNumericsAndHyphens, print_progress: bool) -> Result<(), DestroyError> {
    let name = name.0;
    let config = load_config(false).await;
    let cloudformation_client = Client::new(&config);

    destroy_stack(&name, &cloudformation_client).await?;

    loop {
        let status = get_stack_status(&name, &cloudformation_client).await;

        if let Some(status) = status {
            match status {
                StackStatus::DeleteComplete => return Ok(()),
                StackStatus::DeleteInProgress => {
                    if print_progress {
                        println!("destroying...");
                    }
                }
                StackStatus::DeleteFailed => {
                    return Err(DestroyError::StackDeleteError(format!("{status}")));
                }
                _ => {
                    return Err(DestroyError::UnknownError(format!("{status}")));
                }
            }
        } else {
            // no status, so stack should be gone
            return Ok(());
        }

        sleep(Duration::from_secs(10)).await;
    }
}

async fn destroy_stack(name: &String, cloudformation_client: &Client) -> Result<(), DestroyError> {
    let delete_result = cloudformation_client.delete_stack().stack_name(name).send().await;
    match delete_result {
        Ok(_) => Ok(()),
        Err(e) => Err(DestroyError::StackDeleteError(e.to_string())),
    }
}

pub async fn clean(name: StringWithOnlyAlphaNumericsAndHyphens, print_progress: bool) -> Result<(), DestroyError> {
    let config = load_config(false).await;
    let cloudformation_client = Client::new(&config);

    let stack = get_existing_template(&cloudformation_client, &name.0).await.ok_or_else(|| {
        DestroyError::UnknownStack(format!("could not retrieve stack with name {}", &name.0))
    })?;
    
    let stack: Stack = serde_json::from_str(&stack).expect("to transform template into stack");
 
    for resource in stack.get_cleanable_resources() {
        match resource {
            Cleanable::Bucket(id) => {
                let bucket_info = cloudformation_client.describe_stack_resource()
                    .stack_name(&name.0)
                    .logical_resource_id(id)
                    .send()
                    .await
                    .expect("to find resource that's mentioned in the template");
                let physical_id = bucket_info.stack_resource_detail.expect("stack resource output to have detail").physical_resource_id.expect("physical id to be present");
    
                if print_progress {
                    println!("found bucket {id} (name {physical_id}) that will be deleted - emptying")
                }
                empty_bucket(physical_id).await?
            }
            Cleanable::Topic(id) => {
                let bucket_info = cloudformation_client.describe_stack_resource()
                    .stack_name(&name.0)
                    .logical_resource_id(id)
                    .send()
                    .await
                    .expect("to find resource that's mentioned in the template");
                let physical_id = bucket_info.stack_resource_detail.expect("stack resource output to have detail").physical_resource_id.expect("physical id to be present");
    
                if print_progress {
                    println!("found topic {id} (arn {physical_id}) that will be deleted - remove archival policy")
                }
                remove_archive_policy(physical_id).await?;
            }
        }
    }

    Ok(())
}

async fn remove_archive_policy(arn: String) -> Result<(), DestroyError> {
    let config = load_config(false).await;
    let client = aws_sdk_sns::Client::new(&config);

    client.set_topic_attributes()
        .topic_arn(arn)
        .attribute_name("ArchivePolicy")
        .attribute_value("{}")
        .send()
        .await
        .map_err(|e| DestroyError::EmptyError(format!("could not remove archive policy from topic: {:?}", e)))?;

    Ok(())
}

async fn empty_bucket(name: String) -> Result<(), DestroyError> {
    let config = load_config(false).await;
    let client = aws_sdk_s3::Client::new(&config);

    let mut marker_response = delete_objects(&client, &name, None).await?;

    while let Some(marker) = marker_response {
        println!("more cleanup required for {name}...");
        marker_response = delete_objects(&client, &name, Some(marker)).await?;
    }

    Ok(())
}

async fn delete_objects(client: &aws_sdk_s3::Client, name: &str, marker: Option<String>) -> Result<Option<String>, DestroyError> {
    let mut builder = client.list_objects()
        .bucket(name);

    if let Some(marker) = marker {
        builder = builder.marker(marker);
    }

    let objects = builder.send()
        .await
        .map_err(|e| DestroyError::EmptyError(format!("could not list objects to delete: {:?}", e)))?;

    let marker = objects.marker;

    if let Some(content) = objects.contents {
        let objects_to_delete = content.into_iter()
            .map(|v| ObjectIdentifier::builder().key(v.key.expect("object to have a key")).build().expect("building object identifier to succeed"))
            .collect();

        let to_delete = Delete::builder().set_objects(Some(objects_to_delete)).build().expect("building delete object to succeed");
        client.delete_objects()
            .bucket(name)
            .delete(to_delete)
            .send()
            .await
            .map_err(|e| {
                DestroyError::EmptyError(format!("could not delete objects: {:?}", e))
            })?;

        Ok(marker)
    } else {
        Ok(None)
    }
}
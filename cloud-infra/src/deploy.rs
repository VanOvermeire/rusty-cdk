use std::collections::HashMap;
use std::process::exit;
use aws_sdk_cloudformation::types::{Capability, StackStatus};
use std::sync::Arc;
use std::time::Duration;
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use serde::{Deserialize};
use tokio::time::sleep;
use cloud_infra_core::stack::Stack;

#[derive(Deserialize)]
struct StackOnlyMetadata {
    #[serde(rename = "Metadata")]
    pub (crate) metadata: HashMap<String, String>
}

async fn get_existing_template(client: &aws_sdk_cloudformation::Client, stack_name: &str) -> Option<String> {
    match client.describe_stacks().stack_name(stack_name).send().await {
        Ok(_) => {
            let template = client.get_template()
                .stack_name(stack_name)
                .send()
                .await;
            template.unwrap().template_body
        }
        Err(_) => {
            None
        }
    }
}

pub async fn deploy(name: &str, mut stack: Stack) {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        // https://github.com/awslabs/aws-sdk-rust/issues/1146
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .load()
        .await;
    let s3_client = Arc::new(aws_sdk_s3::Client::new(&config));

    let tasks: Vec<_> = stack
        .get_assets()
        .into_iter()
        .map(|a| {
            println!("uploading asset {} to {}/{}", a.path, a.s3_bucket, a.s3_key);
            let s3_client = s3_client.clone();
            tokio::spawn(async move {
                let body = aws_sdk_s3::primitives::ByteStream::from_path(a.path).await;
                s3_client
                    .put_object()
                    .bucket(a.s3_bucket)
                    .key(a.s3_key)
                    .body(body.unwrap()) //
                    .send()
                    .await
                    .unwrap();
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }

    let cloudformation_client = aws_sdk_cloudformation::Client::new(&config);

    let existing_template = get_existing_template(&cloudformation_client, name).await;

    match existing_template {
        Some(existing) => {
            let meta: StackOnlyMetadata = serde_json::from_str(existing.as_str()).expect("an existing stack should have our 'id' metadata");
            stack.update_resource_ids_for_existing_stack(meta.metadata);
            let body = get_template_or_exit(&stack);

            match cloudformation_client
                .update_stack()
                .stack_name(name)
                .template_body(body)
                .capabilities(Capability::CapabilityNamedIam)
                .send()
                .await
            {
                Ok(_) => println!("stack {name} update started"),
                Err(e) => eprintln!("an error occurred while creating the stack: {e:#?}"),
            }
        }
        None => {
            let body = get_template_or_exit(&stack);

            match cloudformation_client
                .create_stack()
                .stack_name(name)
                .template_body(body)
                .capabilities(Capability::CapabilityNamedIam)
                .send()
                .await
            {
                Ok(_) => println!("stack {name} creation started"),
                Err(e) => {
                    eprintln!("an error occurred while creating the stack: {e:#?}");
                    exit(1);
                },
            }
        }
    }

    loop {
        let status = cloudformation_client.describe_stacks().stack_name(name).send().await;
        let mut stacks = status
            .expect("to get a describe stacks result")
            .stacks
            .expect("to have a list of stacks");
        let first_stack = stacks.get_mut(0).expect("to find our stack");
        let status = first_stack.stack_status.take().expect("stack to have status");

        match status {
            StackStatus::CreateComplete => {
                println!("creation completed successfully!");
                exit(0);
            }
            StackStatus::CreateFailed => {
                println!("creation failed");
                exit(1);
            }
            StackStatus::CreateInProgress => {
                println!("creating...");
            }
            StackStatus::UpdateComplete | StackStatus::UpdateCompleteCleanupInProgress => {
                println!("update completed successfully!");
                exit(0);
            }
            StackStatus::UpdateRollbackComplete
            | StackStatus::UpdateRollbackCompleteCleanupInProgress
            | StackStatus::UpdateRollbackFailed
            | StackStatus::UpdateRollbackInProgress
            | StackStatus::UpdateFailed => {
                println!("update failed");
                exit(1);
            }
            StackStatus::UpdateInProgress => {
                println!("updating...");
            }
            _ => {
                println!("encountered unexpected cloudformation status: {status}");
                exit(1);
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}

fn get_template_or_exit(stack: &Stack) -> String {
    match stack.synth() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{e:#?}");
            exit(1);
        }
    }
}

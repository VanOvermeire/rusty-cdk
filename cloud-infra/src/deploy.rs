use crate::synth::Synth;
use aws_sdk_cloudformation::types::{Capability, StackStatus};
use std::sync::Arc;
use std::time::Duration;
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use tokio::time::sleep;

pub async fn deploy(name: &str, synth: Synth) {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        // https://github.com/awslabs/aws-sdk-rust/issues/1146
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .load()
        .await;
    let s3_client = Arc::new(aws_sdk_s3::Client::new(&config));

    let tasks: Vec<_> = synth
        .1
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

    match cloudformation_client.describe_stacks().stack_name(name).send().await {
        Ok(_) => {
            match cloudformation_client
                .update_stack()
                .stack_name(name)
                .template_body(synth.0)
                .capabilities(Capability::CapabilityNamedIam)
                .send()
                .await
            {
                Ok(_) => println!("stack {name} update started"),
                Err(e) => eprintln!("an error occurred while creating the stack: {e:?}"),
            }
        }
        Err(_) => {
            match cloudformation_client
                .create_stack()
                .stack_name(name)
                .template_body(synth.0)
                .capabilities(Capability::CapabilityNamedIam)
                .send()
                .await
            {
                Ok(_) => println!("stack {name} creation started"),
                Err(e) => eprintln!("an error occurred while creating the stack: {e:?}"),
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
                break;
            }
            StackStatus::CreateFailed => {
                println!("creation failed");
                break;
            }
            StackStatus::CreateInProgress => {
                println!("creating...");
            }
            StackStatus::UpdateComplete | StackStatus::UpdateCompleteCleanupInProgress => {
                println!("update completed successfully!");
                break;
            }
            StackStatus::UpdateRollbackComplete
            | StackStatus::UpdateRollbackCompleteCleanupInProgress
            | StackStatus::UpdateRollbackFailed
            | StackStatus::UpdateRollbackInProgress
            | StackStatus::UpdateFailed => {
                println!("update failed");
                break;
            }
            StackStatus::UpdateInProgress => {
                println!("updating...");
            }
            _ => {
                println!("encountered unexpected cloudformation status: {status}");
                break;
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}

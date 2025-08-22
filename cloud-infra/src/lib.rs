use aws_sdk_cloudformation::types::Capability;
use cloud_infra_core::stack::{Asset, Resource, Stack, StackBuilder};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub struct Synth(pub String, Vec<Asset>);

impl Display for Synth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn synth(resources: Vec<Resource>) -> Result<Synth, String> {
    let mut stack_builder = StackBuilder::new();
    resources.into_iter().for_each(|r| stack_builder.add_resource(r));
    let stack = stack_builder.build();
    let assets = stack.get_assets();

    serde_json::to_string(&stack)
        .map(|s| Synth(s, assets))
        .map_err(|e| format!("Could not serialize resources: {e:?}"))
}

pub fn synth_stack(stack: Stack) -> Result<Synth, String> {
    let assets = stack.get_assets();

    serde_json::to_string(&stack)
        .map(|s| Synth(s, assets))
        .map_err(|e| format!("Could not serialize resources: {e:?}"))
}

// TODO does not work with existing stacks...
pub async fn deploy(name: &str, synth: Synth) {
    let config = aws_config::load_from_env().await;
    let s3_client = Arc::new(aws_sdk_s3::Client::new(&config));

    let tasks: Vec<_> = synth.1.into_iter().map(|a| {
        println!("Uploading asset {} to {}/{}", a.path, a.s3_bucket, a.s3_key);
        let s3_client = s3_client.clone();
        tokio::spawn(async move {
            let body = aws_sdk_s3::primitives::ByteStream::from_path(a.path).await;
            s3_client
                .put_object()
                .bucket(a.s3_bucket)
                .key(a.s3_key)
                .body(body.unwrap())//
                .send()
                .await
                .unwrap();
        })
    }).collect();

    for task in tasks {
        task.await.unwrap();
    }

    let cloudformation_client = aws_sdk_cloudformation::Client::new(&config);
    match cloudformation_client
        .create_stack()
        .stack_name(name)
        .template_body(synth.0)
        .capabilities(Capability::CapabilityNamedIam)
        .send()
        .await
    {
        Ok(_) => println!("Stack {name} creation started"),
        Err(e) => eprintln!("An error occurred while creating the stack: {e:?}"),
    }
}

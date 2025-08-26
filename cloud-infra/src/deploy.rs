use std::sync::Arc;
use aws_sdk_cloudformation::types::Capability;
use crate::synth::Synth;

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

    println!("Starting stack creation");
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
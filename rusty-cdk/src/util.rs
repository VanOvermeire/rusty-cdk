use aws_config::SdkConfig;
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_sdk_cloudformation::Client;
use aws_sdk_cloudformation::types::StackStatus;

pub(crate) async fn load_config(with_stall_protection: bool) -> SdkConfig {
    let mut config = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if with_stall_protection {
        // https://github.com/awslabs/aws-sdk-rust/issues/1146
        config = config.stalled_stream_protection(StalledStreamProtectionConfig::disabled());
    }

    config.load().await
}

pub(crate) async fn get_existing_template(client: &Client, stack_name: &str) -> Option<String> {
    match client.describe_stacks().stack_name(stack_name).send().await {
        Ok(_) => {
            let template = client.get_template().stack_name(stack_name).send().await;
            template.unwrap().template_body
        }
        Err(_) => None,
    }
}

pub(crate) async fn get_stack_status(name: &String, cloudformation_client: &Client) -> Option<StackStatus> {
    let status = cloudformation_client.describe_stacks().stack_name(name).send().await;
    status
        .ok()
        .and_then(|v| v.stacks)
        .and_then(|mut v| v.pop())
        .and_then(|v| v.stack_status)
}

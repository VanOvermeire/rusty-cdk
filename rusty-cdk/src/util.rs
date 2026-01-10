use aws_config::SdkConfig;
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_sdk_cloudformation::Client;
use aws_sdk_cloudformation::types::StackStatus;

pub async fn load_config() -> SdkConfig {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        // https://github.com/awslabs/aws-sdk-rust/issues/1146
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .load()
        .await;
    config
}

pub async fn get_existing_template(client: &Client, stack_name: &str) -> Option<String> {
    match client.describe_stacks().stack_name(stack_name).send().await {
        Ok(_) => {
            let template = client.get_template().stack_name(stack_name).send().await;
            template.unwrap().template_body
        }
        Err(_) => None,
    }
}

pub async fn get_stack_status(name: &String, cloudformation_client: &Client) -> Option<StackStatus> {
    let status = cloudformation_client.describe_stacks().stack_name(name).send().await;
    status
        .ok()
        .and_then(|v| v.stacks)
        .and_then(|mut v| v.pop())
        .and_then(|v| v.stack_status)
}

use aws_sdk_cloudformation::Client;

pub async fn get_existing_template(client: &Client, stack_name: &str) -> Option<String> {
    match client.describe_stacks().stack_name(stack_name).send().await {
        Ok(_) => {
            let template = client.get_template().stack_name(stack_name).send().await;
            template.unwrap().template_body
        }
        Err(_) => None,
    }
}
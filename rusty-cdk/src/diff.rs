use std::process::exit;
use aws_sdk_cloudformation::Client;
use rusty_cdk_core::stack::{Stack, StackDiff};
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
use crate::util::get_existing_template;

/// Creates a diff that will show what ids are being added / removed to an existing stack, as well as showing ids that remain without being added or removed.
/// Currently, the diff does not show modifications to resources.
/// 
/// # Parameters
///
/// * `name` - The existing CloudFormation stack name
/// * `stack` - The new stack
///
/// # AWS Credentials
///
/// This function requires valid AWS credentials.
/// The AWS credentials must have permissions for:
/// - `cloudformation:DescribeStacks`, `cloudformation:GetTemplate`
#[allow(unused)]
pub async fn diff(name: StringWithOnlyAlphaNumericsAndHyphens, stack: Stack) {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let cloudformation_client = Client::new(&config);
    
    match get_existing_template(&cloudformation_client, &name.0).await {
        None => {
            eprintln!("could not find existing stack with name {}", name.0);
            exit(1);
        }
        Some(existing) => {
            let diff = stack.get_diff(&existing);
            
            match diff {
                Ok(diff) => {
                    let StackDiff { new_ids, unchanged_ids, ids_to_be_removed} = diff;
                    println!("- added ids: {}", print_ids(new_ids));
                    println!("- removed ids: {}", print_ids(ids_to_be_removed));
                    println!("- id that stay: {}", print_ids(unchanged_ids));
                }
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }
}

fn print_ids(ids: Vec<(String, String)>) -> String {
    if ids.is_empty() {
        "(none)".to_string()
    } else {
        ids.into_iter().map(|v| format!("{} (resource {})", v.0, v.1)).collect::<Vec<_>>().join(", ")
    }
}
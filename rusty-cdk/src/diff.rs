use aws_sdk_cloudformation::Client;
use rusty_cdk_core::stack::{Stack, StackDiff};
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
use crate::util::{get_existing_template, load_config};

/// Creates a diff that will show what ids are being added / removed to an existing stack, as well as showing ids that remain without being added or removed.
/// Currently, the diff does not show modifications to resources.
/// 
/// # Parameters
///
/// * `name` - The existing CloudFormation stack name
/// * `stack` - The new stack
/// * `print_progress` - Print progress updates to standard out
///
/// # AWS Credentials
///
/// This function requires valid AWS credentials.
/// The AWS credentials must have permissions for:
/// - `cloudformation:DescribeStacks`
/// - `cloudformation:GetTemplate`
pub async fn diff(name: StringWithOnlyAlphaNumericsAndHyphens, stack: Stack) -> Result<String, String> {
    let config = load_config(false).await;
    let cloudformation_client = Client::new(&config);

    match get_existing_template(&cloudformation_client, &name.0).await {
        None => {
            Err(format!("could not find existing stack with name {}", name.0))
        }
        Some(existing) => {
            let diff = stack.get_diff(&existing);

            match diff {
                Ok(diff) => {
                    let StackDiff { new_ids, unchanged_ids, ids_to_be_removed} = diff;
                    let output = format!("- added ids: {}\n- removed ids: {}\n- ids that stay: {}", print_ids(new_ids), print_ids(ids_to_be_removed), print_ids(unchanged_ids));
                    Ok(output)
                }
                Err(e) => {
                    Err(e)
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
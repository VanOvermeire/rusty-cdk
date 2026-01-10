use aws_sdk_cloudformation::Client;
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::exit;
use std::time::Duration;
use aws_sdk_cloudformation::types::StackStatus;
use tokio::time::sleep;
use crate::util::{get_stack_status, load_config};

#[derive(Debug)]
pub enum DestroyError {
    StackDeleteError(String),
    UnknownError(String),
}

impl Error for DestroyError {}

impl Display for DestroyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DestroyError::StackDeleteError(_) => f.write_str("unable to delete stack"),
            DestroyError::UnknownError(_) => f.write_str("unknown error"),
        }
    }
}

/// Destroy a deployed stack
pub async fn destroy(name: StringWithOnlyAlphaNumericsAndHyphens) {
    let name = name.0;
    let config = load_config().await;
    let cloudformation_client = Client::new(&config);

    match destroy_stack(&name, &cloudformation_client).await {
        Ok(()) => {},
        Err(e) => {
            eprintln!("{:?}", e);
            exit(1);
        }
    }

    loop {
        let status = get_stack_status(&name, &cloudformation_client).await;

        if let Some(status) = status {
            match status {
                StackStatus::DeleteComplete => {
                    println!("destroy completed successfully!");
                    exit(0);
                }
                StackStatus::DeleteInProgress => {
                    println!("destroying...");
                }
                StackStatus::DeleteFailed => {
                    println!("destroy failed");
                    exit(1);
                }
                _ => {
                    println!("encountered unexpected cloudformation status: {status}");
                    exit(1);
                }
            }
        } else {
            // no status, so stack should be gone
            println!("destroy completed successfully!");
            exit(0);
        }

        sleep(Duration::from_secs(10)).await;
    }
}

/// Destroy a deployed stack
///
/// It returns a `Result`. In case of error, a `DestroyError` is returned.
pub async fn destroy_with_result(name: StringWithOnlyAlphaNumericsAndHyphens) -> Result<(), DestroyError> {
    let name = name.0;
    let config = load_config().await;
    let cloudformation_client = Client::new(&config);

    destroy_stack(&name, &cloudformation_client).await?;

    loop {
        let status = get_stack_status(&name, &cloudformation_client).await;
        
        if let Some(status) = status {
            match status {
                StackStatus::DeleteComplete => {
                    return Ok(())
                }
                StackStatus::DeleteInProgress => {}
                StackStatus::DeleteFailed => {
                    return Err(DestroyError::StackDeleteError(format!("{status}")));
                }
                _ => {
                    return Err(DestroyError::UnknownError(format!("{status}")));
                }
            }
        } else {
            // no status, so stack should be gone
            return Ok(())
        }

        sleep(Duration::from_secs(10)).await;
    }
}

async fn destroy_stack(name: &String, cloudformation_client: &Client) -> Result<(), DestroyError> {
    let delete_result = cloudformation_client
        .delete_stack()
        .stack_name(name)
        .send()
        .await;
    match delete_result {
        Ok(_) => Ok(()),
        Err(e) => Err(DestroyError::StackDeleteError(e.to_string()))
    }
}

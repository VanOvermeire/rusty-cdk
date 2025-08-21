// #[derive(Serialize)]
// pub enum Effect {
//     Allow,
//     Deny
// }
//
// #[derive(Serialize)]
// pub enum Service {
//     Lambda
// }
//
// impl From<Service> for String {
//     fn from(value: Service) -> Self {
//         match value {
//             Service::Lambda => "lambda.amazonaws.com".to_string()
//         }
//     }
// }

// TODO statement builder (and others) (use them in the other builders)

use serde_json::Value;
use crate::dynamodb::DynamoDBTable;
use crate::iam::{Policy, PolicyDocument, Statement};
use crate::intrinsic_functions::get_arn;

pub enum Permission<'a> {
    DynamoDBRead(&'a DynamoDBTable),
    // DynamoDBReadWrite(DynamoDBTable), // TODO activate
}

impl Permission<'_> {
    pub(crate) fn into_policy(self) -> Policy {
        match self {
            Permission::DynamoDBRead(table) => {
                let id = table.get_id();
                let statement = Statement {
                    action: vec![
                        "dynamodb:Get*".to_string(),
                        "dynamodb:DescribeTable".to_string(),
                        "dynamodb:BatchGetItem".to_string(),
                        "dynamodb:ConditionCheckItem".to_string(),
                        "dynamodb:Query".to_string(),
                        "dynamodb:Scan".to_string(),
                    ],
                    effect: "Allow".to_string(),
                    principal: None,
                    resource: Some(vec![get_arn(&id)]),
                };
                let policy_document = PolicyDocument::new(vec![statement]);
                Policy {
                    policy_name: format!("{}Read", id),
                    policy_document,
                }
            }
        }
    }
}

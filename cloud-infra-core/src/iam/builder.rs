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

use crate::dynamodb::DynamoDBTable;
use crate::iam::{Policy, PolicyDocument, Statement};

pub enum Permission<'a> {
    DynamoDBRead(&'a DynamoDBTable),
    // DynamoDBReadWrite(DynamoDBTable), // TODO activate
}

impl Permission<'_> {
    pub(crate) fn into_policy(self) -> Policy {
        match self {
            Permission::DynamoDBRead(table) => {
                //     "Resource": [
                //     {
                //         "Fn::GetAtt": [
                //         "someId25FE8D3B",
                //         "Arn"
                //         ]
                //     }

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
                    resource: Some("*".to_string()), // TODO!
                };
                let policy_document = PolicyDocument::new(vec![statement]);
                Policy {
                    policy_name: format!("{}Read", table.get_id()),
                    policy_document,
                }
            }
        }
    }
}

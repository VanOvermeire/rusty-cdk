use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct IamRole {
    #[serde(skip)]
    id: String,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(rename = "Properties")]
    properties: IamRoleProperties,
}

impl IamRole {
    pub(crate) fn new(id: String, properties: IamRoleProperties) -> Self {
        Self {
            id,
            r#type: "AWS::IAM::Role".to_string(),
            properties,
        }
    }

    pub(crate) fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Serialize)]
pub struct IamRoleProperties {
    #[serde(rename = "AssumeRolePolicyDocument")]
    pub(crate) assumed_role_policy_document: AssumeRolePolicyDocument,
    #[serde(rename = "ManagedPolicyArns")]
    pub(crate) managed_policy_arns: Vec<Value>,
    // #[serde(rename = "RoleName", skip_serializing_if = "Option::is_none")]
    // pub(crate) role_name: Option<String>,
    // "Policies" : [ Policy, ... ],
}

#[derive(Serialize)]
pub struct AssumeRolePolicyDocument {
    #[serde(rename = "Statement")]
    statements: Vec<Statement>,
    #[serde(rename = "Version")]
    version: String,
}

impl AssumeRolePolicyDocument {
    pub(crate) fn new(statements: Vec<Statement>) -> AssumeRolePolicyDocument {
        Self {
            statements,
            version: "2012-10-17".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Statement {
    #[serde(rename = "Action")]
    pub(crate) action: String,
    #[serde(rename = "Effect")]
    pub(crate) effect: String,
    #[serde(rename = "Principal")]
    pub(crate) principal: Principal,
}

// TODO does not have to contain service per se
#[derive(Serialize)]
pub struct Principal {
    #[serde(rename = "Service")]
    pub(crate) service: String,
}

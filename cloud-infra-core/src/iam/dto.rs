use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct IamRole {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: IamRoleProperties,
}

impl IamRole {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct IamRoleProperties {
    #[serde(rename = "AssumeRolePolicyDocument")]
    pub(crate) assumed_role_policy_document: AssumeRolePolicyDocument,
    #[serde(rename = "ManagedPolicyArns")]
    pub(crate) managed_policy_arns: Vec<Value>,
    #[serde(rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub(crate) policies: Option<Vec<Policy>>,
    #[serde(rename = "RoleName", skip_serializing_if = "Option::is_none")]
    pub(crate) role_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Policy {
    #[serde(rename = "PolicyName")]
    pub(crate) policy_name: String,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: PolicyDocument,
}

#[derive(Debug, Serialize)]
pub struct PolicyDocument {
    #[serde(rename = "Version")]
    pub(crate) version: String,
    #[serde(rename = "Statement")]
    pub(crate) statements: Vec<Statement>
}

#[derive(Debug, Serialize)]
pub struct AssumeRolePolicyDocument {
    #[serde(rename = "Statement")]
    pub(crate) statements: Vec<Statement>,
    #[serde(rename = "Version")]
    pub(crate) version: String,
}

#[derive(Debug, Serialize)]
pub struct Statement {
    #[serde(rename = "Action")]
    pub(crate) action: Vec<String>,
    #[serde(rename = "Effect")]
    pub(crate) effect: String,
    #[serde(rename = "Principal", skip_serializing_if = "Option::is_none")]
    pub(crate) principal: Option<Principal>,
    #[serde(rename = "Resource", skip_serializing_if = "Option::is_none")]
    pub(crate) resource: Option<Vec<Value>>,
}

// TODO does not have to contain service per se (that's only one option)
#[derive(Debug, Serialize)]
pub struct Principal {
    #[serde(rename = "Service")]
    pub(crate) service: String,
}

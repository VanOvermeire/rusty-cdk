use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;
use crate::ref_struct;
use crate::shared::Id;

// try to limit visibility to super 

ref_struct!(RoleRef);

#[derive(Debug, Serialize)]
pub struct Role {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(skip)]
    pub(crate) potentially_missing_services: Vec<String>,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: IamRoleProperties,
}

impl Role {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
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
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub(crate) condition: Option<Value>
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Principal {
    Service(ServicePrincipal),
    AWS(AWSPrincipal),
    Custom(String),
}

#[derive(Debug, Serialize)]
pub struct ServicePrincipal {
    #[serde(rename = "Service")]
    pub(crate) service: String,
}

#[derive(Debug, Serialize)]
pub struct AWSPrincipal {
    #[serde(rename = "AWS")]
    pub(crate) aws: String,
}

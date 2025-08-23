use crate::dynamodb::DynamoDBTable;
use crate::iam::{AssumeRolePolicyDocument, IamRole, IamRoleProperties, Policy, PolicyDocument, Principal, Statement};
use crate::intrinsic_functions::get_arn;
use serde_json::Value;
use std::marker::PhantomData;
use std::vec;

pub struct IamRoleBuilder {}

impl IamRoleBuilder {
    pub fn new(id: String, properties: IamRoleProperties) -> IamRole {
        IamRole {
            id,
            r#type: "AWS::IAM::Role".to_string(),
            properties,
        }
    }
}

pub struct IamRolePropertiesBuilder {
    assumed_role_policy_document: AssumeRolePolicyDocument,
    managed_policy_arns: Vec<Value>,
    policies: Option<Vec<Policy>>,
    role_name: Option<String>,
}

impl IamRolePropertiesBuilder {
    pub fn new(assumed_role_policy_document: AssumeRolePolicyDocument, managed_policy_arns: Vec<Value>) -> IamRolePropertiesBuilder {
        IamRolePropertiesBuilder {
            assumed_role_policy_document,
            managed_policy_arns,
            policies: None,
            role_name: None,
        }
    }
    
    pub fn policies(self, policies: Vec<Policy>) -> IamRolePropertiesBuilder {
        Self {
            policies: Some(policies),
            ..self
        }
    }
    
    pub fn role_name(self, role_name: String) -> IamRolePropertiesBuilder {
        Self {
            role_name: Some(role_name),
            ..self
        }
    }
    
    pub fn build(self) -> IamRoleProperties {
        IamRoleProperties {
            assumed_role_policy_document: self.assumed_role_policy_document,
            managed_policy_arns: self.managed_policy_arns,
            policies: self.policies,
            role_name: self.role_name,
        }
    }
}

pub trait PolicyBuilderState {}

pub struct StartState {}
impl PolicyBuilderState for StartState {}

pub struct PolicyDocumentState {}
impl PolicyBuilderState for PolicyDocumentState {}

pub struct PolicyBuilder<T: PolicyBuilderState> {
    state: PhantomData<T>,
    policy_name: String,
    policy_document: Option<PolicyDocument>,
}

impl<T: PolicyBuilderState> PolicyBuilder<T> {
    pub fn new(policy_name: String) -> PolicyBuilder<StartState> {
        PolicyBuilder {
            state: Default::default(),
            policy_name,
            policy_document: None,
        }
    }
}

impl PolicyBuilder<StartState> {
    pub fn policy_document(self, document: PolicyDocument) -> PolicyBuilder<PolicyDocumentState> {
        PolicyBuilder {
            state: Default::default(),
            policy_name: self.policy_name,
            policy_document: Some(document),
        }
    }
}

impl PolicyBuilder<PolicyDocumentState> {
    pub fn build(self) -> Policy {
        Policy {
            policy_name: self.policy_name,
            policy_document: self
                .policy_document
                .expect("policy document should be set, as this is enforced by the builder"),
        }
    }
}

pub struct PolicyDocumentBuilder {}

impl PolicyDocumentBuilder {
    pub fn new(statements: Vec<Statement>) -> PolicyDocument {
        PolicyDocument {
            version: "2012-10-17".to_string(),
            statements,
        }
    }
}

pub struct AssumeRolePolicyDocumentBuilder {}

impl AssumeRolePolicyDocumentBuilder {
    pub fn new(statements: Vec<Statement>) -> AssumeRolePolicyDocument {
        AssumeRolePolicyDocument {
            version: "2012-10-17".to_string(),
            statements,
        }
    }
}

pub enum Effect {
    Allow,
    Deny,
}

impl From<Effect> for String {
    fn from(value: Effect) -> Self {
        match value {
            Effect::Allow => "Allow".to_string(),
            Effect::Deny => "Deny".to_string(),
        }
    }
}

pub trait StatementState {}

pub struct StatementStartState {}
impl StatementState for StatementStartState {}

pub struct StatementBuilder {
    action: Vec<String>,
    effect: Effect,
    principal: Option<Principal>,
    resource: Option<Vec<Value>>,
}

impl StatementBuilder {
    pub fn new(action: Vec<String>, effect: Effect) -> Self {
        Self {
            action,
            effect,
            principal: None,
            resource: None,
        }
    }

    pub fn principal(self, principal: Principal) -> Self {
        Self {
            principal: Some(principal),
            ..self
        }
    }

    pub fn resources(self, resources: Vec<Value>) -> Self {
        Self {
            resource: Some(resources),
            ..self
        }
    }

    pub fn all_resources(self) -> Self {
        Self {
            resource: Some(vec![Value::String("*".to_string())]),
            ..self
        }
    }

    pub fn build(self) -> Statement {
        Statement {
            action: self.action,
            effect: self.effect.into(),
            principal: self.principal,
            resource: self.resource,
        }
    }
}

// should this also be a builder for conformity?
pub enum Permission<'a> {
    DynamoDBRead(&'a DynamoDBTable),
    DynamoDBReadWrite(&'a DynamoDBTable),
    // TODO custom, add any permission you want...
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
                let policy_document = PolicyDocumentBuilder::new(vec![statement]);
                Policy {
                    policy_name: format!("{}Read", id),
                    policy_document,
                }
            }
            Permission::DynamoDBReadWrite(table) => {
                let id = table.get_id();
                let statement = Statement {
                    action: vec![
                        "dynamodb:Get*".to_string(),
                        "dynamodb:DescribeTable".to_string(),
                        "dynamodb:BatchGetItem".to_string(),
                        "dynamodb:BatchWriteItem".to_string(),
                        "dynamodb:ConditionCheckItem".to_string(),
                        "dynamodb:Query".to_string(),
                        "dynamodb:Scan".to_string(),
                        "dynamodb:DeleteItem".to_string(),
                        "dynamodb:PutItem".to_string(),
                        "dynamodb:UpdateItem".to_string(),
                    ],
                    effect: "Allow".to_string(),
                    principal: None,
                    resource: Some(vec![get_arn(&id)]),
                };
                let policy_document = PolicyDocumentBuilder::new(vec![statement]);
                Policy {
                    policy_name: format!("{}ReadWrite", id),
                    policy_document,
                }
            }
        }
    }
}

use std::marker::PhantomData;
use crate::dynamodb::Table;
use crate::iam::{AssumeRolePolicyDocument, Role, IamRoleProperties, Policy, PolicyDocument, Principal, Statement, AWSPrincipal, ServicePrincipal};
use crate::intrinsic_functions::{get_arn, join};
use crate::s3::dto::Bucket;
use crate::shared::Id;
use crate::sqs::Queue;
use crate::wrappers::IamAction;
use serde_json::Value;
use std::vec;

pub trait PrincipalState {}

pub struct StartState {}
impl PrincipalState for StartState {}

pub struct ChosenState {}
impl PrincipalState for ChosenState {}

pub struct PrincipalBuilder<T: PrincipalState> {
    phantom_data: PhantomData<T>,
    service: Option<String>,
    aws: Option<String>,
    normal: Option<String>
}

impl PrincipalBuilder<StartState> {
    pub fn new() -> PrincipalBuilder<StartState> {
        PrincipalBuilder {
            phantom_data: Default::default(),
            service: None,
            aws: None,
            normal: None,
        }
    }
    
    pub fn service<T: Into<String>>(self, service: T) -> PrincipalBuilder<ChosenState> {
        PrincipalBuilder {
            phantom_data: Default::default(),
            service: Some(service.into()),
            aws: self.aws,
            normal: self.normal
        }
    }

    pub fn aws<T: Into<String>>(self, aws: T) -> PrincipalBuilder<ChosenState> {
        PrincipalBuilder {
            phantom_data: Default::default(),
            aws: Some(aws.into()),
            service: self.service,
            normal: self.normal
        }
    }

    pub fn normal<T: Into<String>>(self, normal: T) -> PrincipalBuilder<ChosenState> {
        PrincipalBuilder {
            phantom_data: Default::default(),
            normal: Some(normal.into()),
            service: self.service,
            aws: self.aws,
        }
    }
}

impl PrincipalBuilder<ChosenState> {
    pub fn build(self) -> Principal {
        if let Some(aws) = self.aws {
            Principal::AWS(AWSPrincipal {
                aws,
            })
        } else if let Some(service) = self.service {
            Principal::Service(ServicePrincipal {
                service,
            })
        } else if let Some(normal) = self.normal {
            Principal::Custom(normal)
        } else {
            unreachable!("can only reach build state when one of the above is present")
        }
    }
}

pub struct RoleBuilder {}

impl RoleBuilder {
    pub fn new(id: &str, resource_id: &str, properties: IamRoleProperties) -> Role {
        Role {
            id: Id(id.to_string()),
            resource_id: resource_id.to_string(),
            potentially_missing_services: vec![],
            r#type: "AWS::IAM::Role".to_string(),
            properties,
        }
    }

    pub(crate) fn new_with_missing_info(id: &str, resource_id: &str, properties: IamRoleProperties, potentially_missing: Vec<String>) -> Role {
        Role {
            id: Id(id.to_string()),
            resource_id: resource_id.to_string(),
            potentially_missing_services: potentially_missing,
            r#type: "AWS::IAM::Role".to_string(),
            properties,
        }
    }
}

pub struct RolePropertiesBuilder {
    assumed_role_policy_document: AssumeRolePolicyDocument,
    managed_policy_arns: Vec<Value>,
    policies: Option<Vec<Policy>>,
    role_name: Option<String>,
}

impl RolePropertiesBuilder {
    pub fn new(assumed_role_policy_document: AssumeRolePolicyDocument, managed_policy_arns: Vec<Value>) -> RolePropertiesBuilder {
        RolePropertiesBuilder {
            assumed_role_policy_document,
            managed_policy_arns,
            policies: None,
            role_name: None,
        }
    }

    pub fn policies(self, policies: Vec<Policy>) -> RolePropertiesBuilder {
        Self {
            policies: Some(policies),
            ..self
        }
    }

    pub fn role_name<T: Into<String>>(self, role_name: T) -> RolePropertiesBuilder {
        Self {
            role_name: Some(role_name.into()),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> IamRoleProperties {
        IamRoleProperties {
            assumed_role_policy_document: self.assumed_role_policy_document,
            managed_policy_arns: self.managed_policy_arns,
            policies: self.policies,
            role_name: self.role_name,
        }
    }
}

pub struct PolicyBuilder {
    policy_name: String,
    policy_document: PolicyDocument,
}

impl PolicyBuilder {
    pub fn new<T: Into<String>>(policy_name: T, policy_document: PolicyDocument) -> Self {
        PolicyBuilder {
            policy_name: policy_name.into(),
            policy_document,
        }
    }

    #[must_use]
    pub fn build(self) -> Policy {
        Policy {
            policy_name: self.policy_name,
            policy_document: self.policy_document,
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
    condition: Option<Value>
}

impl StatementBuilder {
    pub(crate) fn internal_new(action: Vec<String>, effect: Effect) -> Self {
        Self {
            action,
            effect,
            principal: None,
            resource: None,
            condition: None,
        }
    }

    pub fn new(actions: Vec<IamAction>, effect: Effect) -> Self {
        Self::internal_new(actions.into_iter().map(|a| a.0).collect(), effect)
    }

    pub fn principal(self, principal: Principal) -> Self {
        Self {
            principal: Some(principal),
            ..self
        }
    }
    
    pub fn condition(self, condition: Value) -> Self {
        Self {
            condition: Some(condition),
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

    #[must_use]
    pub fn build(self) -> Statement {
        Statement {
            action: self.action,
            effect: self.effect.into(),
            principal: self.principal,
            resource: self.resource,
            condition: self.condition,
        }
    }
}

pub struct CustomPermission {
    id: String,
    statement: Statement,
}

impl CustomPermission {
    pub fn new(id: &str, statement: Statement) -> Self {
        Self {
            id: id.to_string(),
            statement,
        }
    }
}

pub enum Permission<'a> {
    DynamoDBRead(&'a Table),
    DynamoDBReadWrite(&'a Table),
    SqsRead(&'a Queue),
    S3ReadWrite(&'a Bucket),
    Custom(CustomPermission),
}

impl Permission<'_> {
    pub(crate) fn get_referenced_id(&self) -> Option<&str> {
        let id = match self {
            Permission::DynamoDBRead(d) => d.get_resource_id(),
            Permission::DynamoDBReadWrite(d) => d.get_resource_id(),
            Permission::SqsRead(s) => s.get_resource_id(),
            Permission::S3ReadWrite(s) => s.get_resource_id(),
            Permission::Custom { .. } => return None,
        };
        Some(id)
    }

    pub(crate) fn into_policy(self) -> Policy {
        match self {
            Permission::DynamoDBRead(table) => {
                let id = table.get_resource_id();
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
                    resource: Some(vec![get_arn(id)]),
                    principal: None,
                    condition: None,
                };
                let policy_document = PolicyDocumentBuilder::new(vec![statement]);
                PolicyBuilder::new(format!("{}Read", id), policy_document).build()
            }
            Permission::DynamoDBReadWrite(table) => {
                let id = table.get_resource_id();
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
                    resource: Some(vec![get_arn(id)]),
                    principal: None,
                    condition: None,
                };
                let policy_document = PolicyDocumentBuilder::new(vec![statement]);
                PolicyBuilder::new(format!("{}ReadWrite", id), policy_document).build()
            }
            Permission::SqsRead(queue) => {
                let id = queue.get_resource_id();
                let sqs_permissions_statement = StatementBuilder::internal_new(
                    vec![
                        "sqs:ChangeMessageVisibility".to_string(),
                        "sqs:DeleteMessage".to_string(),
                        "sqs:GetQueueAttributes".to_string(),
                        "sqs:GetQueueUrl".to_string(),
                        "sqs:ReceiveMessage".to_string(),
                    ],
                    Effect::Allow,
                )
                .resources(vec![get_arn(id)])
                .build();
                let policy_document = PolicyDocumentBuilder::new(vec![sqs_permissions_statement]);
                PolicyBuilder::new(format!("{}Read", id), policy_document).build()
            }
            Permission::S3ReadWrite(bucket) => {
                let id = bucket.get_resource_id();
                let arn = get_arn(id);
                let s3_permissions_statement = StatementBuilder::internal_new(
                    vec![
                        "s3:Abort*".to_string(),
                        "s3:DeleteObject*".to_string(),
                        "s3:GetBucket*".to_string(),
                        "s3:GetObject*".to_string(),
                        "s3:List*".to_string(),
                        "s3:PutObject".to_string(),
                        "s3:PutObjectLegalHold".to_string(),
                        "s3:PutObjectRetention".to_string(),
                        "s3:PutObjectTagging".to_string(),
                        "s3:PutObjectVersionTagging".to_string(),
                    ],
                    Effect::Allow,
                )
                .resources(vec![arn.clone(), join("/", vec![arn, Value::String("*".to_string())])])
                .build();

                let policy_document = PolicyDocumentBuilder::new(vec![s3_permissions_statement]);
                PolicyBuilder::new(format!("{}ReadWrite", id), policy_document).build()
            }
            Permission::Custom(CustomPermission { id, statement }) => {
                let policy_document = PolicyDocumentBuilder::new(vec![statement]);
                PolicyBuilder::new(id, policy_document).build()
            }
        }
    }
}

use crate::dynamodb::TableRef;
use crate::iam::{AWSPrincipal, AssumeRolePolicyDocument, IamRoleProperties, Policy, PolicyDocument, Principal, Role, RoleRef, ServicePrincipal, Statement};
use crate::intrinsic::{get_arn, get_ref, join, AWS_ACCOUNT_PSEUDO_PARAM};
use crate::s3::BucketRef;
use crate::shared::Id;
use crate::sqs::QueueRef;
use crate::stack::{Resource, StackBuilder};
use crate::wrappers::{IamAction, PolicyName};
use serde_json::Value;
use std::marker::PhantomData;
use std::vec;
use crate::appconfig::{ApplicationRef, ConfigurationProfileRef, EnvironmentRef};
use crate::secretsmanager::SecretRef;
use crate::type_state;

type_state!(
    PrincipalState,
    StartState,
    ChosenState,
);

/// Builder for IAM principals (service, AWS, or custom).
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::iam::PrincipalBuilder;
///
/// // Service principal
/// let service_principal = PrincipalBuilder::new()
///     .service("lambda.amazonaws.com")
///     .build();
///
/// // Custom principal
/// let custom_principal = PrincipalBuilder::new()
///     .normal("*")
///     .build();
/// ```
pub struct PrincipalBuilder<T: PrincipalState> {
    phantom_data: PhantomData<T>,
    service: Option<String>,
    aws: Option<String>,
    normal: Option<String>
}

impl Default for PrincipalBuilder<StartState> {
    fn default() -> Self {
        Self::new()
    }
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

/// Builder for IAM roles.
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::iam::{RoleBuilder, RolePropertiesBuilder, AssumeRolePolicyDocumentBuilder, StatementBuilder, PrincipalBuilder, Effect};
/// use rusty_cdk_core::stack::Resource;
/// use rusty_cdk_macros::iam_action;
/// use rusty_cdk_core::wrappers::IamAction;
///
/// let mut stack_builder = StackBuilder::new();
///
/// let assume_role_statement = StatementBuilder::new(
///     vec![iam_action!("sts:AssumeRole")],
///     Effect::Allow
/// )
/// .principal(PrincipalBuilder::new().service("lambda.amazonaws.com").build())
/// .build();
///
/// let assume_role_policy = AssumeRolePolicyDocumentBuilder::new(vec![assume_role_statement]).build();
///
/// let properties = RolePropertiesBuilder::new(assume_role_policy, vec![])
///     .build();
///
/// let role = RoleBuilder::new("my-role", properties)
///     .build(&mut stack_builder);
/// ```
pub struct RoleBuilder {
    id: Id,
    resource_id: Option<String>,
    properties: IamRoleProperties,
    potentially_missing: Vec<String>
}

impl RoleBuilder {
    /// Creates a new IAM role builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the role
    /// * `properties` - IAM role properties including policies and trust relationships
    pub fn new(id: &str, properties: IamRoleProperties) -> RoleBuilder {
        RoleBuilder {
            id: Id(id.to_string()),
            resource_id: None,
            properties,
            potentially_missing: vec![],
        }
    }

    pub(crate) fn new_with_info_on_missing(id: &str, resource_id: &str, properties: IamRoleProperties, potentially_missing: Vec<String>) -> RoleBuilder {
        Self {
            id: Id(id.to_string()),
            resource_id: Some(resource_id.to_string()),
            properties,
            potentially_missing,
        }
    }
    
    pub fn build(self, stack_builder: &mut StackBuilder) -> RoleRef {
        let resource_id = self.resource_id.unwrap_or_else(|| Resource::generate_id("Role"));
        
        stack_builder.add_resource(Role {
            id: self.id,
            resource_id: resource_id.clone(),
            potentially_missing_services: self.potentially_missing,
            r#type: "AWS::IAM::Role".to_string(),
            properties: self.properties,
        });
        RoleRef::internal_new(resource_id)
    }
}

/// Builder for IAM role properties.
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

/// Builder for IAM policies.
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::iam::{PolicyBuilder, PolicyDocumentBuilder, StatementBuilder, Effect};
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::{iam_action, policy_name};
///
/// let statement = StatementBuilder::new(
///     vec![iam_action!("s3:GetObject")],
///     Effect::Allow
/// )
/// .all_resources()
/// .build();
///
/// let policy_doc = PolicyDocumentBuilder::new(vec![statement]).build();
/// let policy = PolicyBuilder::new(policy_name!("MyPolicy"), policy_doc).build();
/// ```
pub struct PolicyBuilder {
    policy_name: String,
    policy_document: PolicyDocument,
}

impl PolicyBuilder {
    pub fn new(policy_name: PolicyName, policy_document: PolicyDocument) -> Self {
        PolicyBuilder {
            policy_name: policy_name.0,
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

/// Builder for IAM policy documents.
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::iam::{PolicyDocumentBuilder, StatementBuilder, Effect};
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::iam_action;
///
/// let statement = StatementBuilder::new(
///     vec![iam_action!("dynamodb:GetItem"), iam_action!("dynamodb:Query")],
///     Effect::Allow
/// )
/// .all_resources()
/// .build();
///
/// let policy_doc = PolicyDocumentBuilder::new(vec![statement]).build();
/// ```
pub struct PolicyDocumentBuilder {
    statements: Vec<Statement>
}

impl PolicyDocumentBuilder {
    pub fn new(statements: Vec<Statement>) -> PolicyDocumentBuilder {
        Self {
            statements
        }
    }
    
    pub fn build(self) -> PolicyDocument {
        PolicyDocument {
            version: "2012-10-17".to_string(),
            statements: self.statements,
        }
    }
}

pub struct AssumeRolePolicyDocumentBuilder {
    statements: Vec<Statement>
}

impl AssumeRolePolicyDocumentBuilder {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
        }
    }
    
    pub fn build(self) -> AssumeRolePolicyDocument {
        AssumeRolePolicyDocument {
            version: "2012-10-17".to_string(),
            statements: self.statements,
        }
    }
}

#[derive(Debug, Clone)]
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

/// Builder for IAM policy statements.
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::iam::{StatementBuilder, Effect, PrincipalBuilder};
/// use rusty_cdk_core::wrappers::*;
/// use serde_json::Value;
/// use rusty_cdk_macros::iam_action;
///
/// let statement = StatementBuilder::new(
///     vec![iam_action!("s3:GetObject"), iam_action!("s3:PutObject")],
///     Effect::Allow
/// )
/// .resources(vec![Value::String("arn:aws:s3:::my-bucket/*".to_string())])
/// .principal(PrincipalBuilder::new().service("lambda.amazonaws.com").build())
/// .build();
/// ```
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
    id: PolicyName,
    statement: Statement,
}

impl CustomPermission {
    /// Creates a new custom IAM permission.
    ///
    /// # Arguments
    /// * `id` - Unique name for the permission
    /// * `statement` - IAM policy statement defining the permission
    pub fn new(id: PolicyName, statement: Statement) -> Self {
        Self {
            id,
            statement,
        }
    }
}

pub enum Permission<'a> {
    AppConfigRead(&'a ApplicationRef, &'a EnvironmentRef, &'a ConfigurationProfileRef),
    DynamoDBRead(&'a TableRef),
    DynamoDBReadWrite(&'a TableRef),
    SecretsManagerRead(&'a SecretRef),
    SqsRead(&'a QueueRef),
    S3ReadWrite(&'a BucketRef),
    Custom(CustomPermission),
}

impl Permission<'_> {
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
                let policy_document = PolicyDocumentBuilder::new(vec![statement]).build();
                PolicyBuilder::new(PolicyName(format!("{}Read", id)), policy_document).build()
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
                let policy_document = PolicyDocumentBuilder::new(vec![statement]).build();
                PolicyBuilder::new(PolicyName(format!("{}ReadWrite", id)), policy_document).build()
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
                let policy_document = PolicyDocumentBuilder::new(vec![sqs_permissions_statement]).build();
                PolicyBuilder::new(PolicyName(format!("{}Read", id)), policy_document).build()
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

                let policy_document = PolicyDocumentBuilder::new(vec![s3_permissions_statement]).build();
                PolicyBuilder::new(PolicyName(format!("{}ReadWrite", id)), policy_document).build()
            }
            Permission::SecretsManagerRead(secret) => {
                let id = secret.get_resource_id();
                let statement = StatementBuilder::internal_new(vec!["secretsmanager:GetSecretValue".to_string()], Effect::Allow)
                    .resources(vec![secret.get_ref()])
                    .build();
                let policy_document = PolicyDocumentBuilder::new(vec![statement]).build();
                PolicyBuilder::new(PolicyName(format!("{}Read", id)), policy_document).build()
            }
            Permission::AppConfigRead(app, env, profile) => {
                let id = app.get_resource_id();
                let resource = join(
                    "",
                    vec![
                        Value::String("arn:aws:appconfig:*:".to_string()),
                        get_ref(AWS_ACCOUNT_PSEUDO_PARAM),
                        Value::String(":application/".to_string()),
                        app.get_ref(),
                        Value::String("/environment/".to_string()),
                        env.get_ref(),
                        Value::String("/configuration/".to_string()),
                        profile.get_ref(),
                    ],
                );
                
                let statement = StatementBuilder::internal_new(vec![
                    "appconfig:StartConfigurationSession".to_string(),
                    "appconfig:GetLatestConfiguration".to_string(),
                ], Effect::Allow)
                    .resources(vec![resource])
                    .build();
                let policy_document = PolicyDocumentBuilder::new(vec![statement]).build();
                PolicyBuilder::new(PolicyName(format!("{}Read", id)), policy_document).build()
            }
            Permission::Custom(CustomPermission { id, statement }) => {
                let policy_document = PolicyDocumentBuilder::new(vec![statement]).build();
                PolicyBuilder::new(id, policy_document).build()
            }
        }
    }
}

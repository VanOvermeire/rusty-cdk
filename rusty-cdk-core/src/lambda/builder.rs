use crate::cloudwatch::{LogGroupBuilder, LogGroupRef};
use crate::iam::{
    find_missing_services, map_toml_dependencies_to_services, AssumeRolePolicyDocumentBuilder, Effect, Permission as IamPermission, Policy, PrincipalBuilder,
    RoleBuilder, RolePropertiesBuilder, RoleRef, StatementBuilder,
};
use crate::intrinsic::{get_arn, get_ref, join, AWS_PARTITION_PSEUDO_PARAM};
use crate::lambda::{Environment, EventSourceMapping, EventSourceMappingType, EventSourceProperties, Function, FunctionRef, FunctionType, LambdaCode, LambdaFunctionProperties, LambdaPermissionProperties, LoggingInfo, Permission, PermissionRef, PermissionType, ScalingConfig};
use crate::shared::Id;
use crate::sqs::QueueRef;
use crate::stack::{Asset, Resource, StackBuilder};
use crate::type_state;
use crate::wrappers::{
    Bucket, EnvVarKey, LambdaPermissionAction, LogGroupName, Memory, RetentionInDays, SqsEventSourceMaxConcurrency,
    StringWithOnlyAlphaNumericsUnderscoresAndHyphens, Timeout, TomlFile, ZipFile,
};
use serde_json::Value;
use std::marker::PhantomData;
use std::vec;

#[derive(Debug, Clone)]
pub enum Runtime {
    NodeJs22,
    Java21,
    Python313,
    ProvidedAl2023,
}

impl From<Runtime> for String {
    fn from(value: Runtime) -> Self {
        match value {
            Runtime::NodeJs22 => "nodejs22.x".to_string(),
            Runtime::Java21 => "java21".to_string(),
            Runtime::Python313 => "python3.13".to_string(),
            Runtime::ProvidedAl2023 => "provided.al2023".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Architecture {
    X86_64,
    ARM64,
}

impl From<Architecture> for String {
    fn from(value: Architecture) -> Self {
        match value {
            Architecture::X86_64 => "x86_64".to_string(),
            Architecture::ARM64 => "arm64".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackageType {
    Image,
    Zip,
}

impl From<PackageType> for String {
    fn from(value: PackageType) -> Self {
        match value {
            PackageType::Image => "Image".to_string(),
            PackageType::Zip => "Zip".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Zip {
    bucket: String,
    file: ZipFile,
}

impl Zip {
    pub fn new(bucket: Bucket, file: ZipFile) -> Self {
        Zip { bucket: bucket.0, file }
    }
}

#[derive(Debug, Clone)]
pub enum Code {
    Zip(Zip),
    Inline(String),
}

type_state!(
    FunctionBuilderState,
    StartState,
    CodeState,
    ZipStateWithHandler,
    ZipStateWithHandlerAndRuntime,
    EventSourceMappingState,
);

struct EventSourceMappingInfo {
    id: String,
    max_concurrency: Option<u16>,
}

/// Builder for creating AWS Lambda functions.
///
/// This builder provides a fluent API for configuring Lambda functions with their associated IAM roles, environment variables, permissions, and event sources.
///
/// # Example
///
/// ```rust,no_run
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::lambda::{FunctionBuilder, Architecture, Runtime, Zip};
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::{memory, timeout, env_var_key, zip_file};
///
/// let mut stack_builder = StackBuilder::new();
///
/// let zip = unimplemented!("create a zip");
///
/// let (function, role, log_group) = FunctionBuilder::new(
///         "my-function",
///         Architecture::ARM64,
///         memory!(512),
///         timeout!(30)
///     )
///     .code(zip)
///     .handler("index.handler")
///     .runtime(Runtime::NodeJs22)
///     .env_var_string(env_var_key!("TABLE_NAME"), "my-table")
///     .build(&mut stack_builder);
/// ```
pub struct FunctionBuilder<T: FunctionBuilderState> {
    state: PhantomData<T>,
    id: Id,
    architecture: Architecture,
    memory: u16,
    timeout: u16,
    code: Option<Code>,
    handler: Option<String>,
    runtime: Option<Runtime>,
    additional_policies: Vec<Policy>,
    aws_services_in_dependencies: Vec<String>,
    env_vars: Vec<(String, Value)>,
    function_name: Option<String>,
    sqs_event_source_mapping: Option<EventSourceMappingInfo>,
    reserved_concurrent_executions: Option<u32>,
    log_group: Option<LogGroupRef>,
}

impl<T: FunctionBuilderState> FunctionBuilder<T> {
    /// Sets a custom name for the function.
    ///
    /// If not specified, a name will be generated automatically.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the function.
    pub fn function_name(self, name: StringWithOnlyAlphaNumericsUnderscoresAndHyphens) -> FunctionBuilder<T> {
        Self {
            function_name: Some(name.0),
            ..self
        }
    }

    /// Adds an IAM permission to the functions execution role.
    ///
    /// # Arguments
    ///
    /// * `permission` - The IAM permission to add.
    pub fn add_permission(mut self, permission: IamPermission) -> FunctionBuilder<T> {
        self.additional_policies.push(permission.into_policy());
        Self { ..self }
    }

    /// Checks that the function has permissions for AWS services listed in Cargo.toml dependencies.
    ///
    /// Parses the Cargo.toml to find AWS SDK dependencies and verifies that IAM permissions
    /// have been granted for those services.
    pub fn check_permissions_against_dependencies(self, cargo_toml: TomlFile) -> Self {
        let services = map_toml_dependencies_to_services(cargo_toml.0.as_ref());

        Self {
            aws_services_in_dependencies: services,
            ..self
        }
    }

    /// Adds an environment variable to the function.
    /// If you just want to set a string as the environment variable value, use `env_var_string`
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable.
    /// * `value` - The value of the environment variable, as a `serde_json::Value`.
    pub fn env_var(mut self, key: EnvVarKey, value: Value) -> FunctionBuilder<T> {
        self.env_vars.push((key.0, value));
        Self { ..self }
    }

    /// Adds a string environment variable to the function.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable.
    /// * `value` - The string value of the environment variable.
    pub fn env_var_string<V: Into<String>>(mut self, key: EnvVarKey, value: V) -> FunctionBuilder<T> {
        self.env_vars.push((key.0, Value::String(value.into())));
        Self { ..self }
    }

    /// Sets the number of reserved concurrent executions for the function.
    ///
    /// # Arguments
    ///
    /// * `executions` - The number of reserved concurrent executions.
    pub fn reserved_concurrent_executions(self, executions: u32) -> FunctionBuilder<T> {
        Self {
            reserved_concurrent_executions: Some(executions),
            ..self
        }
    }

    /// Use a custom log group for this function.
    /// If no log group is set, once is created automatically
    pub fn log_group(self, log_group: &LogGroupRef) -> Self {
        Self {
            log_group: Some(log_group.clone()),
            ..self
        }
    }

    fn build_internal(self, stack_builder: &mut StackBuilder) -> (FunctionRef, RoleRef, LogGroupRef) {
        let function_resource_id = Resource::generate_id("LambdaFunction");

        let code = match self.code.expect("code to be present, enforced by builder") {
            Code::Zip(z) => {
                let asset_id = Resource::generate_id("Asset");
                let asset_id = format!("{asset_id}.zip");

                let asset = Asset {
                    s3_bucket: z.bucket.clone(),
                    s3_key: asset_id.clone(),
                    path: z.file.0.to_string(),
                };

                let code = LambdaCode {
                    s3_bucket: Some(z.bucket),
                    s3_key: Some(asset_id),
                    zipfile: None,
                };

                (Some(asset), code)
            }
            Code::Inline(inline_code) => {
                let code = LambdaCode {
                    s3_bucket: None,
                    s3_key: None,
                    zipfile: Some(inline_code),
                };
                (None, code)
            }
        };

        if let Some(mapping) = self.sqs_event_source_mapping {
            let event_id = Id::generate_id(&self.id, "ESM");
            let event_resource_id = format!("EventSourceMapping{}", function_resource_id);
            let event_source_mapping = EventSourceMapping {
                id: event_id,
                resource_id: event_resource_id.clone(),
                r#type: EventSourceMappingType::EventSourceMappingType,
                properties: EventSourceProperties {
                    event_source_arn: Some(get_arn(&mapping.id)),
                    function_name: Some(get_ref(&function_resource_id)),
                    scaling_config: mapping.max_concurrency.map(|c| ScalingConfig { max_concurrency: c }),
                },
            };
            stack_builder.add_resource(event_source_mapping);
        };

        let assume_role_statement = StatementBuilder::internal_new(vec!["sts:AssumeRole".to_string()], Effect::Allow)
            .principal(PrincipalBuilder::new().service("lambda.amazonaws.com").build())
            .build();
        let assumed_role_policy_document = AssumeRolePolicyDocumentBuilder::new(vec![assume_role_statement]).build();
        let managed_policy_arns = vec![join(
            "",
            vec![
                Value::String("arn:".to_string()),
                get_ref(AWS_PARTITION_PSEUDO_PARAM),
                Value::String(":iam::aws:policy/service-role/AWSLambdaBasicExecutionRole".to_string()),
            ],
        )];
        let potentially_missing = find_missing_services(&self.aws_services_in_dependencies, &self.additional_policies);
        let props = RolePropertiesBuilder::new(assumed_role_policy_document, managed_policy_arns)
            .policies(self.additional_policies)
            .build();

        let role_id = Id::generate_id(&self.id, "Role");
        let role_resource_id = Resource::generate_id("LambdaFunctionRole");
        let role_ref = get_arn(&role_resource_id);
        let role = RoleBuilder::new_with_info_on_missing(&role_id, &role_resource_id, props, potentially_missing).build(stack_builder);

        let environment = if self.env_vars.is_empty() {
            None
        } else {
            Some(Environment {
                variables: self.env_vars.into_iter().collect(),
            })
        };

        let log_group = if let Some(log_group) = self.log_group {
            log_group
        } else {
            let log_group_id = Id::generate_id(&self.id, "LogGroup");
            let log_group_name = self.function_name.clone().map(|fun_name| format!("/aws/lambda/{fun_name}"));
            let base_builder = LogGroupBuilder::new(&log_group_id).log_group_retention(RetentionInDays(731));
            let log_group = if let Some(name) = log_group_name {
                base_builder.log_group_name_string(LogGroupName(name)).build(stack_builder)
            } else {
                base_builder.build(stack_builder)
            };
            log_group
        };

        let logging_info = LoggingInfo {
            log_group: Some(log_group.get_ref()),
        };

        let properties = LambdaFunctionProperties {
            code: code.1,
            architectures: vec![self.architecture.into()],
            memory_size: self.memory,
            timeout: self.timeout,
            handler: self.handler,
            runtime: self.runtime.map(Into::into),
            role: role_ref,
            function_name: self.function_name,
            environment,
            reserved_concurrent_executions: self.reserved_concurrent_executions,
            logging_info,
        };

        stack_builder.add_resource(Function {
            id: self.id.clone(),
            resource_id: function_resource_id.clone(),
            asset: code.0,
            r#type: FunctionType::FunctionType,
            properties,
        });

        let function = FunctionRef::internal_new(self.id, function_resource_id);

        (function, role, log_group)
    }
}

// TODO does it make more sense to add runtime and handler to `new`? Other builders do this for required args
impl FunctionBuilder<StartState> {
    /// Creates a new Lambda function builder.
    /// You will have to specify a handler and runtime before you are able to build a function.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the function
    /// * `architecture` - CPU architecture (x86_64 or ARM64)
    /// * `memory` - Memory allocation in MB
    /// * `timeout` - Maximum execution time
    pub fn new(id: &str, architecture: Architecture, memory: Memory, timeout: Timeout) -> FunctionBuilder<StartState> {
        FunctionBuilder {
            state: Default::default(),
            id: Id(id.to_string()),
            architecture,
            memory: memory.0,
            timeout: timeout.0,
            code: None,
            handler: None,
            runtime: None,
            additional_policies: vec![],
            aws_services_in_dependencies: vec![],
            env_vars: vec![],
            function_name: None,
            sqs_event_source_mapping: None,
            reserved_concurrent_executions: None,
            log_group: None,
        }
    }

    /// Sets the source code for the function.
    ///
    /// This can either be a zip file from S3 or inline code.
    ///
    /// # Arguments
    ///
    /// * `code` - The functions source code.
    pub fn code(self, code: Code) -> FunctionBuilder<CodeState> {
        FunctionBuilder {
            code: Some(code),
            state: Default::default(),
            id: self.id,
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            handler: self.handler,
            runtime: self.runtime,
            additional_policies: self.additional_policies,
            aws_services_in_dependencies: self.aws_services_in_dependencies,
            env_vars: self.env_vars,
            function_name: self.function_name,
            sqs_event_source_mapping: self.sqs_event_source_mapping,
            reserved_concurrent_executions: self.reserved_concurrent_executions,
            log_group: self.log_group,
        }
    }
}

impl FunctionBuilder<CodeState> {
    /// Sets the functions handler.
    ///
    /// This is the entry point for the function, e.g., "index.handler", "bootstrap" (for 'provided' runtimes), etc.
    ///
    /// # Arguments
    ///
    /// * `handler` - The function handler.
    pub fn handler<T: Into<String>>(self, handler: T) -> FunctionBuilder<ZipStateWithHandler> {
        FunctionBuilder {
            id: self.id,
            handler: Some(handler.into()),
            state: Default::default(),
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            code: self.code,
            runtime: self.runtime,
            additional_policies: self.additional_policies,
            aws_services_in_dependencies: self.aws_services_in_dependencies,
            env_vars: self.env_vars,
            function_name: self.function_name,
            sqs_event_source_mapping: self.sqs_event_source_mapping,
            reserved_concurrent_executions: self.reserved_concurrent_executions,
            log_group: self.log_group,
        }
    }
}

impl FunctionBuilder<ZipStateWithHandler> {
    /// Sets the functions runtime.
    ///
    /// # Arguments
    ///
    /// * `runtime` - The runtime environment for the function.
    pub fn runtime(self, runtime: Runtime) -> FunctionBuilder<ZipStateWithHandlerAndRuntime> {
        FunctionBuilder {
            id: self.id,
            runtime: Some(runtime),
            state: Default::default(),
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            code: self.code,
            handler: self.handler,
            additional_policies: self.additional_policies,
            aws_services_in_dependencies: self.aws_services_in_dependencies,
            env_vars: self.env_vars,
            function_name: self.function_name,
            sqs_event_source_mapping: self.sqs_event_source_mapping,
            reserved_concurrent_executions: self.reserved_concurrent_executions,
            log_group: self.log_group,
        }
    }
}

impl FunctionBuilder<ZipStateWithHandlerAndRuntime> {
    /// Configures the function to be triggered by an SQS queue.
    ///
    /// Automatically adds the necessary IAM permissions for reading from the queue.
    pub fn sqs_event_source_mapping(
        mut self,
        sqs_queue: &QueueRef,
        max_concurrency: Option<SqsEventSourceMaxConcurrency>,
    ) -> FunctionBuilder<EventSourceMappingState> {
        self.additional_policies.push(IamPermission::SqsRead(sqs_queue).into_policy());

        let mapping = EventSourceMappingInfo {
            id: sqs_queue.get_resource_id().to_string(),
            max_concurrency: max_concurrency.map(|c| c.0),
        };

        FunctionBuilder {
            id: self.id,
            sqs_event_source_mapping: Some(mapping),
            state: Default::default(),
            runtime: self.runtime,
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            code: self.code,
            handler: self.handler,
            additional_policies: self.additional_policies,
            aws_services_in_dependencies: self.aws_services_in_dependencies,
            env_vars: self.env_vars,
            function_name: self.function_name,
            reserved_concurrent_executions: self.reserved_concurrent_executions,
            log_group: self.log_group,
        }
    }

    /// Builds the Lambda function and adds it to the stack.
    ///
    /// Creates the function along with its IAM execution role and CloudWatch log group.
    /// Returns references to all three resources.
    pub fn build(self, stack_builder: &mut StackBuilder) -> (FunctionRef, RoleRef, LogGroupRef) {
        self.build_internal(stack_builder)
    }
}

impl FunctionBuilder<EventSourceMappingState> {
    /// Builds the Lambda function and adds it to the stack.
    ///
    /// Creates the function along with its IAM execution role, CloudWatch log group, and event source mapping.
    /// Returns references to the function, role, and log group.
    pub fn build(self, stack_builder: &mut StackBuilder) -> (FunctionRef, RoleRef, LogGroupRef) {
        self.build_internal(stack_builder)
    }
}

/// Builder for Lambda function permissions.
///
/// Creates permission resources that allow other AWS services to invoke a Lambda function.
pub struct PermissionBuilder {
    id: Id,
    action: String,
    function_name: Value,
    principal: String,
    source_arn: Option<Value>,
    source_account: Option<Value>,
}

impl PermissionBuilder {
    /// Creates a new permission builder for a Lambda function.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the permission resource
    /// * `action` - Lambda action to allow (e.g., "lambda:InvokeFunction")
    /// * `function_name` - Reference to the Lambda function
    /// * `principal` - AWS service or account that will be granted permission
    pub fn new<R: Into<String>>(id: &str, action: LambdaPermissionAction, function_name: Value, principal: R) -> Self {
        Self {
            id: Id(id.to_string()),
            action: action.0,
            function_name,
            principal: principal.into(),
            source_arn: None,
            source_account: None,
        }
    }

    pub fn source_arn(self, arn: Value) -> Self {
        Self {
            source_arn: Some(arn),
            ..self
        }
    }

    pub fn current_account(self) -> Self {
        Self {
            source_account: Some(get_ref("AWS::AccountId")),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> PermissionRef {
        let permission_resource_id = Resource::generate_id("LambdaPermission");

        stack_builder.add_resource(Permission {
            id: self.id.clone(),
            resource_id: permission_resource_id.clone(),
            r#type: PermissionType::PermissionType,
            properties: LambdaPermissionProperties {
                action: self.action,
                function_name: self.function_name,
                principal: self.principal,
                source_arn: self.source_arn,
                source_account: self.source_account,
            },
        });

        PermissionRef::internal_new(self.id, permission_resource_id)
    }
}

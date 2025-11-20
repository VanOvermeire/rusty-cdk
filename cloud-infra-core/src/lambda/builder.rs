use crate::iam::{AssumeRolePolicyDocumentBuilder, Effect, RoleBuilder, RolePropertiesBuilder, Permission as IamPermission, Policy, StatementBuilder, PrincipalBuilder, map_toml_dependencies_to_services, find_missing_services, RoleRef};
use crate::intrinsic_functions::{get_arn, get_ref, join};
use crate::lambda::{Environment, EventSourceMapping, EventSourceProperties, LambdaCode, Function, LambdaFunctionProperties, Permission, LambdaPermissionProperties, LoggingInfo, ScalingConfig, FunctionRef, PermissionRef};
use crate::sqs::{QueueRef};
use crate::stack::{Asset, Resource, StackBuilder};
use crate::wrappers::{Bucket, EnvVarKey, LogGroupName, Memory, RetentionInDays, SqsEventSourceMaxConcurrency, StringWithOnlyAlphaNumericsUnderscoresAndHyphens, Timeout, TomlFile, ZipFile};
use serde_json::Value;
use std::marker::PhantomData;
use std::vec;
use crate::cloudwatch::{LogGroupBuilder, LogGroupRef};
use crate::shared::Id;

pub enum Runtime {
    NodeJs22,
    Java21,
    Python313,
    ProvidedAl2023
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

pub enum Architecture {
    X86_64,
    ARM64
}

impl From<Architecture> for String {
    fn from(value: Architecture) -> Self {
        match value {
            Architecture::X86_64 => "x86_64".to_string(),
            Architecture::ARM64 => "arm64".to_string(),
        }
    }
}

pub enum PackageType {
    Image,
    Zip,
}

impl From<PackageType> for String {
    fn from(value: PackageType) -> Self {
        match value {
            PackageType::Image => "Image".to_string(),
            PackageType::Zip => "Zip".to_string()
        }
    }
}

pub struct Zip {
    bucket: String,
    file: ZipFile,
}

impl Zip {
    pub fn new(bucket: Bucket, file: ZipFile) -> Self {
        Zip {
            bucket: bucket.0,
            file
        }
    }
}

pub enum Code {
    Zip(Zip)
}

pub trait FunctionBuilderState {}

pub struct StartState {}
impl FunctionBuilderState for StartState {}
pub struct ZipState {}
impl FunctionBuilderState for ZipState {}
pub struct ZipStateWithHandler {}
impl FunctionBuilderState for ZipStateWithHandler {}
pub struct ZipStateWithHandlerAndRuntime {}
impl FunctionBuilderState for ZipStateWithHandlerAndRuntime {}
pub struct EventSourceMappingState {}
impl FunctionBuilderState for EventSourceMappingState {}

struct EventSourceMappingInfo {
    id: String,
    max_concurrency: Option<u16>,
}

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
}

impl<T: FunctionBuilderState> FunctionBuilder<T> {
    pub fn function_name(self, name: StringWithOnlyAlphaNumericsUnderscoresAndHyphens) -> FunctionBuilder<T> {
        Self {
            function_name: Some(name.0),
            ..self
        }
    }

    pub fn permissions(mut self, permission: IamPermission) -> FunctionBuilder<T> {
        self.additional_policies.push(permission.into_policy());
        Self {
            ..self
        }
    }

    // TODO macro
    pub fn check_permissions_against_dependencies(self, cargo_toml: TomlFile) -> Self {
        let services = map_toml_dependencies_to_services(cargo_toml.0.as_ref());
        
        Self {
            aws_services_in_dependencies: services,
            ..self
        }
    }
    
    pub fn env_var(mut self, key: EnvVarKey, value: Value) -> FunctionBuilder<T> {
        self.env_vars.push((key.0, value));
        Self {
            ..self
        }
    }

    pub fn env_var_string<V: Into<String>>(mut self, key: EnvVarKey, value: V) -> FunctionBuilder<T> {
        self.env_vars.push((key.0, Value::String(value.into())));
        Self {
            ..self
        }
    }

    pub fn reserved_concurrent_executions(self, executions: u32) -> FunctionBuilder<T> {
        Self {
            reserved_concurrent_executions: Some(executions),
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
                };

                (asset, code)
            }
        };

        if let Some(mapping) = self.sqs_event_source_mapping {
            let event_id = Id::generate_id(&self.id, "ESM");
            let event_resource_id = format!("EventSourceMapping{}", function_resource_id);
            let event_source_mapping = EventSourceMapping {
                id: event_id,
                resource_id: event_resource_id.clone(),
                r#type: "AWS::Lambda::EventSourceMapping".to_string(),
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
        let assumed_role_policy_document = AssumeRolePolicyDocumentBuilder::new(vec![assume_role_statement]);
        let managed_policy_arns = vec![join("", vec![Value::String("arn:".to_string()), get_ref("AWS::Partition"), Value::String(":iam::aws:policy/service-role/AWSLambdaBasicExecutionRole".to_string())])];
        let potentially_missing = find_missing_services(&self.aws_services_in_dependencies, &self.additional_policies);
        let props = RolePropertiesBuilder::new(assumed_role_policy_document, managed_policy_arns).policies(self.additional_policies).build();
        
        let role_id = Id::generate_id(&self.id, "Role");
        let role_resource_id = Resource::generate_id("LambdaFunctionRole");
        let role_ref = get_arn(&role_resource_id);
        let role = RoleBuilder::new_with_missing_info(&role_id, &role_resource_id, props, potentially_missing).build(stack_builder);

        let environment = if self.env_vars.is_empty() {
            None
        } else {
            Some(Environment {
                variables: self.env_vars.into_iter().collect(),
            })
        };

        let log_group_id = Id::generate_id(&self.id, "LogGroup");
        let log_group_name = self.function_name.clone().map(|fun_name| format!("/aws/lambda/{fun_name}"));
        let base_builder = LogGroupBuilder::new(&log_group_id)
            .log_group_retention(RetentionInDays(731));
        let log_group = if let Some(name) = log_group_name {
            base_builder.log_group_name_string(LogGroupName(name)).build(stack_builder)
        } else {
            base_builder.build(stack_builder)
        };
        
        let logging_info = LoggingInfo { log_group: Some( get_ref(log_group.get_resource_id())) };

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
            r#type: "AWS::Lambda::Function".to_string(),
            properties,
        });

        let function = FunctionRef::new(self.id, function_resource_id);

        (
            function,
            role,
            log_group,
        )
    }
}

impl FunctionBuilder<StartState> {
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
        }
    }

    pub fn zip(self, zip: Zip) -> FunctionBuilder<ZipState> {
        FunctionBuilder {
            code: Some(Code::Zip(zip)),
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
        }
    }
}

impl FunctionBuilder<ZipState> {
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
        }
    }
}

impl FunctionBuilder<ZipStateWithHandler> {
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
        }
    }
}

impl FunctionBuilder<ZipStateWithHandlerAndRuntime> {
    pub fn sqs_event_source_mapping(mut self, sqs_queue: &QueueRef, max_concurrency: Option<SqsEventSourceMaxConcurrency>) -> FunctionBuilder<EventSourceMappingState>  {
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
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> (FunctionRef, RoleRef, LogGroupRef) {
        self.build_internal(stack_builder)
    }
}

impl FunctionBuilder<EventSourceMappingState> {
    pub fn build(self, stack_builder: &mut StackBuilder) -> (FunctionRef, RoleRef, LogGroupRef) {
        self.build_internal(stack_builder)
    }
}

pub struct PermissionBuilder {
    id: Id,
    action: String, // TODO should start with 'lambda:' => macro
    function_name: Value,
    principal: String,
    source_arn: Option<Value>,
}

impl PermissionBuilder {
    pub fn new<T: Into<String>, R: Into<String>>(id: &str, action: T, function_name: Value, principal: R) -> Self {
        Self {
            id: Id(id.to_string()),
            action: action.into(),
            function_name,
            principal: principal.into(),
            source_arn: None,
        }
    }

    pub fn source_arn(self, arn: Value) -> Self {
        Self {
            source_arn: Some(arn),
            ..self
        }
    }

    pub fn build(self , stack_builder: &mut StackBuilder) -> PermissionRef {
        let permission_resource_id = Resource::generate_id("LambdaPermission");
        let properties = LambdaPermissionProperties {
            action: self.action,
            function_name: self.function_name,
            principal: self.principal,
            source_arn: self.source_arn,
        };

        stack_builder.add_resource(Permission {
            id: self.id,
            resource_id: permission_resource_id.clone(),
            r#type: "AWS::Lambda::Permission".to_string(),
            properties,
        });
        
        PermissionRef::new(permission_resource_id)
    }
}

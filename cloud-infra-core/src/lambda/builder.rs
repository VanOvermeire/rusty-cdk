use crate::iam::{AssumeRolePolicyDocumentBuilder, Effect, IamRole, IamRoleBuilder, IamRolePropertiesBuilder, Permission, Policy, PolicyBuilder, PolicyDocumentBuilder, Principal, StatementBuilder};
use crate::intrinsic_functions::{get_arn, get_ref, join};
use crate::lambda::{Environment, EventSourceMapping, EventSourceProperties, LambdaCode, LambdaFunction, LambdaFunctionProperties};
use crate::sqs::SqsQueue;
use crate::stack::{Asset, Resource};
use crate::wrappers::{EnvVarKey, Memory, StringWithOnlyAlphaNumericsAndUnderscores, Timeout, ZipFile};
use serde_json::Value;
use std::marker::PhantomData;
use std::vec;

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
    pub fn new(bucket: &str, file: ZipFile) -> Self {
        Zip {
            bucket: bucket.to_string(),
            file
        }
    }
}

pub enum Code {
    Zip(Zip)
}

pub trait LambdaFunctionBuilderState {}

pub struct StartState {}
impl LambdaFunctionBuilderState for StartState {}
pub struct ZipState {}
impl LambdaFunctionBuilderState for ZipState {}
pub struct ZipStateWithHandler {}
impl LambdaFunctionBuilderState for ZipStateWithHandler {}
pub struct ZipStateWithHandlerAndRuntime {}
impl LambdaFunctionBuilderState for ZipStateWithHandlerAndRuntime {}
pub struct EventSourceMappingState {}
impl LambdaFunctionBuilderState for EventSourceMappingState {}

pub struct LambdaFunctionBuilder<T: LambdaFunctionBuilderState> {
    state: PhantomData<T>,
    architecture: Architecture,
    memory: u16,
    timeout: u16,
    code: Option<Code>,
    handler: Option<String>,
    runtime: Option<Runtime>,
    additional_policies: Vec<Policy>,
    env_vars: Vec<(String, Value)>,
    function_name: Option<String>,
    sqs_event_source_mapping: Option<String>
}

impl<T: LambdaFunctionBuilderState> LambdaFunctionBuilder<T> {
    pub fn add_permission_to_role(mut self, permission: Permission) -> LambdaFunctionBuilder<T> {
        self.additional_policies.push(permission.into_policy());
        Self {
            ..self
        }
    }
    
    pub fn function_name(self, name: StringWithOnlyAlphaNumericsAndUnderscores) -> LambdaFunctionBuilder<T> {
        Self {
            function_name: Some(name.0),
            ..self
        }
    }
    
    pub fn add_env_var(mut self, key: EnvVarKey, value: Value) -> LambdaFunctionBuilder<T> {
        self.env_vars.push((key.0, value));
        Self {
            ..self
        }
    }

    pub fn add_env_var_string(mut self, key: EnvVarKey, value: String) -> LambdaFunctionBuilder<T> {
        self.env_vars.push((key.0, Value::String(value)));
        Self {
            ..self
        }
    }
    
    fn build_internal(mut self) -> (LambdaFunction, IamRole, Option<EventSourceMapping>) {
        let id = Resource::generate_id("LambdaFunction");
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

        let mapping = if let Some(sqs_queue_id) = self.sqs_event_source_mapping {
            let sqs_permissions_statement = StatementBuilder::new(vec![
                "sqs:ChangeMessageVisibility".to_string(),
                "sqs:DeleteMessage".to_string(),
                "sqs:GetQueueAttributes".to_string(),
                "sqs:GetQueueUrl".to_string(),
                "sqs:ReceiveMessage".to_string(),
            ], Effect::Allow)
                .resources(vec![get_arn(&sqs_queue_id)])
                .build();
            let policy = PolicyBuilder::new("example".to_string(), PolicyDocumentBuilder::new(vec![sqs_permissions_statement])).build();
            self.additional_policies.push(policy);

            let event_source_mapping = EventSourceMapping {
                id: format!("EventSourceMapping{}", id),
                r#type: "AWS::Lambda::EventSourceMapping".to_string(),
                properties: EventSourceProperties {
                    event_source_arn: Some(get_arn(&sqs_queue_id)),
                    function_name: Some(get_ref(&id)),
                },
            };
            Some(event_source_mapping)
        } else {
            None
        };

        let assume_role_statement = StatementBuilder::new(vec!["sts:AssumeRole".to_string()], Effect::Allow).principal(Principal {
            service: "lambda.amazonaws.com".to_string(),
        }).build();
        let assumed_role_policy_document = AssumeRolePolicyDocumentBuilder::new(vec![assume_role_statement]);
        let managed_policy_arns = vec![join("", vec![Value::String("arn:".to_string()), get_ref("AWS::Partition"), Value::String(":iam::aws:policy/service-role/AWSLambdaBasicExecutionRole".to_string())])];
        let props = IamRolePropertiesBuilder::new(assumed_role_policy_document, managed_policy_arns).policies(self.additional_policies).build();

        let role_id = Resource::generate_id("LambdaFunctionRole");
        let role_ref = get_arn(&role_id);
        let role = IamRoleBuilder::new(role_id, props);

        let environment = if self.env_vars.is_empty() {
            None
        } else {
            Some(Environment {
                variables: self.env_vars.into_iter().collect(),
            })
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
        };

        let function = LambdaFunction {
            id,
            asset: code.0,
            r#type: "AWS::Lambda::Function".to_string(),
            properties,
        };
        
        (
            function,
            role,
            mapping,
        )
    }
}

impl LambdaFunctionBuilder<StartState> {
    pub fn new(architecture: Architecture, memory: Memory, timeout: Timeout) -> LambdaFunctionBuilder<StartState> {
        LambdaFunctionBuilder {
            architecture,
            memory: memory.0,
            timeout: timeout.0,
            state: Default::default(),
            code: None,
            handler: None,
            runtime: None,
            additional_policies: vec![],
            env_vars: vec![],
            function_name: None,
            sqs_event_source_mapping: None,
        }
    }

    pub fn zip(self, zip: Zip) -> LambdaFunctionBuilder<ZipState> {
        LambdaFunctionBuilder {
            code: Some(Code::Zip(zip)),
            state: Default::default(),
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            handler: self.handler,
            runtime: self.runtime,
            additional_policies: self.additional_policies,
            env_vars: self.env_vars,
            function_name: self.function_name,
            sqs_event_source_mapping: self.sqs_event_source_mapping,
        }
    }
}

impl LambdaFunctionBuilder<ZipState> {
    pub fn handler(self, handler: String) -> LambdaFunctionBuilder<ZipStateWithHandler> {
        LambdaFunctionBuilder {
            handler: Some(handler),
            state: Default::default(),
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            code: self.code,
            runtime: self.runtime,
            additional_policies: self.additional_policies,
            env_vars: self.env_vars,
            function_name: self.function_name,
            sqs_event_source_mapping: self.sqs_event_source_mapping,
        }
    }
}

impl LambdaFunctionBuilder<ZipStateWithHandler> {
    pub fn runtime(self, runtime: Runtime) -> LambdaFunctionBuilder<ZipStateWithHandlerAndRuntime> {
        LambdaFunctionBuilder {
            runtime: Some(runtime),
            state: Default::default(),
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            code: self.code,
            handler: self.handler,
            additional_policies: self.additional_policies,
            env_vars: self.env_vars,
            function_name: self.function_name,
            sqs_event_source_mapping: self.sqs_event_source_mapping,
        }
    }
}

impl LambdaFunctionBuilder<ZipStateWithHandlerAndRuntime> {
    pub fn sqs_event_source_mapping(self, sqs_queue: &SqsQueue) -> LambdaFunctionBuilder<EventSourceMappingState>  {
        LambdaFunctionBuilder {
            sqs_event_source_mapping: Some(sqs_queue.get_id().to_string()),
            state: Default::default(),
            runtime: self.runtime,
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            code: self.code,
            handler: self.handler,
            additional_policies: self.additional_policies,
            env_vars: self.env_vars,
            function_name: self.function_name,
        }
    }

    #[must_use]
    pub fn build(self) -> (LambdaFunction, IamRole) {
        let (lambda, iam_role, _) = self.build_internal();
        (lambda, iam_role)
    }
}

impl LambdaFunctionBuilder<EventSourceMappingState> {
    #[must_use]
    pub fn build(self) -> (LambdaFunction, IamRole, EventSourceMapping) {
        let (lambda, iam_role, mapping) = self.build_internal();
        (lambda, iam_role, mapping.expect("should be Some because we are in the event source mapping state"))
    }
}
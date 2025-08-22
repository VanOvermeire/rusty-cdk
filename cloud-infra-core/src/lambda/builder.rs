use crate::iam::{AssumeRolePolicyDocumentBuilder, IamRole, IamRoleBuilder, IamRoleProperties, Permission, Policy, Principal, Statement};
use crate::intrinsic_functions::{get_arn, get_ref, join};
use crate::lambda::{Environment, LambdaCode, LambdaFunction, LambdaFunctionProperties};
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

// TODO check bucket! once checked, store the result in a local json file so you don't need to check again
//  also allow override with CLOUD_INFRA_NO_REMOTE => accept bucket as correct
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

pub struct LambdaFunctionBuilder<T: LambdaFunctionBuilderState> {
    state: PhantomData<T>,
    architecture: Architecture,
    memory: u16,
    timeout: u16,
    code: Option<Code>,
    handler: Option<String>,
    runtime: Option<Runtime>,
    additional_policies: Vec<Policy>,
    env_vars: Vec<(String, String)>,
    function_name: Option<String>
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
    
    pub fn add_env_var(mut self, key: EnvVarKey, value: String) -> LambdaFunctionBuilder<T> {
        self.env_vars.push((key.0, value));
        Self {
            ..self
        }
    }
    
    fn build_internal(self) -> (LambdaFunction, IamRole) {
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
        
        let assumed_role_policy_document = AssumeRolePolicyDocumentBuilder::new(vec![Statement {
            action: vec!["sts:AssumeRole".to_string()],
            effect: "Allow".to_string(),
            principal: Some(Principal {
                service: "lambda.amazonaws.com".to_string(),
            }),
            resource: None,
        }]);

        let managed_policy_arns = vec![join("", vec![Value::String("arn:".to_string()), get_ref("AWS::Partition"), Value::String(":iam::aws:policy/service-role/AWSLambdaBasicExecutionRole".to_string())])];
        let props = IamRoleProperties {
            assumed_role_policy_document,
            managed_policy_arns,
            policies: Some(self.additional_policies),
            role_name: None,
        };

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

        (
            LambdaFunction::new(Resource::generate_id("LambdaFunction"), code.0, properties),
            role,
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
        }
    }
}

impl LambdaFunctionBuilder<ZipStateWithHandlerAndRuntime> {
    pub fn build(self) -> (LambdaFunction, IamRole) {
        self.build_internal()
    }
}
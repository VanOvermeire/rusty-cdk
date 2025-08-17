// TODO set rights
//  link Env var

use std::marker::PhantomData;
use std::vec;
use crate::dynamodb::DynamoDBTable;
use crate::intrinsic_functions::{get_arn, join};
use crate::iam::{AssumeRolePolicyDocument, IamRole, IamRoleProperties, Principal, Statement};
use crate::lambda::{LambdaCode, LambdaFunction, LambdaFunctionProperties};
use crate::stack::Resource;

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

// TODO could check bucket and dir validity with macros
pub struct Zip {
    bucket: String,
    dir: String,
}

impl Zip {
    pub fn new(bucket: &str, dir: &str) -> Self {
        Zip {
            bucket: bucket.to_string(),
            dir: dir.to_string(),
        }
    }
}

pub enum Code {
    Zip(Zip)
}

pub enum Permission {
    DynamoDBRead(DynamoDBTable),
    DynamoDBReadWrite(DynamoDBTable),
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
    permissions: Vec<Permission>,
}

impl<T: LambdaFunctionBuilderState> LambdaFunctionBuilder<T> {
    fn build_internal(self) -> Vec<Resource> {
        let code = match self.code.expect("code to be present, enforced by builder") {
            Code::Zip(z) => {
                LambdaCode {
                    s3_bucket: Some(z.bucket),
                    s3_key: Some("generate.zip".to_string()), // TODO!
                }
            }
        };
        
        let assumed_role_policy_document = AssumeRolePolicyDocument::new(vec![Statement {
            action: "sts:AssumeRole".to_string(),
            effect: "Allow".to_string(),
            principal: Principal {
                service: "lambda.amazonaws.com".to_string(),
            }
        }]);
        let props = IamRoleProperties {
            assumed_role_policy_document,
            managed_policy_arns: vec![join()],
        };

        let role_id = Resource::generate_id("LambdaFunctionRole");
        let role_ref = get_arn(&role_id);
        let role = IamRole::new(role_id, props);
        
        let properties = LambdaFunctionProperties {
            code,
            architectures: vec![self.architecture.into()],
            memory_size: self.memory,
            timeout: self.timeout,
            handler: self.handler,
            runtime: self.runtime.map(Into::into),
            role: role_ref,
        };
        
        vec![
            Resource::LambdaFunction(LambdaFunction::new(Resource::generate_id("LambdaFunction"), properties)),
            Resource::IamRole(role),
        ]
    }
}

// TODO better checking for mem and duration
impl LambdaFunctionBuilder<StartState> {
    pub fn new(architecture: Architecture, memory: u16, timeout: u16) -> LambdaFunctionBuilder<StartState> {
        LambdaFunctionBuilder {
            architecture,
            memory,
            timeout,
            state: Default::default(),
            code: None,
            handler: None,
            runtime: None,
            permissions: vec![],
        }
    }
    
    // TODO other optional config
    
    pub fn add_permission_to_role(mut self, permissions: Vec<Permission>) -> LambdaFunctionBuilder<StartState> {
        self.permissions.extend(permissions);
        Self {
            permissions: self.permissions,
            state: Default::default(),
            code: self.code,
            architecture: self.architecture,
            memory: self.memory,
            timeout: self.timeout,
            handler: self.handler,
            runtime: self.runtime,
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
            permissions: self.permissions,
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
            permissions: self.permissions,
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
            permissions: self.permissions,
        }
    }
}

impl LambdaFunctionBuilder<ZipStateWithHandlerAndRuntime> {
    pub fn build(self) -> Vec<Resource> {
        self.build_internal()
    }
}
// TODO set rights
//  link Env var

use std::marker::PhantomData;
use crate::dynamodb::DynamoDBTable;

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

// TODO better checking for mem and duration
impl LambdaFunctionBuilder<StartState> {
    pub fn new(architecture: Architecture, memory: u16, timeout: u16) -> Self {
        Self {
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
    pub fn handler(self, runtime: Runtime) -> LambdaFunctionBuilder<ZipStateWithHandlerAndRuntime> {
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
    pub fn build(self) {
        
    }
}
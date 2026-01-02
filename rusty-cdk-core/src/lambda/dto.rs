use crate::{dto_methods, ref_struct};
use crate::shared::Id;
use crate::stack::Asset;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use crate::intrinsic::{get_arn, get_att, get_ref};

// this one also needs the `id` field for some custom ids used by API Gateway and subscriptions
pub struct FunctionRef {
    id: Id,
    resource_id: String,
}

impl FunctionRef {
    pub fn new(id: Id, resource_id: String) -> Self {
        Self { id, resource_id }
    }
    
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_ref(&self) -> Value {
        crate::intrinsic::get_ref(self.get_resource_id())
    }

    pub fn get_arn(&self) -> Value {
        crate::intrinsic::get_arn(self.get_resource_id())
    }

    pub fn get_att(&self, id: &str) -> Value {
        crate::intrinsic::get_att(self.get_resource_id(), id)
    }
}

#[derive(Debug, Serialize)]
pub struct Function {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(skip)]
    pub(crate) asset: Option<Asset>,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: LambdaFunctionProperties,
}

impl Function {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct LambdaFunctionProperties {
    #[serde(rename = "Code")]
    pub(super) code: LambdaCode,
    #[serde(rename = "MemorySize")]
    pub(super) memory_size: u16,
    #[serde(rename = "Timeout")]
    pub(super) timeout: u16,
    #[serde(rename = "Architectures")]
    pub(super) architectures: Vec<String>,
    #[serde(rename = "Role")]
    pub(super) role: Value,
    #[serde(rename = "Runtime", skip_serializing_if = "Option::is_none")]
    pub(super) runtime: Option<String>,
    #[serde(rename = "Handler", skip_serializing_if = "Option::is_none")]
    pub(super) handler: Option<String>,
    #[serde(rename = "FunctionName", skip_serializing_if = "Option::is_none")]
    pub(super) function_name: Option<String>,
    #[serde(rename = "Environment", skip_serializing_if = "Option::is_none")]
    pub(super) environment: Option<Environment>,
    #[serde(rename = "ReservedConcurrentExecutions", skip_serializing_if = "Option::is_none")]
    pub(super) reserved_concurrent_executions: Option<u32>,
    #[serde(rename = "LoggingConfig")]
    pub(super) logging_info: LoggingInfo, 
    // package_type: Option<String>,
    // "VpcConfig": VpcConfig
}

#[derive(Debug, Serialize)]
pub struct LambdaCode {
    #[serde(rename = "S3Bucket", skip_serializing_if = "Option::is_none")]
    pub(super) s3_bucket: Option<String>,
    #[serde(rename = "S3Key", skip_serializing_if = "Option::is_none")]
    pub(super) s3_key: Option<String>,
    #[serde(rename = "ZipFile", skip_serializing_if = "Option::is_none")]
    pub(super) zipfile: Option<String>,
    // s3_object_version: Option<String>,
    // image_uri: Option<String>,
    // source_kmskey_arn: String
}

#[derive(Debug, Serialize)]
pub struct Environment {
    #[serde(rename = "Variables")]
    pub(super) variables: HashMap<String, Value>,
}

#[derive(Debug, Serialize)]
pub struct LoggingInfo {
    #[serde(rename = "LogGroup")]
    pub(super) log_group: Option<Value>,
    // "ApplicationLogLevel" : String,
    // "LogFormat" : String,
    // "SystemLogLevel" : String
}

ref_struct!(EventSourceMappingRef);

#[derive(Debug, Serialize)]
pub struct EventSourceMapping {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: EventSourceProperties,
}
dto_methods!(EventSourceMapping);

#[derive(Debug, Serialize)]
pub struct EventSourceProperties {
    #[serde(rename = "EventSourceArn", skip_serializing_if = "Option::is_none")]
    pub(super) event_source_arn: Option<Value>,
    #[serde(rename = "FunctionName", skip_serializing_if = "Option::is_none")]
    pub(super) function_name: Option<Value>,
    #[serde(rename = "ScalingConfig", skip_serializing_if = "Option::is_none")]
    pub(super) scaling_config: Option<ScalingConfig>,
}

#[derive(Debug, Serialize)]
pub struct ScalingConfig {
    #[serde(rename = "MaximumConcurrency")]
    pub(super) max_concurrency: u16,
}

pub struct PermissionRef {
    id: Id,
    resource_id: String,
}

impl PermissionRef {
    pub fn new(id: Id, resource_id: String) -> Self {
        Self {
            id,
            resource_id
        }
    }
    
    pub fn get_id(&self) -> Id {
        self.id.clone()
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }
    
    pub fn get_arn(&self) -> Value {
        get_arn(self.get_resource_id())
    }
    
    pub fn get_att(&self, id: &str) -> Value {
        get_att(self.get_resource_id(), id)
    }
}

#[derive(Debug, Serialize)]
pub struct Permission {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: LambdaPermissionProperties,
}
dto_methods!(Permission);

#[derive(Debug, Serialize)]
pub struct LambdaPermissionProperties {
    #[serde(rename = "Action")]
    pub(super) action: String,
    #[serde(rename = "FunctionName")]
    pub(super) function_name: Value,
    #[serde(rename = "Principal")]
    pub(super) principal: String,
    #[serde(rename = "SourceArn", skip_serializing_if = "Option::is_none")]
    pub(super) source_arn: Option<Value>,
    #[serde(rename = "SourceAccount", skip_serializing_if = "Option::is_none")]
    pub(super) source_account: Option<Value>,
}

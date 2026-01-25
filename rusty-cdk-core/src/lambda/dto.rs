use crate::{dto_methods, ref_struct, ref_struct_with_id_methods};
use crate::shared::Id;
use crate::stack::Asset;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

ref_struct_with_id_methods!(FunctionRef);

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    #[serde(rename = "Variables")]
    pub(super) variables: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingInfo {
    #[serde(rename = "LogGroup")]
    pub(super) log_group: Option<Value>,
    // "ApplicationLogLevel" : String,
    // "LogFormat" : String,
    // "SystemLogLevel" : String
}

ref_struct!(EventSourceMappingRef);

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSourceProperties {
    #[serde(rename = "EventSourceArn", skip_serializing_if = "Option::is_none")]
    pub(super) event_source_arn: Option<Value>,
    #[serde(rename = "FunctionName", skip_serializing_if = "Option::is_none")]
    pub(super) function_name: Option<Value>,
    #[serde(rename = "ScalingConfig", skip_serializing_if = "Option::is_none")]
    pub(super) scaling_config: Option<ScalingConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalingConfig {
    #[serde(rename = "MaximumConcurrency")]
    pub(super) max_concurrency: u16,
}

ref_struct_with_id_methods!(PermissionRef);

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

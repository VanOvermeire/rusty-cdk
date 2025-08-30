use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;
use crate::stack::Asset;

#[derive(Debug, Serialize)]
pub struct LambdaFunction {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(skip)]
    pub(crate) referenced_ids: Vec<String>,
    #[serde(skip)]
    pub(crate) asset: Asset,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: LambdaFunctionProperties,
}

impl LambdaFunction {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
    
    pub fn get_referenced_ids(&self) -> Vec<&str> {
        self.referenced_ids.iter().map(|r| r.as_str()).collect()
    }
}

#[derive(Debug, Serialize)]
pub struct LambdaFunctionProperties {
    #[serde(rename = "Code")]
    pub(crate) code: LambdaCode,
    #[serde(rename = "MemorySize")]
    pub(crate) memory_size: u16,
    #[serde(rename = "Timeout")]
    pub(crate) timeout: u16,
    #[serde(rename = "Architectures")]
    pub(crate) architectures: Vec<String>,
    #[serde(rename = "Role")]
    pub(crate) role: Value,
    #[serde(rename = "Runtime", skip_serializing_if = "Option::is_none")]
    pub(crate) runtime: Option<String>,
    #[serde(rename = "Handler", skip_serializing_if = "Option::is_none")]
    pub(crate) handler: Option<String>,
    #[serde(rename = "FunctionName", skip_serializing_if = "Option::is_none")]
    pub(crate) function_name: Option<String>,
    #[serde(rename = "Environment", skip_serializing_if = "Option::is_none")]
    pub(crate) environment: Option<Environment>,
    #[serde(rename = "ReservedConcurrentExecutions", skip_serializing_if = "Option::is_none")]
    pub(crate) reserved_concurrent_executions: Option<u32>,
    #[serde(rename = "LoggingConfig")]
    pub(crate) logging_info: LoggingInfo
    // package_type: Option<String>,
    // "VpcConfig": VpcConfig
}

#[derive(Debug, Serialize)]
pub struct LambdaCode {
    #[serde(rename = "S3Bucket")]
    pub(crate) s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    pub(crate) s3_key: Option<String>,
    // s3_object_version: Option<String>,
    // zipfile: Option<String>,
    // image_uri: Option<String>,
    // source_kmskey_arn: String
}

#[derive(Debug, Serialize)]
pub struct Environment {
    #[serde(rename = "Variables")]
    pub(crate) variables: HashMap<String, Value>
}

#[derive(Debug, Serialize)]
pub struct LoggingInfo {
    #[serde(rename = "LogGroup")]
    pub(crate) log_group: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct EventSourceMapping {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: EventSourceProperties,
}

impl EventSourceMapping {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct EventSourceProperties {
    #[serde(rename = "EventSourceArn", skip_serializing_if = "Option::is_none")]
    pub(crate) event_source_arn: Option<Value>,
    #[serde(rename = "FunctionName", skip_serializing_if = "Option::is_none")]
    pub(crate) function_name: Option<Value>,
    #[serde(rename = "ScalingConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) scaling_config: Option<ScalingConfig>,
}

#[derive(Debug, Serialize)]
pub struct ScalingConfig {
    #[serde(rename = "MaximumConcurrency")]
    pub(crate) max_concurrency: u16,
}

#[derive(Debug, Serialize)]
pub struct LambdaPermission {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(skip)]
    pub(crate) referenced_ids: Vec<String>,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: LambdaPermissionProperties,
}

impl LambdaPermission {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_referenced_ids(&self) -> Vec<&str> {
        self.referenced_ids.iter().map(|r| r.as_str()).collect()
    }
}

// TODO add and use builder
#[derive(Debug, Serialize)]
pub struct LambdaPermissionProperties {
    #[serde(rename = "Action")]
    pub(crate) action: String,
    #[serde(rename = "FunctionName")]
    pub(crate) function_name: Value,
    #[serde(rename = "Principal")]
    pub(crate) principal: String,
    #[serde(rename = "SourceArn", skip_serializing_if = "Option::is_none")]
    pub(crate) source_arn: Option<Value>
}

use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;
use crate::stack::Asset;

#[derive(Debug, Serialize)]
pub struct LambdaFunction {
    #[serde(skip)]
    pub(crate) id: String,
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
    // #[serde(rename = "PackageType", skip_serializing_if = "Option::is_none")]
    // package_type: Option<String>,

    // "LoggingConfig": LoggingConfig,
    // "ReservedConcurrentExecutions": Integer,
    // "TracingConfig": TracingConfig,
    // "VpcConfig": VpcConfig
}

#[derive(Debug, Serialize)]
pub struct LambdaCode {
    #[serde(rename = "S3Bucket")]
    pub(crate) s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    pub(crate) s3_key: Option<String>,
    // #[serde(rename = "S3ObjectVersion")]
    // s3_object_version: Option<String>,
    // #[serde(rename = "ZipFile")]
    // zipfile: Option<String>,
    // #[serde(rename = "ImageUri")]
    // image_uri: Option<String>,
    // #[serde(rename = "SourceKMSKeyArn")]
    // source_kmskey_arn: String
}

#[derive(Debug, Serialize)]
pub struct Environment {
    #[serde(rename = "Variables")]
    pub(crate) variables: HashMap<String, Value>
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

#[derive(Debug, Serialize)]
pub struct EventSourceProperties {
    #[serde(rename = "EventSourceArn", skip_serializing_if = "Option::is_none")]
    pub(crate) event_source_arn: Option<Value>,
    #[serde(rename = "FunctionName", skip_serializing_if = "Option::is_none")]
    pub(crate) function_name: Option<Value>
}

impl EventSourceMapping {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

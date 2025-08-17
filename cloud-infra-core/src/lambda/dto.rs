use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;
use crate::stack::Asset;

#[derive(Serialize)]
pub struct LambdaFunction {
    #[serde(skip)]
    id: String,
    #[serde(skip)]
    pub(crate) asset: Asset,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(rename = "Properties")]
    properties: LambdaFunctionProperties,
}

impl LambdaFunction {
    pub(crate) fn new(id: String, asset: Asset, properties: LambdaFunctionProperties) -> Self {
        Self {
            id,
            asset,
            r#type: "AWS::Lambda::Function".to_string(),
            properties,
        }
    }

    pub(crate) fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Serialize)]
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
    // #[serde(rename = "FunctionName", skip_serializing_if = "Option::is_none")]
    // function_name: Option<String>,
    // #[serde(rename = "PackageType", skip_serializing_if = "Option::is_none")]
    // package_type: Option<String>,
    // #[serde(rename = "Environment", skip_serializing_if = "Option::is_none")]
    // environment: Option<Environment>,

    // "LoggingConfig": LoggingConfig,
    // "ReservedConcurrentExecutions": Integer,
    // "TracingConfig": TracingConfig,
    // "VpcConfig": VpcConfig
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct Environment {
    // TODO check env vars: [a-zA-Z][a-zA-Z0-9_]+
    pub(crate) variables: HashMap<String, String>
}
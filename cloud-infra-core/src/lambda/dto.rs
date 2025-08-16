use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct LambdaFunction {
    #[serde(skip)]
    id: String,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(rename = "Properties")]
    properties: LambdaFunctionProperties,
}

#[derive(Serialize)]
pub struct LambdaFunctionProperties {
    #[serde(rename = "Code")]
    code: LambdaCode,
    #[serde(rename = "MemorySize")]
    memory_size: u16,
    #[serde(rename = "Timeout")]
    timeout: u16,
    #[serde(rename = "Architectures")]
    architectures: Vec<String>,
    #[serde(rename = "Role")]
    role: String,
    #[serde(rename = "Runtime", skip_serializing_if = "Option::is_none")]
    runtime: Option<String>,
    #[serde(rename = "Handler", skip_serializing_if = "Option::is_none")]
    handler: Option<String>,
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
    s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    s3_key: Option<String>,
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
    variables: HashMap<String, String>
}
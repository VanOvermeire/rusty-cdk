use serde::Serialize;
use serde_json::Value;
use crate::{dto_methods, ref_struct};
use crate::shared::Id;

ref_struct!(ApiGatewayV2ApiRef);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2Api {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: ApiGatewayV2ApiProperties,
}
dto_methods!(ApiGatewayV2Api);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2ApiProperties {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub(super) name: Option<String>,
    #[serde(rename = "ProtocolType")]
    pub(super) protocol_type: String,
    #[serde(rename = "DisableExecuteApiEndpoint", skip_serializing_if = "Option::is_none")]
    pub(super) disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "CorsConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) cors_configuration: Option<CorsConfiguration>,
}

#[derive(Debug, Serialize)]
pub struct CorsConfiguration {
    #[serde(rename = "AllowCredentials", skip_serializing_if = "Option::is_none")]
    pub(super) allow_credentials: Option<bool>,
    #[serde(rename = "AllowHeaders", skip_serializing_if = "Option::is_none")]
    pub(super) allow_headers: Option<Vec<String>>,
    #[serde(rename = "AllowMethods", skip_serializing_if = "Option::is_none")]
    pub(super) allow_methods: Option<Vec<String>>,
    #[serde(rename = "AllowOrigins", skip_serializing_if = "Option::is_none")]
    pub(super) allow_origins: Option<Vec<String>>,
    #[serde(rename = "ExposeHeaders", skip_serializing_if = "Option::is_none")]
    pub(super) expose_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge", skip_serializing_if = "Option::is_none")]
    pub(super) max_age: Option<u64>,
}

ref_struct!(ApiGatewayV2StageRef);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2Stage {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: ApiGatewayV2StageProperties,
}
dto_methods!(ApiGatewayV2Stage);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2StageProperties {
    #[serde(rename = "ApiId")]
    pub(super) api_id: Value,
    #[serde(rename = "StageName")]
    pub(super) stage_name: String,
    #[serde(rename = "AutoDeploy")]
    pub(super) auto_deploy: bool,
    #[serde(rename = "DefaultRouteSettings", skip_serializing_if = "Option::is_none")]
    pub(super) default_route_settings: Option<RouteSettings>,
    #[serde(rename = "RouteSettings", skip_serializing_if = "Option::is_none")]
    pub(super) route_settings: Option<Value>,
}

ref_struct!(ApiGatewayV2IntegrationRef);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2Integration {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: ApiGatewayV2IntegrationProperties,
}
dto_methods!(ApiGatewayV2Integration);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2IntegrationProperties {
    #[serde(rename = "ApiId")]
    pub(super) api_id: Value,
    #[serde(rename = "IntegrationType")]
    pub(super) integration_type: String,
    #[serde(rename = "CredentialsArn", skip_serializing_if = "Option::is_none")]
    pub(super) integration_method: Option<String>,
    #[serde(rename = "IntegrationUri", skip_serializing_if = "Option::is_none")]
    pub(super) integration_uri: Option<Value>,
    #[serde(rename = "PassthroughBehavior", skip_serializing_if = "Option::is_none")]
    pub(super) passthrough_behavior: Option<String>,
    #[serde(rename = "PayloadFormatVersion", skip_serializing_if = "Option::is_none")]
    pub(super) payload_format_version: Option<String>,
    #[serde(rename = "RequestParameters", skip_serializing_if = "Option::is_none")]
    pub(super) request_parameters: Option<Value>,
    #[serde(rename = "RequestTemplates", skip_serializing_if = "Option::is_none")]
    pub(super) request_templates: Option<Value>,
    #[serde(rename = "ResponseParameters", skip_serializing_if = "Option::is_none")]
    pub(super) response_parameters: Option<Value>,
    #[serde(rename = "TimeoutInMillis", skip_serializing_if = "Option::is_none")]
    pub(super) timeout_in_millis: Option<u32>,
}

ref_struct!(ApiGatewayV2RouteRef);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2Route {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: ApiGatewayV2RouteProperties,
}
dto_methods!(ApiGatewayV2Route);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2RouteProperties {
    #[serde(rename = "ApiId")]
    pub(super) api_id: Value,
    #[serde(rename = "RouteKey")]
    pub(super) route_key: String,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub(super) target: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct RouteSettings {
    #[serde(rename = "ThrottlingBurstLimit", skip_serializing_if = "Option::is_none")]
    pub(super) throttling_burst_limit: Option<u32>,
    #[serde(rename = "ThrottlingRateLimit", skip_serializing_if = "Option::is_none")]
    pub(super) throttling_rate_limit: Option<f64>,
}

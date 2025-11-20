use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;
use crate::ref_struct;
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

impl ApiGatewayV2Api {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2ApiProperties {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(rename = "ProtocolType")]
    pub(crate) protocol_type: String,
    #[serde(rename = "DisableExecuteApiEndpoint", skip_serializing_if = "Option::is_none")]
    pub(crate) disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "CorsConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) cors_configuration: Option<CorsConfiguration>,
}

#[derive(Debug, Serialize)]
pub struct CorsConfiguration {
    #[serde(rename = "AllowCredentials", skip_serializing_if = "Option::is_none")]
    pub(crate) allow_credentials: Option<bool>,
    #[serde(rename = "AllowHeaders", skip_serializing_if = "Option::is_none")]
    pub(crate) allow_headers: Option<Vec<String>>,
    #[serde(rename = "AllowMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) allow_methods: Option<Vec<String>>,
    #[serde(rename = "AllowOrigins", skip_serializing_if = "Option::is_none")]
    pub(crate) allow_origins: Option<Vec<String>>,
    #[serde(rename = "ExposeHeaders", skip_serializing_if = "Option::is_none")]
    pub(crate) expose_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge", skip_serializing_if = "Option::is_none")]
    pub(crate) max_age: Option<u64>,
}

ref_struct!(ApiGatewayV2StageRef);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2Stage {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: ApiGatewayV2StageProperties,
}

impl ApiGatewayV2Stage {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }
}

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2StageProperties {
    #[serde(rename = "ApiId")]
    pub(crate) api_id: Value,
    #[serde(rename = "StageName")]
    pub(crate) stage_name: String,
    #[serde(rename = "AutoDeploy")]
    pub(crate) auto_deploy: bool,
    #[serde(rename = "DefaultRouteSettings", skip_serializing_if = "Option::is_none")]
    pub(crate) default_route_settings: Option<RouteSettings>,
    #[serde(rename = "RouteSettings", skip_serializing_if = "Option::is_none")]
    pub(crate) route_settings: Option<Value>,
}

ref_struct!(ApiGatewayV2IntegrationRef);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2Integration {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: ApiGatewayV2IntegrationProperties,
}

impl ApiGatewayV2Integration {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }
}

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2IntegrationProperties {
    #[serde(rename = "ApiId")]
    pub(crate) api_id: Value,
    #[serde(rename = "IntegrationType")]
    pub(crate) integration_type: String,
    #[serde(rename = "CredentialsArn", skip_serializing_if = "Option::is_none")]
    pub(crate) integration_method: Option<String>,
    #[serde(rename = "IntegrationUri", skip_serializing_if = "Option::is_none")]
    pub(crate) integration_uri: Option<Value>,
    #[serde(rename = "PassthroughBehavior", skip_serializing_if = "Option::is_none")]
    pub(crate) passthrough_behavior: Option<String>,
    #[serde(rename = "PayloadFormatVersion", skip_serializing_if = "Option::is_none")]
    pub(crate) payload_format_version: Option<String>,
    #[serde(rename = "RequestParameters", skip_serializing_if = "Option::is_none")]
    pub(crate) request_parameters: Option<Value>,
    #[serde(rename = "RequestTemplates", skip_serializing_if = "Option::is_none")]
    pub(crate) request_templates: Option<Value>,
    #[serde(rename = "ResponseParameters", skip_serializing_if = "Option::is_none")]
    pub(crate) response_parameters: Option<Value>,
    #[serde(rename = "TimeoutInMillis", skip_serializing_if = "Option::is_none")]
    pub(crate) timeout_in_millis: Option<u32>,
}

ref_struct!(ApiGatewayV2RouteRef);

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2Route {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: ApiGatewayV2RouteProperties,
}

impl ApiGatewayV2Route {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct ApiGatewayV2RouteProperties {
    #[serde(rename = "ApiId")]
    pub(crate) api_id: Value,
    #[serde(rename = "RouteKey")]
    pub(crate) route_key: String,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub(crate) target: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct RouteSettings {
    #[serde(rename = "ThrottlingBurstLimit", skip_serializing_if = "Option::is_none")]
    pub(crate) throttling_burst_limit: Option<u32>,
    #[serde(rename = "ThrottlingRateLimit", skip_serializing_if = "Option::is_none")]
    pub(crate) throttling_rate_limit: Option<f64>,
}

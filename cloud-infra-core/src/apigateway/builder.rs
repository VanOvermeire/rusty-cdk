use std::time::Duration;
use serde_json::Value;
use crate::apigateway::dto::{ApiGatewayV2Api, ApiGatewayV2ApiProperties, ApiGatewayV2Integration, ApiGatewayV2IntegrationProperties, ApiGatewayV2Route, ApiGatewayV2RouteProperties, ApiGatewayV2Stage, ApiGatewayV2StageProperties, CorsConfiguration};
use crate::intrinsic_functions::{get_arn, get_ref, join};
use crate::lambda::{LambdaFunction, LambdaPermission, LambdaPermissionProperties};
use crate::stack::Resource;

// TODO auth, websockets
// most of the websocket stuff left out, some things specific to http (cors), others for websocket (RouteSelectionExpression)

pub enum HttpMethod {
    Any,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Delete
}

impl From<HttpMethod> for String {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Any => "*".to_string(),
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Head => "HEAD".to_string(),
            HttpMethod::Options => "OPTIONS".to_string(),
            HttpMethod::Patch => "PATCH".to_string(),
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Put => "PUT".to_string(),
            HttpMethod::Delete => "DELETE".to_string()
        }
    }
}

pub struct HttpApiGatewayBuilder {
    name: Option<String>,
    disable_execute_api_endpoint: Option<bool>,
    cors_configuration: Option<CorsConfiguration>,
    route_info: Vec<(String, Option<HttpMethod>, String)>
}

impl HttpApiGatewayBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            disable_execute_api_endpoint: None,
            cors_configuration: None,
            route_info: vec![],
        }
    }
    
    pub fn name(self, name: String) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }
    
    pub fn disable_execute_api_endpoint(self, disable_api_endpoint: bool) -> Self {
        Self {
            disable_execute_api_endpoint: Some(disable_api_endpoint),
            ..self
        }
    }
    
    pub fn cors_configuration(self, config: CorsConfiguration) -> Self {
        Self {
            cors_configuration: Some(config),
            ..self
        }
    }
    
    pub fn add_default_route_lambda(mut self, lambda: &LambdaFunction) -> Self {
        self.route_info.push(("$default".to_string(), None, lambda.get_id().to_string()));
        Self {
            ..self
        }
    }
    
    pub fn add_route_lambda(mut self, path: String, method: HttpMethod, lambda: &LambdaFunction) -> Self {
        let path = if path.starts_with("/") {
            path
        } else {
            format!("/{}", path)  
        };
        
        self.route_info.push((path, Some(method), lambda.get_id().to_string()));
        Self {
            ..self
        }
    }
    
    pub fn build(self) -> (ApiGatewayV2Api, ApiGatewayV2Stage, Vec<(ApiGatewayV2Route, ApiGatewayV2Integration, LambdaPermission)>) {
        let api_id = Resource::generate_id("HttpApiGateway");
        let stage_id = Resource::generate_id("HttpApiStage");
        
        let routes: Vec<_> = self.route_info.into_iter().map(|info| {
            let integration_id = Resource::generate_id("HttpApiIntegration");
            let route_id = Resource::generate_id("HttpApiRoute");
            let permission_id = Resource::generate_id("LambdaPermission");

            let properties = LambdaPermissionProperties {
                action: "lambda:InvokeFunction".to_string(),
                function_name: get_arn(&info.2),
                principal: "apigateway.amazonaws.com".to_string(),
                source_arn: Some(
                    join("", vec![
                        Value::String("arn:".to_string()),
                        get_ref("AWS::Partition"),
                        Value::String(":execute-api:".to_string()),
                        get_ref("AWS::Region"),
                        Value::String(":".to_string()),
                        get_ref("AWS::AccountId"),
                        Value::String(":".to_string()),
                        get_ref(&api_id),
                        Value::String(format!("*/*{}", info.0)),
                    ])
                ),
            };
            
            let permission = LambdaPermission {
                id: permission_id,
                referenced_ids: vec![],
                r#type: "AWS::Lambda::Permission".to_string(),
                properties,
            };

            let properties = ApiGatewayV2IntegrationProperties {
                api_id: get_ref(&api_id),
                integration_type: "AWS_PROXY".to_string(),
                payload_format_version: Some("2.0".to_string()),
                integration_uri: Some(get_arn(&info.2)),
                integration_method: None,
                passthrough_behavior: None,
                request_parameters: None,
                request_templates: None,
                response_parameters: None,
                timeout_in_millis: None,
            };
            
            let integration = ApiGatewayV2Integration {
                id: integration_id.clone(),
                referenced_ids: vec![api_id.clone()],
                r#type: "AWS::ApiGatewayV2::Integration".to_string(),
                properties,
            };
            
            let route_key = if let Some(method) = info.1 {
                let method: String = method.into();
                format!("{} {}", method, info.0)
            } else {
                info.0  
            };
            
            let properties = ApiGatewayV2RouteProperties {
                api_id: get_ref(&api_id),
                route_key,
                target: Some(join("", vec![Value::String("integrations/".to_string()), get_ref(&integration_id)]))
            };
            
            let route = ApiGatewayV2Route {
                id: route_id,
                referenced_ids: vec![api_id.clone(), integration_id],
                r#type: "AWS::ApiGatewayV2::Route".to_string(),
                properties,
            };

            (route, integration, permission)
        }).collect();

        let properties = ApiGatewayV2StageProperties {
            api_id: get_ref(&api_id),
            stage_name: "$default".to_string(),
            auto_deploy: true,
            default_route_settings: None,
            route_settings: None,
        };
        
        let stage = ApiGatewayV2Stage {
            id: stage_id,
            r#type: "AWS::ApiGatewayV2::Stage".to_string(),
            properties,
        };
        
        let properties = ApiGatewayV2ApiProperties {
            name: self.name,
            protocol_type: "HTTP".to_string(),
            disable_execute_api_endpoint: self.disable_execute_api_endpoint,
            cors_configuration: self.cors_configuration,
        };
        
        let api = ApiGatewayV2Api {
            id: api_id,
            r#type: "AWS::ApiGatewayV2::Api".to_string(),
            properties,
        };

        (api, stage, routes)
    }
}

pub struct CorsConfigurationBuilder {
    allow_credentials: Option<bool>,
    allow_headers: Option<Vec<String>>,
    allow_methods: Option<Vec<String>>,
    allow_origins: Option<Vec<String>>,
    expose_headers: Option<Vec<String>>,
    max_age: Option<u64>,
}

impl CorsConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            allow_credentials: None,
            allow_headers: None,
            allow_methods: None,
            allow_origins: None,
            expose_headers: None,
            max_age: None,
        }
    }
    
    pub fn allow_credentials(self, allow: bool) -> Self {
        Self {
            allow_credentials: Some(allow),
            ..self
        }
    }
    
    pub fn allow_headers(self, headers: Vec<String>) -> Self {
        Self {
            allow_headers: Some(headers),
            ..self
        }
    }
    
    pub fn allow_methods(self, methods: Vec<HttpMethod>) -> Self {
        Self {
            allow_methods: Some(methods.into_iter().map(Into::into).collect()),
            ..self
        }
    }
    
    pub fn allow_origins(self, origins: Vec<String>) -> Self {
        Self {
            allow_origins: Some(origins),
            ..self
        }
    }
    
    pub fn expose_headers(self, headers: Vec<String>) -> Self {
        Self {
            expose_headers: Some(headers),
            ..self
        }
    }
    
    pub fn max_age(self, age: Duration) -> Self {
        Self {
            max_age: Some(age.as_secs()),
            ..self
        }
    }
    
    pub fn build(self) -> CorsConfiguration {
        CorsConfiguration {
            allow_credentials: self.allow_credentials,
            allow_headers: self.allow_headers,
            allow_methods: self.allow_methods,
            allow_origins: self.allow_origins,
            expose_headers: self.expose_headers,
            max_age: self.max_age,
        }
    }
}
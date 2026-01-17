use std::marker::PhantomData;
use crate::apigateway::{ApiGatewayV2Api, ApiGatewayV2ApiProperties, ApiGatewayV2ApiRef, ApiGatewayV2Integration, ApiGatewayV2IntegrationProperties, ApiGatewayV2Route, ApiGatewayV2RouteProperties, ApiGatewayV2Stage, ApiGatewayV2StageProperties, ApiGatewayV2StageRef, CorsConfiguration};
use crate::intrinsic::{get_arn, get_ref, join, AWS_ACCOUNT_PSEUDO_PARAM, AWS_PARTITION_PSEUDO_PARAM, AWS_REGION_PSEUDO_PARAM};
use crate::lambda::{FunctionRef, PermissionBuilder};
use crate::shared::HttpMethod;
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use serde_json::Value;
use std::time::Duration;
use crate::type_state;
use crate::wrappers::LambdaPermissionAction;

// TODO auth, api keys also still to do

struct RouteInfo {
    lambda_id: Id,
    path: String,
    method: Option<HttpMethod>,
    resource_id: String,
}

type_state!(
    ApiGatewayV2APIState,
    StartState,
    HttpState,
    WebsocketState,
);

/// Builder for API Gateway V2 HTTP APIs.
///
/// Creates an HTTP API with routes to Lambda functions. Automatically creates integrations and permissions for each route.
///
/// # Example
///
/// ```rust,no_run
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::apigateway::ApiGatewayV2Builder;
/// use rusty_cdk_core::shared::HttpMethod;
/// use rusty_cdk_core::lambda::{FunctionBuilder, Architecture, Runtime, Zip};
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::{memory, timeout, zip_file};
///
/// let mut stack_builder = StackBuilder::new();
///
/// let function = unimplemented!("create a function");
///
/// let (api, stage) = ApiGatewayV2Builder::new("my-api", "MyHttpApi")
///     .http()
///     .add_route_lambda("/hello", HttpMethod::Get, &function)
///     .add_route_lambda("/world", HttpMethod::Post, &function)
///     .build(&mut stack_builder);
/// ```
pub struct ApiGatewayV2Builder<T: ApiGatewayV2APIState> {
    phantom_data: PhantomData<T>,
    id: Id,
    name: Option<String>,
    protocol_type: Option<String>,
    disable_execute_api_endpoint: Option<bool>,
    disable_schema_validation: Option<bool>,
    cors_configuration: Option<CorsConfiguration>,
    route_info: Vec<RouteInfo>,
    route_selection_expression: Option<String>,
}

impl ApiGatewayV2Builder<StartState> {
    /// Creates a new API Gateway V2 HTTP API builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the API Gateway
    /// * `name` - Name of the API Gateway
    pub fn new<T: Into<String>>(id: &str, name: T) -> ApiGatewayV2Builder<StartState> {
        Self {
            phantom_data: Default::default(),
            id: Id(id.to_string()),
            name: Some(name.into()), // name is required when not OpenAPI (so currently always)
            protocol_type: None,
            disable_execute_api_endpoint: None,
            disable_schema_validation: None,
            cors_configuration: None,
            route_selection_expression: None,
            route_info: vec![],
        }
    }

    pub fn http(self) -> ApiGatewayV2Builder<HttpState> {
        ApiGatewayV2Builder {
            phantom_data: Default::default(),
            id: self.id,
            name: self.name,
            protocol_type: Some("HTTP".to_string()),
            disable_execute_api_endpoint: self.disable_execute_api_endpoint,
            cors_configuration: self.cors_configuration,
            route_info: self.route_info,
            disable_schema_validation: None,
            route_selection_expression: None,
        }
    }

    pub fn websocket<T: Into<String>>(self, route_selection_expression: T) -> ApiGatewayV2Builder<WebsocketState> {
        ApiGatewayV2Builder {
            phantom_data: Default::default(),
            id: self.id,
            name: self.name,
            protocol_type: Some("WEBSOCKET".to_string()),
            route_selection_expression: Some(route_selection_expression.into()),
            disable_execute_api_endpoint: self.disable_execute_api_endpoint,
            route_info: self.route_info,
            disable_schema_validation: self.disable_schema_validation,
            cors_configuration: None,
        }
    }
}

impl ApiGatewayV2Builder<HttpState> {
    pub fn cors_configuration(self, config: CorsConfiguration) -> Self {
        Self {
            cors_configuration: Some(config),
            ..self
        }
    }

    /// Adds a route for a specific HTTP method and path.
    ///
    /// Automatically creates the integration and Lambda permission.
    pub fn add_route_lambda<T: Into<String>>(mut self, path: T, method: HttpMethod, lambda: &FunctionRef) -> Self {
        let path = path.into();
        let path = if path.starts_with("/") { path } else { format!("/{}", path) };

        self.route_info.push(RouteInfo {
            lambda_id: lambda.get_id().clone(),
            path,
            method: Some(method),
            resource_id: lambda.get_resource_id().to_string(),
        });
        Self { ..self }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> (
        ApiGatewayV2ApiRef,
        ApiGatewayV2StageRef,
    ) {
        self.build_internal(stack_builder)
    }
}

impl ApiGatewayV2Builder<WebsocketState> {
    pub fn disable_schema_validation(self, disable: bool) -> Self {
        Self {
            disable_schema_validation: Some(disable),
            ..self
        }
    }

    /// Adds a route for a specific route key.
    ///
    /// Automatically creates the integration and Lambda permission.
    pub fn add_route_lambda<T: Into<String>>(mut self, route_key: T, lambda: &FunctionRef) -> Self {
        self.route_info.push(RouteInfo {
            lambda_id: lambda.get_id().clone(),
            path: route_key.into(),
            method: None,
            resource_id: lambda.get_resource_id().to_string(),
        });
        Self { ..self }
    }
    
    pub fn build(self, stack_builder: &mut StackBuilder) -> (
        ApiGatewayV2ApiRef,
        ApiGatewayV2StageRef,
    ) {
        self.build_internal(stack_builder)
    }
}

impl<T: ApiGatewayV2APIState> ApiGatewayV2Builder<T> {
    pub fn disable_execute_api_endpoint(self, disable_api_endpoint: bool) -> Self {
        Self {
            disable_execute_api_endpoint: Some(disable_api_endpoint),
            ..self
        }
    }

    /// Adds a default route that catches all requests not matching other routes.
    ///
    /// Automatically creates the integration and Lambda permission.
    pub fn add_default_route_lambda(mut self, lambda: &FunctionRef) -> Self {
        self.route_info.push(RouteInfo {
            lambda_id: lambda.get_id().clone(),
            path: "$default".to_string(),
            method: None,
            resource_id: lambda.get_resource_id().to_string(),
        });
        Self { ..self }
    }

    fn build_internal(
        self, stack_builder: &mut StackBuilder
    ) -> (
        ApiGatewayV2ApiRef,
        ApiGatewayV2StageRef,
    ) {
        let api_resource_id = Resource::generate_id("HttpApiGateway");
        let stage_resource_id = Resource::generate_id("HttpApiStage");
        let stage_id = Id::generate_id(&self.id, "Stage");

        let protocol_type = self.protocol_type.expect("protocol type should be present, enforced by builder");

        self
            .route_info
            .into_iter()
            .for_each(|info| {
                let route_id = Id::combine_with_resource_id(&self.id, &info.lambda_id);
                let route_permission_id = Id::generate_id(&self.id, "Permission");
                let route_integration_id = Id::generate_id(&self.id, "Integration");

                let integration_resource_id = Resource::generate_id("HttpApiIntegration");
                let route_resource_id = Resource::generate_id("HttpApiRoute");

                PermissionBuilder::new(
                    &route_permission_id,
                    LambdaPermissionAction("lambda:InvokeFunction".to_string()),
                    get_arn(&info.resource_id),
                    "apigateway.amazonaws.com".to_string(),
                )
                    .source_arn(join(
                        "",
                        vec![
                            Value::String("arn:".to_string()),
                            get_ref(AWS_PARTITION_PSEUDO_PARAM),
                            Value::String(":execute-api:".to_string()),
                            get_ref(AWS_REGION_PSEUDO_PARAM),
                            Value::String(":".to_string()),
                            get_ref(AWS_ACCOUNT_PSEUDO_PARAM),
                            Value::String(":".to_string()),
                            get_ref(&api_resource_id),
                            Value::String(format!("*/*{}", info.path)),
                        ],
                    ))
                    .build(stack_builder);

                let integration = ApiGatewayV2Integration {
                    id: route_integration_id,
                    resource_id: integration_resource_id.clone(),
                    r#type: "AWS::ApiGatewayV2::Integration".to_string(),
                    properties: ApiGatewayV2IntegrationProperties {
                        api_id: get_ref(&api_resource_id),
                        integration_type: "AWS_PROXY".to_string(),
                        payload_format_version: if &protocol_type == "HTTP" { Some("2.0".to_string()) } else { Some("1.0".to_string()) },
                        integration_uri: Some(get_arn(&info.resource_id)),
                        // TODO allow passing these
                        content_handling_strategy: None, // only for websocket
                        integration_method: None, // only for websocket - set to post for lambda integration
                        passthrough_behavior: None,
                        request_parameters: None,
                        request_templates: None,
                        response_parameters: None,
                        timeout_in_millis: None,
                    },
                };
                stack_builder.add_resource(integration);

                let route_key = if let Some(method) = info.method {
                    let method: String = method.into();
                    format!("{} {}", method, info.path)
                } else {
                    info.path
                };

                let route = ApiGatewayV2Route {
                    id: route_id,
                    resource_id: route_resource_id.clone(),
                    r#type: "AWS::ApiGatewayV2::Route".to_string(),
                    properties: ApiGatewayV2RouteProperties {
                        api_id: get_ref(&api_resource_id),
                        route_key,
                        target: Some(join(
                            "",
                            vec![Value::String("integrations/".to_string()), get_ref(&integration_resource_id)],
                        )),
                    },
                };
                stack_builder.add_resource(route);
            });

        stack_builder.add_resource(ApiGatewayV2Stage {
            id: stage_id,
            resource_id: stage_resource_id.clone(),
            r#type: "AWS::ApiGatewayV2::Stage".to_string(),
            properties: ApiGatewayV2StageProperties {
                api_id: get_ref(&api_resource_id),
                stage_name: if &protocol_type == "HTTP" { "$default".to_string() } else { "prod".to_string() }, // in the future, expose this
                auto_deploy: true,
                default_route_settings: None,
                route_settings: None,
            },
        });

        stack_builder.add_resource(ApiGatewayV2Api {
            id: self.id,
            resource_id: api_resource_id.clone(),
            r#type: "AWS::ApiGatewayV2::Api".to_string(),
            properties: ApiGatewayV2ApiProperties {
                name: self.name,
                protocol_type,
                disable_execute_api_endpoint: self.disable_execute_api_endpoint,
                disable_schema_validation: self.disable_schema_validation,
                cors_configuration: self.cors_configuration,
                route_selection_expression: self.route_selection_expression,
            },
        });

        let stage = ApiGatewayV2StageRef::new(stage_resource_id);
        let api = ApiGatewayV2ApiRef::new(api_resource_id);

        (api, stage)
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

impl Default for CorsConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
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

    #[must_use]
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

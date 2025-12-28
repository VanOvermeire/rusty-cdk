use serde_json::Value;
use crate::appsync::{AppSyncApi, AppSyncApiProperties, AppSyncApiRef, AppSyncAuthMode, AuthProvider, ChannelNamespace, ChannelNamespaceProperties, ChannelNamespaceRef, CognitoConfig, EventConfig, EventLogConfig, LambdaAuthorizerConfig, OpenIDConnectConfig};
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use crate::wrappers::{AppSyncApiName, ChannelNamespaceName};

// TODO add api key builder + DTO

pub struct AppSyncApiBuilder {
    id: Id,
    name: String,
    event_config: Option<EventConfig>,
}

impl AppSyncApiBuilder {
    pub fn new(id: &str, app_sync_api_name: AppSyncApiName) -> Self {
        Self { id: Id(id.to_string()), name: app_sync_api_name.0, event_config: None }
    }

    pub fn event_config(self, event_config: EventConfig) -> Self {
        Self {
            event_config: Some(event_config),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> AppSyncApiRef {
        let resource_id = Resource::generate_id("AppSyncApi");
        let api = AppSyncApi {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::AppSync::Api".to_string(),
            properties: AppSyncApiProperties {
                name: self.name,
                event_config: self.event_config,
            },
        };
        stack_builder.add_resource(api);

        AppSyncApiRef::new(resource_id)
    }
}

pub struct EventConfigBuilder {
    auth_providers: Vec<AuthProvider>,
    connection_auth_modes: Vec<AppSyncAuthMode>,
    default_auth_modes: Vec<AppSyncAuthMode>,
    default_subscribe_auth_modes: Vec<AppSyncAuthMode>,
    log_config: Option<EventLogConfig>,
}

impl EventConfigBuilder {
    pub fn new(auth_providers: Vec<AuthProvider>, connection_auth_modes: Vec<AuthMode>, default_auth_modes: Vec<AuthMode>, default_subscribe_auth_modes: Vec<AuthMode>) -> Self {
        Self {
            auth_providers,
            connection_auth_modes: connection_auth_modes.into_iter().map(Into::into).collect(),
            default_auth_modes: default_auth_modes.into_iter().map(Into::into).collect(),
            default_subscribe_auth_modes: default_subscribe_auth_modes.into_iter().map(Into::into).collect(),
            log_config: None,
        }
    }

    pub fn log_config(self, config: EventLogConfig) -> Self {
        Self {
            log_config: Some(config),
            ..self
        }
    }

    pub fn build(self) -> EventConfig {
        EventConfig {
            auth_providers: self.auth_providers,
            connection_auth_modes: self.connection_auth_modes,
            default_auth_modes: self.default_auth_modes,
            default_subscribe_auth_modes: self.default_subscribe_auth_modes,
            log_config: self.log_config,
        }
    }
}

pub enum AuthType {
    AmazonCognitoUserPools(CognitoConfig),
    AwsIam,
    ApiKey,
    OpenidConnect(OpenIDConnectConfig),
    AwsLambda(LambdaAuthorizerConfig),
}

#[derive(Debug, Clone)]
pub enum AuthMode {
    AmazonCognitoUserPools,
    AwsIam,
    ApiKey,
    OpenidConnect,
    AwsLambda,
}

impl From<AuthMode> for AppSyncAuthMode {
    fn from(mode: AuthMode) -> AppSyncAuthMode {
        match mode {
            AuthMode::AmazonCognitoUserPools => {
                AppSyncAuthMode {
                    auth_type: Some("AMAZON_COGNITO_USER_POOLS".to_string())
                }
            },
            AuthMode::AwsIam => {
                AppSyncAuthMode {
                    auth_type: Some("AWS_IAM".to_string())
                }
            },
            AuthMode::ApiKey => {
                AppSyncAuthMode {
                    auth_type: Some("API_KEY".to_string())
                }
            },
            AuthMode::OpenidConnect => {
                AppSyncAuthMode {
                    auth_type: Some("OPENID_CONNECT".to_string())
                }
            },
            AuthMode::AwsLambda => {
                AppSyncAuthMode {
                    auth_type: Some("AWS_LAMBDA".to_string())
                }
            },
        }
    }
}

pub struct AuthProviderBuilder {
    auth_type: String,
    cognito_config: Option<CognitoConfig>,
    lambda_auth_config: Option<LambdaAuthorizerConfig>,
    open_id_connect_config: Option<OpenIDConnectConfig>,
}

impl AuthProviderBuilder {
    pub fn new(auth_type: AuthType) -> Self {
        match auth_type {
            AuthType::AmazonCognitoUserPools(c) => {
                Self {
                    auth_type: "AMAZON_COGNITO_USER_POOLS".to_string(),
                    cognito_config: Some(c),
                    lambda_auth_config: None,
                    open_id_connect_config: None,
                }
            }
            AuthType::AwsIam => {
                Self {
                    auth_type: "AWS_IAM".to_string(),
                    cognito_config: None,
                    lambda_auth_config: None,
                    open_id_connect_config: None,
                }
            }
            AuthType::ApiKey => {
                Self {
                    auth_type: "API_KEY".to_string(),
                    cognito_config: None,
                    lambda_auth_config: None,
                    open_id_connect_config: None,
                }
            }
            AuthType::OpenidConnect(c) => {
                Self {
                    auth_type: "OPENID_CONNECT".to_string(),
                    open_id_connect_config: Some(c),
                    cognito_config: None,
                    lambda_auth_config: None,
                }
            }
            AuthType::AwsLambda(c) => {
                Self {
                    auth_type: "AWS_LAMBDA".to_string(),
                    lambda_auth_config: Some(c),
                    cognito_config: None,
                    open_id_connect_config: None,
                }
            }
        }
    }

    pub fn build(self) -> AuthProvider {
        AuthProvider {
            auth_type: self.auth_type,
            cognito_config: self.cognito_config,
            lambda_auth_config: self.lambda_auth_config,
            open_id_connect_config: self.open_id_connect_config,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppSyncApiLogLevel {
    None,
    Error,
    All,
    Info,
    Debug
}

impl From<AppSyncApiLogLevel> for String {
    fn from(api: AppSyncApiLogLevel) -> String {
        match api {
            AppSyncApiLogLevel::None => "NONE".to_string(),
            AppSyncApiLogLevel::Error => "ERROR".to_string(),
            AppSyncApiLogLevel::All => "ALL".to_string(),
            AppSyncApiLogLevel::Info => "INFO".to_string(),
            AppSyncApiLogLevel::Debug => "DEBUG".to_string(),
        }
    }
}

pub struct EventLogConfigBuilder {
    cloudwatch_logs_role_arn: String,
    log_level: String
}

impl EventLogConfigBuilder {
    pub fn new(cloudwatch_logs_role_arn: String, log_level: AppSyncApiLogLevel) -> Self {
        Self { cloudwatch_logs_role_arn, log_level: log_level.into() }
    }

    pub fn build(self) -> EventLogConfig {
        EventLogConfig {
            cloudwatch_logs_role_arn: self.cloudwatch_logs_role_arn,
            log_level: self.log_level,
        }
    }
}

pub struct ChannelNamespaceBuilder {
    id: Id,
    api_id: Value,
    name: String, // TODO should actually also be unique within the api
    publish_auth_modes: Option<Vec<AppSyncAuthMode>>,
    subscribe_auth_modes: Option<Vec<AppSyncAuthMode>>,
}

impl ChannelNamespaceBuilder {
    pub fn new(id: &str, api_id: &AppSyncApiRef, name: ChannelNamespaceName) -> Self {
        Self {
            id: Id(id.to_string()),
            api_id: api_id.get_att("ApiId"),
            name: name.0,
            publish_auth_modes: None,
            subscribe_auth_modes: None,
        }
    }
    
    pub fn publish_auth_modes(self, publish_auth_modes: Vec<AuthMode>) -> Self {
        Self {
            publish_auth_modes: Some(publish_auth_modes.into_iter().map(Into::into).collect()),
            ..self
        }
    }

    
    pub fn subscribe_auth_modes(self, subscribe_auth_modes: Vec<AuthMode>) -> Self {
        Self {
            subscribe_auth_modes: Some(subscribe_auth_modes.into_iter().map(Into::into).collect()),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> ChannelNamespaceRef {
        let resource_id = Resource::generate_id("ChannelNamespace");
        let channel = ChannelNamespace {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::AppSync::ChannelNamespace".to_string(),
            properties: ChannelNamespaceProperties {
                api_id: self.api_id,
                name: self.name,
                publish_auth_modes: self.publish_auth_modes,
                subscribe_auth_modes: self.subscribe_auth_modes,
            },
        };
        stack_builder.add_resource(channel);

        ChannelNamespaceRef::new(resource_id)
    }
}
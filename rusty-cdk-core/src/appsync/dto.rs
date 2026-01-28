use serde_json::Value;
use serde::{Deserialize, Serialize};
use crate::{dto_methods, ref_struct};
use crate::shared::Id;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum AppSyncApiType {
    #[serde(rename = "AWS::AppSync::Api")]
    AppSyncApiType
}

ref_struct!(AppSyncApiRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSyncApi {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: AppSyncApiType,
    #[serde(rename = "Properties")]
    pub(crate) properties: AppSyncApiProperties,
}
dto_methods!(AppSyncApi);

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSyncApiProperties {
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "EventConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) event_config: Option<EventConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventConfig {
    #[serde(rename = "AuthProviders")]
    pub(crate) auth_providers: Vec<AuthProvider>,
    #[serde(rename = "ConnectionAuthModes")]
    pub(crate) connection_auth_modes: Vec<AppSyncAuthMode>,
    #[serde(rename = "DefaultPublishAuthModes")]
    pub(crate) default_auth_modes: Vec<AppSyncAuthMode>,
    #[serde(rename = "DefaultSubscribeAuthModes")]
    pub(crate) default_subscribe_auth_modes: Vec<AppSyncAuthMode>,
    #[serde(rename = "LogConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) log_config: Option<EventLogConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthProvider {
    #[serde(rename = "AuthType")]
    pub(crate) auth_type: String,
    #[serde(rename = "CognitoConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) cognito_config: Option<CognitoConfig>,
    #[serde(rename = "LambdaAuthorizerConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) lambda_auth_config: Option<LambdaAuthorizerConfig>,
    #[serde(rename = "OpenIDConnectConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) open_id_connect_config: Option<OpenIDConnectConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CognitoConfig {
    #[serde(rename = "AwsRegion")]
    pub(crate) aws_region: String,
    #[serde(rename = "UserPoolId")]
    pub(crate) user_pool_id: String,
    // AppIdClientRegex: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LambdaAuthorizerConfig {
    #[serde(rename = "AuthorizerResultTtlInSeconds", skip_serializing_if = "Option::is_none")]
    pub(crate) authorizer_result_ttl_seconds: Option<u16>,
    #[serde(rename = "AuthorizerUri")]
    pub(crate) authorizer_uri: String,
    // IdentityValidationExpression: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenIDConnectConfig {
    #[serde(rename = "AuthTTL", skip_serializing_if = "Option::is_none")]
    pub(crate) auth_ttl_millis: Option<u32>,
    #[serde(rename = "ClientId", skip_serializing_if = "Option::is_none")]
    pub(crate) client_id: Option<String>,
    #[serde(rename = "IatTTL", skip_serializing_if = "Option::is_none")]
    pub(crate) iat_ttl_millis: Option<u32>,
    #[serde(rename = "Issuer")]
    pub(crate) issuer: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSyncAuthMode {
    #[serde(rename = "AuthType", skip_serializing_if = "Option::is_none")]
    pub(crate) auth_type: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogConfig {
    #[serde(rename = "CloudWatchLogsRoleArn")]
    pub(crate) cloudwatch_logs_role_arn: String,
    #[serde(rename = "LogLevel")]
    pub(crate) log_level: String
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ChannelNamespaceType {
    #[serde(rename = "AWS::AppSync::ChannelNamespace")]
    ChannelNamespaceType
}

ref_struct!(ChannelNamespaceRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelNamespace {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: ChannelNamespaceType,
    #[serde(rename = "Properties")]
    pub(crate) properties: ChannelNamespaceProperties,
}
dto_methods!(ChannelNamespace);

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelNamespaceProperties {
    #[serde(rename = "ApiId")]
    pub(crate) api_id: Value,
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "PublishAuthModes", skip_serializing_if = "Option::is_none")]
    pub(crate) publish_auth_modes: Option<Vec<AppSyncAuthMode>>,
    #[serde(rename = "SubscribeAuthModes", skip_serializing_if = "Option::is_none")]
    pub(crate) subscribe_auth_modes: Option<Vec<AppSyncAuthMode>>,
    // CodeHandlers: String
    // CodeS3Location: String
    // HandlerConfigs
}

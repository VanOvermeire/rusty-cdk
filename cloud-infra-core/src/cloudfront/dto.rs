use crate::intrinsic_functions::{get_att};
use crate::s3::dto::BucketPolicy;
use crate::shared::Id;
use serde::Serialize;
use serde_json::Value;
use crate::ref_struct;

ref_struct!(OriginAccessControlRef);

#[derive(Debug, Serialize)]
pub struct OriginAccessControl {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: OriginControlProperties,
}

impl OriginAccessControl {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_att(&self, att: &str) -> Value {
        get_att(self.get_resource_id(), att)
    }
}

#[derive(Debug, Serialize)]
pub struct OriginControlProperties {
    #[serde(rename = "OriginAccessControlConfig")]
    pub(crate) config: OriginAccessControlConfig,
}

#[derive(Debug, Serialize)]
pub struct OriginAccessControlConfig {
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "OriginAccessControlOriginType")]
    pub(crate) origin_access_control_type: String,
    #[serde(rename = "SigningBehavior")]
    pub(crate) signing_behavior: String,
    #[serde(rename = "SigningProtocol")]
    pub(crate) signing_protocol: String,
}

ref_struct!(CachePolicyRef);

#[derive(Debug, Serialize)]
pub struct CachePolicy {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: CachePolicyProperties,
}

impl CachePolicy {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_att_id(&self) -> Value {
        get_att(self.get_resource_id(), "Id")
    }
}

#[derive(Debug, Serialize)]
pub struct CachePolicyProperties {
    #[serde(rename = "CachePolicyConfig")]
    pub(crate) config: CachePolicyConfig,
}

#[derive(Debug, Serialize)]
pub struct CachePolicyConfig {
    #[serde(rename = "DefaultTTL")]
    pub(crate) default_ttl: u32,
    #[serde(rename = "MinTTL")]
    pub(crate) min_ttl: u32,
    #[serde(rename = "MaxTTL")]
    pub(crate) max_ttl: u32,
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "ParametersInCacheKeyAndForwardedToOrigin")]
    pub(crate) params_in_cache_key_and_forwarded: ParametersInCacheKeyAndForwardedToOrigin,
}

#[derive(Debug, Serialize)]
pub struct ParametersInCacheKeyAndForwardedToOrigin {
    #[serde(rename = "CookiesConfig")]
    pub(crate) cookies_config: CookiesConfig,
    #[serde(rename = "EnableAcceptEncodingBrotli", skip_serializing_if = "Option::is_none")]
    pub(crate) accept_encoding_brotli: Option<bool>,
    #[serde(rename = "EnableAcceptEncodingGzip")]
    pub(crate) accept_encoding_gzip: bool,
    #[serde(rename = "HeadersConfig")]
    pub(crate) headers_config: HeadersConfig,
    #[serde(rename = "QueryStringsConfig")]
    pub(crate) query_strings_config: QueryStringsConfig,
}

#[derive(Debug, Serialize)]
pub struct HeadersConfig {
    #[serde(rename = "HeaderBehavior")]
    pub(crate) headers_behavior: String,
    #[serde(rename = "Headers", skip_serializing_if = "Option::is_none")]
    pub(crate) headers: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct QueryStringsConfig {
    #[serde(rename = "QueryStringBehavior")]
    pub(crate) query_strings_behavior: String,
    #[serde(rename = "QueryStrings", skip_serializing_if = "Option::is_none")]
    pub(crate) query_strings: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct CookiesConfig {
    #[serde(rename = "CookieBehavior")]
    pub(crate) cookie_behavior: String,
    #[serde(rename = "Cookies", skip_serializing_if = "Option::is_none")]
    pub(crate) cookies: Option<Vec<String>>,
}

ref_struct!(DistributionRef);

#[derive(Debug, Serialize)]
pub struct Distribution {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: DistributionProperties,
}

impl Distribution {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct DistributionProperties {
    #[serde(rename = "DistributionConfig")]
    pub(crate) config: DistributionConfig,
}

#[derive(Debug, Serialize)]
pub struct DistributionConfig {
    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    pub(crate) aliases: Option<Vec<String>>, // probably can limit possible values this further
    #[serde(rename = "CacheBehaviors", skip_serializing_if = "Option::is_none")]
    pub(crate) cache_behaviors: Option<Vec<CacheBehavior>>,
    #[serde(rename = "CNAMEs", skip_serializing_if = "Option::is_none")]
    pub(crate) cnames: Option<Vec<String>>,
    #[serde(rename = "DefaultCacheBehavior")]
    pub(crate) default_cache_behavior: DefaultCacheBehavior,
    #[serde(rename = "DefaultRootObject")]
    pub(crate) default_root_object: String,
    #[serde(rename = "Enabled")]
    pub(crate) enabled: bool,
    #[serde(rename = "HttpVersion", skip_serializing_if = "Option::is_none")]
    pub(crate) http_version: Option<String>,
    #[serde(rename = "IPV6Enabled", skip_serializing_if = "Option::is_none")]
    pub(crate) ipv6_enabled: Option<bool>,
    #[serde(rename = "OriginGroups", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_groups: Option<OriginGroups>,
    #[serde(rename = "Origins", skip_serializing_if = "Option::is_none")]
    pub(crate) origins: Option<Vec<Origin>>,
    #[serde(rename = "PriceClass", skip_serializing_if = "Option::is_none")]
    pub(crate) price_class: Option<String>,
    #[serde(rename = "ViewerCertificate", skip_serializing_if = "Option::is_none")]
    pub(crate) viewer_certificate: Option<ViewerCertificate>,
    // "Restrictions" : Restrictions,
    // "Logging" : Logging,
    // "ConnectionMode" : String,
    // "ContinuousDeploymentPolicyId" : String,
    // "CustomErrorResponses" : [ CustomErrorResponse, ... ],
    // "TenantConfig" : TenantConfig,
    // "Staging" : Boolean,
    // "WebACLId" : String
}

#[derive(Debug, Serialize)]
pub struct ViewerCertificate {
    #[serde(rename = "AcmCertificateArn", skip_serializing_if = "Option::is_none")]
    pub(crate) acm_cert_arn: Option<String>,
    #[serde(rename = "CloudFrontDefaultCertificate", skip_serializing_if = "Option::is_none")]
    pub(crate) cloudfront_default_cert: Option<bool>,
    #[serde(rename = "IamCertificateId", skip_serializing_if = "Option::is_none")]
    pub(crate) iam_cert_id: Option<String>,
    #[serde(rename = "MinimumProtocolVersion", skip_serializing_if = "Option::is_none")]
    pub(crate) min_protocol_version: Option<String>,
    #[serde(rename = "SslSupportMethod", skip_serializing_if = "Option::is_none")]
    pub(crate) ssl_support_method: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OriginGroups {
    #[serde(rename = "Items")]
    pub(crate) items: Vec<OriginGroup>,
    #[serde(rename = "Quantity")]
    pub(crate) quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct OriginGroup {
    #[serde(rename = "Id")]
    pub(crate) id: String,
    #[serde(rename = "FailoverCriteria")]
    pub(crate) fail_over_criteria: FailOverCriteria,
    #[serde(rename = "Members")]
    pub(crate) members: OriginGroupMembers,
    #[serde(rename = "SelectionCriteria", skip_serializing_if = "Option::is_none")]
    pub(crate) selection_criteria: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OriginGroupMembers {
    #[serde(rename = "Items")]
    pub(crate) items: Vec<u32>, // exactly two
    #[serde(rename = "Quantity")]
    pub(crate) quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct OriginGroupMember {
    #[serde(rename = "OriginId")]
    pub(crate) origin_id: String,
}

#[derive(Debug, Serialize)]
pub struct FailOverCriteria {
    #[serde(rename = "StatusCodes")]
    pub(crate) status_codes: StatusCodes,
}

#[derive(Debug, Serialize)]
pub struct StatusCodes {
    #[serde(rename = "Items")]
    pub(crate) items: Vec<u32>, // min 1
    #[serde(rename = "Quantity")]
    pub(crate) quantity: u32,
}

// should have AN origin
#[derive(Debug, Serialize)]
pub struct Origin {
    #[serde(rename = "Id")]
    pub(crate) id: String,
    #[serde(skip)]
    pub(crate) s3_bucket_policy: Option<BucketPolicy>,
    #[serde(rename = "DomainName")]
    pub(crate) domain_name: Value,
    #[serde(rename = "ConnectionAttempts", skip_serializing_if = "Option::is_none")]
    pub(crate) connection_attempts: Option<u16>,
    #[serde(rename = "ConnectionTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) connection_timeout: Option<u16>,
    #[serde(rename = "OriginAccessControlId", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_access_control_id: Option<Value>,
    #[serde(rename = "OriginPath", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_path: Option<String>,
    #[serde(rename = "ResponseCompletionTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) response_completion_timeout: Option<u16>,
    #[serde(rename = "S3OriginConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) s3origin_config: Option<S3OriginConfig>,
    #[serde(rename = "OriginCustomHeaders", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_custom_headers: Option<Vec<OriginCustomHeader>>,
    #[serde(rename = "VpcOriginConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) vpc_origin_config: Option<VpcOriginConfig>,
    // "CustomOriginConfig"
    // "OriginShield"
}

impl Origin {
    pub fn get_origin_id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct OriginCustomHeader {
    #[serde(rename = "HeaderName")]
    pub(crate) header_name: String,
    #[serde(rename = "HeaderValue")]
    pub(crate) header_value: String,
}

#[derive(Debug, Serialize)]
pub struct VpcOriginConfig {
    #[serde(rename = "VpcOriginId")]
    pub(crate) vpc_origin_id: String,
    #[serde(rename = "OriginKeepaliveTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_keep_alive_timeout: Option<u32>, // 1-5
    #[serde(rename = "OriginReadTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_read_timeout: Option<u32>, // 1-120
    #[serde(rename = "OwnerAccountId", skip_serializing_if = "Option::is_none")]
    pub(crate) owner_account_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct S3OriginConfig {
    #[serde(rename = "OriginReadTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_read_timeout: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct DefaultCacheBehavior {
    // did not add deprecated fields like MaxTTL //
    #[serde(rename = "TargetOriginId")]
    pub(crate) target_origin_id: String,
    #[serde(rename = "CachePolicyId")]
    pub(crate) cache_policy_id: Value,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub(crate) viewer_protocol_policy: String,
    #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_methods: Option<Vec<String>>,
    #[serde(rename = "CachedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) cached_methods: Option<Vec<String>>,
    #[serde(rename = "Compress", skip_serializing_if = "Option::is_none")]
    pub(crate) compress: Option<bool>,
    // #[serde(rename = "TrustedKeyGroups", skip_serializing_if = "Option::is_none")]
    // pub(crate) trusted_key_groups: Option<Vec<String>>,
    // "RealtimeLogConfigArn" : String,
    // "GrpcConfig" : GrpcConfig, => Update your distribution's cache behavior to allow HTTP methods, including the POST method; Specify HTTP/2 as one of the supported HTTP versions.
    // "OriginRequestPolicyId" : String,
    // "LambdaFunctionAssociations" : [ LambdaFunctionAssociation, ... ],
    // "FunctionAssociations" : [ FunctionAssociation, ... ],
    // "FieldLevelEncryptionId" : String,
    // "ResponseHeadersPolicyId" : String,
    // "SmoothStreaming" : Boolean,
}

#[derive(Debug, Serialize)]
pub struct CacheBehavior {
    #[serde(rename = "PathPattern")]
    pub(crate) path_pattern: String,
    #[serde(rename = "TargetOriginId")]
    pub(crate) target_origin_id: String,
    #[serde(rename = "CachePolicyId")]
    pub(crate) cache_policy_id: String,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub(crate) viewer_protocol_policy: String,
    #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_methods: Option<Vec<String>>,
    #[serde(rename = "CachedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) cached_methods: Option<Vec<String>>,
    #[serde(rename = "Compress", skip_serializing_if = "Option::is_none")]
    pub(crate) compress: Option<bool>,
    #[serde(rename = "TrustedKeyGroups", skip_serializing_if = "Option::is_none")]
    pub(crate) trusted_key_groups: Option<Vec<String>>,
    // "RealtimeLogConfigArn" : String,
    // "GrpcConfig" : GrpcConfig,
    // "OriginRequestPolicyId" : String,
    // "LambdaFunctionAssociations" : [ LambdaFunctionAssociation, ... ],
    // "FunctionAssociations" : [ FunctionAssociation, ... ],
    // "FieldLevelEncryptionId" : String,
    // "ResponseHeadersPolicyId" : String,
    // "SmoothStreaming" : Boolean,
}

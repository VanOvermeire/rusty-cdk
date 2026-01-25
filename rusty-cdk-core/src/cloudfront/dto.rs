use crate::s3::BucketPolicy;
use crate::shared::Id;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{dto_methods, ref_struct};

ref_struct!(OriginAccessControlRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginAccessControl {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: OriginControlProperties,
}
dto_methods!(OriginAccessControl);

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginControlProperties {
    #[serde(rename = "OriginAccessControlConfig")]
    pub(super) config: OriginAccessControlConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginAccessControlConfig {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "OriginAccessControlOriginType")]
    pub(super) origin_access_control_type: String,
    #[serde(rename = "SigningBehavior")]
    pub(super) signing_behavior: String,
    #[serde(rename = "SigningProtocol")]
    pub(super) signing_protocol: String,
}

ref_struct!(CachePolicyRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct CachePolicy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: CachePolicyProperties,
}
dto_methods!(CachePolicy);

#[derive(Debug, Serialize, Deserialize)]
pub struct CachePolicyProperties {
    #[serde(rename = "CachePolicyConfig")]
    pub(super) config: CachePolicyConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachePolicyConfig {
    #[serde(rename = "DefaultTTL")]
    pub(super) default_ttl: u32,
    #[serde(rename = "MinTTL")]
    pub(super) min_ttl: u32,
    #[serde(rename = "MaxTTL")]
    pub(super) max_ttl: u32,
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "ParametersInCacheKeyAndForwardedToOrigin")]
    pub(super) params_in_cache_key_and_forwarded: ParametersInCacheKeyAndForwardedToOrigin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParametersInCacheKeyAndForwardedToOrigin {
    #[serde(rename = "CookiesConfig")]
    pub(super) cookies_config: CookiesConfig,
    #[serde(rename = "EnableAcceptEncodingBrotli", skip_serializing_if = "Option::is_none")]
    pub(super) accept_encoding_brotli: Option<bool>,
    #[serde(rename = "EnableAcceptEncodingGzip")]
    pub(super) accept_encoding_gzip: bool,
    #[serde(rename = "HeadersConfig")]
    pub(super) headers_config: HeadersConfig,
    #[serde(rename = "QueryStringsConfig")]
    pub(super) query_strings_config: QueryStringsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadersConfig {
    #[serde(rename = "HeaderBehavior")]
    pub(super) headers_behavior: String,
    #[serde(rename = "Headers", skip_serializing_if = "Option::is_none")]
    pub(super) headers: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryStringsConfig {
    #[serde(rename = "QueryStringBehavior")]
    pub(super) query_strings_behavior: String,
    #[serde(rename = "QueryStrings", skip_serializing_if = "Option::is_none")]
    pub(super) query_strings: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookiesConfig {
    #[serde(rename = "CookieBehavior")]
    pub(super) cookie_behavior: String,
    #[serde(rename = "Cookies", skip_serializing_if = "Option::is_none")]
    pub(super) cookies: Option<Vec<String>>,
}

ref_struct!(DistributionRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: DistributionProperties,
}
dto_methods!(Distribution);

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionProperties {
    #[serde(rename = "DistributionConfig")]
    pub(super) config: DistributionConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionConfig {
    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    pub(super) aliases: Option<Vec<String>>, // probably can limit possible values this further
    #[serde(rename = "CacheBehaviors", skip_serializing_if = "Option::is_none")]
    pub(super) cache_behaviors: Option<Vec<CacheBehavior>>,
    #[serde(rename = "CNAMEs", skip_serializing_if = "Option::is_none")]
    pub(super) cnames: Option<Vec<String>>,
    #[serde(rename = "DefaultCacheBehavior")]
    pub(super) default_cache_behavior: DefaultCacheBehavior,
    #[serde(rename = "DefaultRootObject")]
    pub(super) default_root_object: String,
    #[serde(rename = "Enabled")]
    pub(super) enabled: bool,
    #[serde(rename = "HttpVersion", skip_serializing_if = "Option::is_none")]
    pub(super) http_version: Option<String>,
    #[serde(rename = "IPV6Enabled", skip_serializing_if = "Option::is_none")]
    pub(super) ipv6_enabled: Option<bool>,
    #[serde(rename = "OriginGroups", skip_serializing_if = "Option::is_none")]
    pub(super) origin_groups: Option<OriginGroups>,
    #[serde(rename = "Origins", skip_serializing_if = "Option::is_none")]
    pub(super) origins: Option<Vec<Origin>>,
    #[serde(rename = "PriceClass", skip_serializing_if = "Option::is_none")]
    pub(super) price_class: Option<String>,
    #[serde(rename = "ViewerCertificate", skip_serializing_if = "Option::is_none")]
    pub(super) viewer_certificate: Option<ViewerCertificate>,
    // "Restrictions" : Restrictions,
    // "Logging" : Logging,
    // "ConnectionMode" : String,
    // "ContinuousDeploymentPolicyId" : String,
    // "CustomErrorResponses" : [ CustomErrorResponse, ... ],
    // "TenantConfig" : TenantConfig,
    // "Staging" : Boolean,
    // "WebACLId" : String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewerCertificate {
    #[serde(rename = "AcmCertificateArn", skip_serializing_if = "Option::is_none")]
    pub(super) acm_cert_arn: Option<String>,
    #[serde(rename = "CloudFrontDefaultCertificate", skip_serializing_if = "Option::is_none")]
    pub(super) cloudfront_default_cert: Option<bool>,
    #[serde(rename = "IamCertificateId", skip_serializing_if = "Option::is_none")]
    pub(super) iam_cert_id: Option<String>,
    #[serde(rename = "MinimumProtocolVersion", skip_serializing_if = "Option::is_none")]
    pub(super) min_protocol_version: Option<String>,
    #[serde(rename = "SslSupportMethod", skip_serializing_if = "Option::is_none")]
    pub(super) ssl_support_method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginGroups {
    #[serde(rename = "Items")]
    pub(super) items: Vec<OriginGroup>,
    #[serde(rename = "Quantity")]
    pub(super) quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginGroup {
    #[serde(rename = "Id")]
    pub(super) id: String,
    #[serde(rename = "FailoverCriteria")]
    pub(super) fail_over_criteria: FailOverCriteria,
    #[serde(rename = "Members")]
    pub(super) members: OriginGroupMembers,
    #[serde(rename = "SelectionCriteria", skip_serializing_if = "Option::is_none")]
    pub(super) selection_criteria: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginGroupMembers {
    #[serde(rename = "Items")]
    pub(super) items: Vec<u32>, // exactly two
    #[serde(rename = "Quantity")]
    pub(super) quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginGroupMember {
    #[serde(rename = "OriginId")]
    pub(super) origin_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailOverCriteria {
    #[serde(rename = "StatusCodes")]
    pub(super) status_codes: StatusCodes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCodes {
    #[serde(rename = "Items")]
    pub(super) items: Vec<u32>, // min 1
    #[serde(rename = "Quantity")]
    pub(super) quantity: u32,
}

// should have AN origin
#[derive(Debug, Serialize, Deserialize)]
pub struct Origin {
    #[serde(rename = "Id")]
    pub(super) id: String,
    #[serde(skip)]
    pub(super) s3_bucket_policy: Option<BucketPolicy>,
    #[serde(rename = "DomainName")]
    pub(super) domain_name: Value,
    #[serde(rename = "ConnectionAttempts", skip_serializing_if = "Option::is_none")]
    pub(super) connection_attempts: Option<u8>,
    #[serde(rename = "ConnectionTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) connection_timeout: Option<u16>,
    #[serde(rename = "OriginAccessControlId", skip_serializing_if = "Option::is_none")]
    pub(super) origin_access_control_id: Option<Value>,
    #[serde(rename = "OriginPath", skip_serializing_if = "Option::is_none")]
    pub(super) origin_path: Option<String>,
    #[serde(rename = "ResponseCompletionTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) response_completion_timeout: Option<u16>,
    #[serde(rename = "S3OriginConfig", skip_serializing_if = "Option::is_none")]
    pub(super) s3origin_config: Option<S3OriginConfig>,
    #[serde(rename = "OriginCustomHeaders", skip_serializing_if = "Option::is_none")]
    pub(super) origin_custom_headers: Option<Vec<OriginCustomHeader>>,
    #[serde(rename = "VpcOriginConfig", skip_serializing_if = "Option::is_none")]
    pub(super) vpc_origin_config: Option<VpcOriginConfig>,
    #[serde(rename = "CustomOriginConfig", skip_serializing_if = "Option::is_none")]
    pub(super) custom_origin_config: Option<CustomOriginConfig>,
    // "OriginShield"
}

impl Origin {
    pub fn get_origin_id(&self) -> &str {
        self.id.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginCustomHeader {
    #[serde(rename = "HeaderName")]
    pub(super) header_name: String,
    #[serde(rename = "HeaderValue")]
    pub(super) header_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpcOriginConfig {
    #[serde(rename = "VpcOriginId")]
    pub(super) vpc_origin_id: String,
    #[serde(rename = "OriginKeepaliveTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) origin_keep_alive_timeout: Option<u32>, // 1-5
    #[serde(rename = "OriginReadTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) origin_read_timeout: Option<u32>, // 1-120
    #[serde(rename = "OwnerAccountId", skip_serializing_if = "Option::is_none")]
    pub(super) owner_account_id: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomOriginConfig {
    #[serde(rename = "OriginProtocolPolicy")]
    pub(super) origin_protocol_policy: String, // http-only | match-viewer | https-only
    #[serde(rename = "HTTPPort", skip_serializing_if = "Option::is_none")]
    pub(super) http_port: Option<u16>,
    #[serde(rename = "HTTPSPort", skip_serializing_if = "Option::is_none")]
    pub(super) https_port: Option<u16>,
    #[serde(rename = "IpAddressType", skip_serializing_if = "Option::is_none")]
    pub(super) ip_address_type: Option<String>, // ipv4 | ipv6 | dualstack
    #[serde(rename = "OriginKeepaliveTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) origin_keep_alive_timeout: Option<u8>, // 1 - 120
    #[serde(rename = "OriginReadTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) origin_read_timeout: Option<u8>, // 1 - 120
    #[serde(rename = "OriginSSLProtocols", skip_serializing_if = "Option::is_none")]
    pub(super) origin_ssl_protocols: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3OriginConfig {
    #[serde(rename = "OriginReadTimeout", skip_serializing_if = "Option::is_none")]
    pub(super) origin_read_timeout: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultCacheBehavior {
    // did not add deprecated fields like MaxTTL //
    #[serde(rename = "TargetOriginId")]
    pub(super) target_origin_id: String,
    #[serde(rename = "CachePolicyId")]
    pub(super) cache_policy_id: Value,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub(super) viewer_protocol_policy: String,
    #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
    pub(super) allowed_methods: Option<Vec<String>>,
    #[serde(rename = "CachedMethods", skip_serializing_if = "Option::is_none")]
    pub(super) cached_methods: Option<Vec<String>>,
    #[serde(rename = "Compress", skip_serializing_if = "Option::is_none")]
    pub(super) compress: Option<bool>,
    // #[serde(rename = "TrustedKeyGroups", skip_serializing_if = "Option::is_none")]
    // pub(super) trusted_key_groups: Option<Vec<String>>,
    // "RealtimeLogConfigArn" : String,
    // "GrpcConfig" : GrpcConfig, => Update your distribution's cache behavior to allow HTTP methods, including the POST method; Specify HTTP/2 as one of the supported HTTP versions.
    // "OriginRequestPolicyId" : String,
    // "LambdaFunctionAssociations" : [ LambdaFunctionAssociation, ... ],
    // "FunctionAssociations" : [ FunctionAssociation, ... ],
    // "FieldLevelEncryptionId" : String,
    // "ResponseHeadersPolicyId" : String,
    // "SmoothStreaming" : Boolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheBehavior {
    #[serde(rename = "PathPattern")]
    pub(super) path_pattern: String,
    #[serde(rename = "TargetOriginId")]
    pub(super) target_origin_id: String,
    #[serde(rename = "CachePolicyId")]
    pub(super) cache_policy_id: String,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub(super) viewer_protocol_policy: String,
    #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
    pub(super) allowed_methods: Option<Vec<String>>,
    #[serde(rename = "CachedMethods", skip_serializing_if = "Option::is_none")]
    pub(super) cached_methods: Option<Vec<String>>,
    #[serde(rename = "Compress", skip_serializing_if = "Option::is_none")]
    pub(super) compress: Option<bool>,
    #[serde(rename = "TrustedKeyGroups", skip_serializing_if = "Option::is_none")]
    pub(super) trusted_key_groups: Option<Vec<String>>,
    // "RealtimeLogConfigArn" : String,
    // "GrpcConfig" : GrpcConfig,
    // "OriginRequestPolicyId" : String,
    // "LambdaFunctionAssociations" : [ LambdaFunctionAssociation, ... ],
    // "FunctionAssociations" : [ FunctionAssociation, ... ],
    // "FieldLevelEncryptionId" : String,
    // "ResponseHeadersPolicyId" : String,
    // "SmoothStreaming" : Boolean,
}

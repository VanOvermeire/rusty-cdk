use crate::shared::Id;
use serde::Serialize;

// TODO AWS::CloudFront::CloudFrontOriginAccessIdentity

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
}

#[derive(Debug, Serialize)]
pub struct CachePolicyProperties {
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
    #[serde(rename = "EnableAcceptEncodingBrotli")]
    pub(crate) accept_encoding_brotli: bool,
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
    pub(crate) headers_behavior: String, // none, whitelist => whitelist requires headers
    #[serde(rename = "Headers", skip_serializing_if = "Option::is_none")]
    pub(crate) headers: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct QueryStringsConfig {
    #[serde(rename = "QueryStringBehavior")]
    pub(crate) query_strings_behavior: String, // none, whitelist, allExcept, all => allExcept and whitelist require query strings vec
    #[serde(rename = "QueryStrings", skip_serializing_if = "Option::is_none")]
    pub(crate) query_strings: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct CookiesConfig {
    #[serde(rename = "CookieBehavior")]
    pub(crate) cookie_behavior: String, // none, whitelist, allExcept, all => allExcept and whitelist require cookies vec
    #[serde(rename = "Cookies", skip_serializing_if = "Option::is_none")]
    pub(crate) cookies: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct CloudFrontDistribution {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: CloudFrontDistributionProperties,
}

impl CloudFrontDistribution {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct CloudFrontDistributionProperties {
    #[serde(rename = "DistributionConfig")]
    pub(crate) config: String,
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
    #[serde(rename = "DefaultRootObject", skip_serializing_if = "Option::is_none")]
    pub(crate) default_root_object: Option<String>, //  => requires some special work if empty?
    #[serde(rename = "Enabled")]
    pub(crate) enabled: bool, // TODO set to true by default?
    #[serde(rename = "HttpVersion", skip_serializing_if = "Option::is_none")]
    pub(crate) http_version: Option<String>, // http1.1 | http2 | http3 | http2and3
    #[serde(rename = "IPV6Enabled", skip_serializing_if = "Option::is_none")]
    pub(crate) ipv6_enabled: Option<bool>,
    #[serde(rename = "OriginGroups", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_groups: Option<OriginGroups>, // TODO either this or the next is required!
    #[serde(rename = "Origins", skip_serializing_if = "Option::is_none")]
    pub(crate) origins: Option<Vec<Origin>>,
    #[serde(rename = "PriceClass", skip_serializing_if = "Option::is_none")]
    pub(crate) price_class: Option<String>, // PriceClass_100 | PriceClass_200 | PriceClass_All | None
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
    pub(crate) acm_cert_arn: Option<String>, // when your cert is in ACM
    #[serde(rename = "CloudFrontDefaultCertificate", skip_serializing_if = "Option::is_none")]
    pub(crate) cloudfront_default_cert: Option<bool>, // when you use the cf domain name
    #[serde(rename = "IamCertificateId", skip_serializing_if = "Option::is_none")]
    pub(crate) iam_cert_id: Option<String>, // when your cert is in IAM
    #[serde(rename = "MinimumProtocolVersion", skip_serializing_if = "Option::is_none")]
    pub(crate) min_protocol_version: Option<String>, // SSLv3 | TLSv1 | TLSv1_2016 | TLSv1.1_2016 | TLSv1.2_2018 | TLSv1.2_2019 | TLSv1.2_2021 | TLSv1.3_2025 | TLSv1.2_2025 // not needed when its cloudfront default
    #[serde(rename = "SslSupportMethod", skip_serializing_if = "Option::is_none")]
    pub(crate) ssl_support_method: Option<String>, // sni-only | vip | static-ip // not needed when its cloudfront default
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
    #[serde(rename = "DomainName")]
    pub(crate) domain_name: String,
    #[serde(rename = "ConnectionAttempts", skip_serializing_if = "Option::is_none")]
    pub(crate) connection_attempts: Option<u32>, // 1-3
    #[serde(rename = "ConnectionTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) connection_timeout: Option<u32>, // 1-10
    #[serde(rename = "OriginAccessControlId", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_access_control_id: Option<String>,
    #[serde(rename = "OriginPath", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_path: Option<String>,
    #[serde(rename = "ResponseCompletionTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) response_completion_timeout: Option<u32>, // >= OriginReadTimeout
    #[serde(rename = "S3OriginConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) s3origin_config: Option<S3OriginConfig>,
    #[serde(rename = "OriginCustomHeaders", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_custom_headers: Option<Vec<OriginCustomHeader>>,
    #[serde(rename = "VpcOriginConfig", skip_serializing_if = "Option::is_none")]
    pub(crate) vpc_origin_config: Option<VpcOriginConfig>,

    // "CustomOriginConfig" : CustomOriginConfig,
    // "OriginShield" : OriginShield,
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
    #[serde(rename = "OriginAccessIdentity", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_access_identity: Option<String>, // origin-access-identity/cloudfront/ID-of-origin-access-identity
    #[serde(rename = "OriginReadTimeout", skip_serializing_if = "Option::is_none")]
    pub(crate) origin_read_timeout: Option<u32>, // 1-120 seconds
}

#[derive(Debug, Serialize)]
pub struct DefaultCacheBehavior {
    #[serde(rename = "TargetOriginId")]
    pub(crate) target_origin_id: String,
    #[serde(rename = "CachePolicyId")]
    pub(crate) cache_policy_id: String,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub(crate) viewer_protocol_policy: String, // allow-all: Viewers can use HTTP or HTTPS. OR redirect-to-https OR https-only
    #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_methods: Option<Vec<String>>, // GET and HEAD requests OR only GET, HEAD, and OPTIONS requests OR GET, HEAD, OPTIONS, PUT, PATCH, POST, and DELETE requests.
    #[serde(rename = "CachedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) cached_methods: Option<Vec<String>>, // CloudFront caches responses to GET and HEAD requests OR CloudFront caches responses to GET, HEAD, and OPTIONS requests
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

#[derive(Debug, Serialize)]
pub struct CacheBehavior {
    #[serde(rename = "PathPattern")]
    pub(crate) path_pattern: String,
    #[serde(rename = "TargetOriginId")]
    pub(crate) target_origin_id: String,
    #[serde(rename = "CachePolicyId")]
    pub(crate) cache_policy_id: String,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub(crate) viewer_protocol_policy: String, // allow-all: Viewers can use HTTP or HTTPS. OR redirect-to-https OR https-only
    #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_methods: Option<Vec<String>>, // GET and HEAD requests OR only GET, HEAD, and OPTIONS requests OR GET, HEAD, OPTIONS, PUT, PATCH, POST, and DELETE requests.
    #[serde(rename = "CachedMethods", skip_serializing_if = "Option::is_none")]
    pub(crate) cached_methods: Option<Vec<String>>, // CloudFront caches responses to GET and HEAD requests OR CloudFront caches responses to GET, HEAD, and OPTIONS requests
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

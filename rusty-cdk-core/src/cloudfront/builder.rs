use crate::cloudfront::{CacheBehavior, CachePolicy, CachePolicyConfig, CachePolicyProperties, CachePolicyRef, CookiesConfig, CustomOriginConfig, DefaultCacheBehavior, Distribution, DistributionConfig, DistributionProperties, DistributionRef, HeadersConfig, Origin, OriginAccessControl, OriginAccessControlConfig, OriginAccessControlRef, OriginControlProperties, OriginCustomHeader, ParametersInCacheKeyAndForwardedToOrigin, QueryStringsConfig, S3OriginConfig, ViewerCertificate, VpcOriginConfig};
use crate::iam::Principal::Service;
use crate::iam::{Effect, PolicyDocumentBuilder, ServicePrincipal, StatementBuilder};
use crate::intrinsic_functions::{get_att, get_ref, join};
use crate::s3::BucketPolicyBuilder;
use crate::s3::BucketRef;
use crate::shared::http::HttpMethod::{Delete, Get, Head, Options, Patch, Post, Put};
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use crate::wrappers::{CfConnectionTimeout, ConnectionAttempts, DefaultRootObject, IamAction, OriginPath, S3OriginReadTimeout};
use serde_json::{json, Value};
use std::marker::PhantomData;
use crate::type_state;

pub enum SslSupportedMethod {
    SniOnly,
    Vip,
    StaticIp,
}

impl From<SslSupportedMethod> for String {
    fn from(value: SslSupportedMethod) -> Self {
        match value {
            SslSupportedMethod::SniOnly => "sni-only".to_string(),
            SslSupportedMethod::Vip => "vip".to_string(),
            SslSupportedMethod::StaticIp => "static-ip".to_string(),
        }
    }
}

pub enum MinProtocolVersion {
    SSLV3,
    TLSv1,
    TLSv1_1,
    TLSv1_2_2018,
    TLSv1_2_2019,
    TLSv1_2_2021,
    TLSv1_2_2025,
    TLSv1_3,
}

impl From<MinProtocolVersion> for String {
    fn from(value: MinProtocolVersion) -> Self {
        match value {
            MinProtocolVersion::SSLV3 => "SSLv3".to_string(),
            MinProtocolVersion::TLSv1 => "TLSv1".to_string(),
            MinProtocolVersion::TLSv1_1 => "TLSv1.1_2016".to_string(),
            MinProtocolVersion::TLSv1_2_2018 => "TLSv1.2_2018".to_string(),
            MinProtocolVersion::TLSv1_2_2019 => "TLSv1.2_2019".to_string(),
            MinProtocolVersion::TLSv1_2_2021 => "TLSv1.2_2021".to_string(),
            MinProtocolVersion::TLSv1_2_2025 => "TLSv1.2_2025".to_string(),
            MinProtocolVersion::TLSv1_3 => "TLSv1.3_2025".to_string(),
        }
    }
}

type_state!(
    ViewerCertificateState,
    ViewerCertificateStateStartState,
    ViewerCertificateStateAcmOrIamState,
    ViewerCertificateStateEndState,
);

/// Builder for CloudFront viewer certificates.
pub struct ViewerCertificateBuilder<T: ViewerCertificateState> {
    phantom_data: PhantomData<T>,
    cloudfront_default_cert: Option<bool>,
    acm_cert_arn: Option<String>,
    iam_cert_id: Option<String>,
    min_protocol_version: Option<String>,
    ssl_support_method: Option<String>,
}

impl ViewerCertificateBuilder<ViewerCertificateStateStartState> {
    pub fn new() -> ViewerCertificateBuilder<ViewerCertificateStateStartState> {
        ViewerCertificateBuilder {
            phantom_data: Default::default(),
            acm_cert_arn: None,
            cloudfront_default_cert: None,
            iam_cert_id: None,
            min_protocol_version: None,
            ssl_support_method: None,
        }
    }

    pub fn cloudfront_default_cert(self) -> ViewerCertificateBuilder<ViewerCertificateStateEndState> {
        ViewerCertificateBuilder {
            phantom_data: Default::default(),
            cloudfront_default_cert: Some(true),
            acm_cert_arn: self.acm_cert_arn,
            iam_cert_id: self.iam_cert_id,
            min_protocol_version: self.min_protocol_version,
            ssl_support_method: self.ssl_support_method,
        }
    }

    pub fn iam_cert_id(self, id: String) -> ViewerCertificateBuilder<ViewerCertificateStateAcmOrIamState> {
        ViewerCertificateBuilder {
            phantom_data: Default::default(),
            cloudfront_default_cert: Some(true),
            acm_cert_arn: self.acm_cert_arn,
            iam_cert_id: Some(id),
            min_protocol_version: self.min_protocol_version,
            ssl_support_method: self.ssl_support_method,
        }
    }

    pub fn acm_cert_arn(self, id: String) -> ViewerCertificateBuilder<ViewerCertificateStateAcmOrIamState> {
        ViewerCertificateBuilder {
            phantom_data: Default::default(),
            cloudfront_default_cert: Some(true),
            acm_cert_arn: Some(id),
            iam_cert_id: self.iam_cert_id,
            min_protocol_version: self.min_protocol_version,
            ssl_support_method: self.ssl_support_method,
        }
    }
}

impl<T: ViewerCertificateState> ViewerCertificateBuilder<T> {
    fn build_internal(self) -> ViewerCertificate {
        ViewerCertificate {
            cloudfront_default_cert: self.cloudfront_default_cert,
            acm_cert_arn: self.acm_cert_arn,
            iam_cert_id: self.iam_cert_id,
            min_protocol_version: self.min_protocol_version,
            ssl_support_method: self.ssl_support_method,
        }
    }
}

impl ViewerCertificateBuilder<ViewerCertificateStateAcmOrIamState> {
    pub fn min_protocol_version(self, protocol_version: MinProtocolVersion) -> Self {
        Self {
            phantom_data: Default::default(),
            min_protocol_version: Some(protocol_version.into()),
            cloudfront_default_cert: self.cloudfront_default_cert,
            acm_cert_arn: self.acm_cert_arn,
            iam_cert_id: self.iam_cert_id,
            ssl_support_method: self.ssl_support_method,
        }
    }

    pub fn ssl_support_method(self, ssl_support: SslSupportedMethod) -> Self {
        Self {
            phantom_data: Default::default(),
            ssl_support_method: Some(ssl_support.into()),
            min_protocol_version: self.min_protocol_version,
            cloudfront_default_cert: self.cloudfront_default_cert,
            acm_cert_arn: self.acm_cert_arn,
            iam_cert_id: self.iam_cert_id,
        }
    }

    #[must_use]
    pub fn build(self) -> ViewerCertificate {
        self.build_internal()
    }
}

impl ViewerCertificateBuilder<ViewerCertificateStateEndState> {
    #[must_use]
    pub fn build(self) -> ViewerCertificate {
        self.build_internal()
    }
}

pub enum Cookies {
    None,
    Whitelist(Vec<String>),
    AllExcept(Vec<String>),
    All,
}
pub enum QueryString {
    None,
    Whitelist(Vec<String>),
    AllExcept(Vec<String>),
    All,
}
pub enum Headers {
    None,
    Whitelist(Vec<String>),
}

/// Builder for cache key and forwarding parameters.
///
/// Configures which request parameters (cookies, headers, query strings) are included in the cache key and forwarded to the origin.
pub struct ParametersInCacheKeyAndForwardedToOriginBuilder {
    cookies_config: CookiesConfig,
    headers_config: HeadersConfig,
    query_strings_config: QueryStringsConfig,
    accept_encoding_gzip: bool,
    accept_encoding_brotli: Option<bool>,
}

impl ParametersInCacheKeyAndForwardedToOriginBuilder {
    pub fn new(accept_encoding_gzip: bool, cookies: Cookies, query_string: QueryString, headers: Headers) -> Self {
        let cookies_config = match cookies {
            Cookies::None => CookiesConfig {
                cookie_behavior: "none".to_string(),
                cookies: None,
            },
            Cookies::Whitelist(list) => CookiesConfig {
                cookie_behavior: "whitelist".to_string(),
                cookies: Some(list),
            },
            Cookies::AllExcept(list) => CookiesConfig {
                cookie_behavior: "allExcept".to_string(),
                cookies: Some(list),
            },
            Cookies::All => CookiesConfig {
                cookie_behavior: "all".to_string(),
                cookies: None,
            },
        };
        let query_strings_config = match query_string {
            QueryString::None => QueryStringsConfig {
                query_strings_behavior: "none".to_string(),
                query_strings: None,
            },
            QueryString::Whitelist(list) => QueryStringsConfig {
                query_strings_behavior: "whitelist".to_string(),
                query_strings: Some(list),
            },
            QueryString::AllExcept(list) => QueryStringsConfig {
                query_strings_behavior: "allExcept".to_string(),
                query_strings: Some(list),
            },
            QueryString::All => QueryStringsConfig {
                query_strings_behavior: "all".to_string(),
                query_strings: None,
            },
        };
        let headers_config = match headers {
            Headers::None => HeadersConfig {
                headers_behavior: "none".to_string(),
                headers: None,
            },
            Headers::Whitelist(list) => HeadersConfig {
                headers_behavior: "whitelist".to_string(),
                headers: Some(list),
            },
        };

        Self {
            cookies_config,
            headers_config,
            query_strings_config,
            accept_encoding_gzip,
            accept_encoding_brotli: None,
        }
    }

    pub fn accept_encoding_brotli(self, accept: bool) -> Self {
        Self {
            accept_encoding_brotli: Some(accept),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> ParametersInCacheKeyAndForwardedToOrigin {
        ParametersInCacheKeyAndForwardedToOrigin {
            cookies_config: self.cookies_config,
            accept_encoding_brotli: self.accept_encoding_brotli,
            accept_encoding_gzip: self.accept_encoding_gzip,
            headers_config: self.headers_config,
            query_strings_config: self.query_strings_config,
        }
    }
}

/// Builder for CloudFront cache policies.
pub struct CachePolicyBuilder {
    id: Id,
    name: String,
    default_ttl: u32,
    min_ttl: u32,
    max_ttl: u32,
    cache_params: ParametersInCacheKeyAndForwardedToOrigin,
}

impl CachePolicyBuilder {
    /// Creates a new CloudFront cache policy builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the cache policy
    /// * `unique_name` - Name for the cache policy (must be unique)
    /// * `default_ttl` - Default time to live in seconds
    /// * `min_ttl` - Minimum time to live in seconds
    /// * `max_ttl` - Maximum time to live in seconds
    /// * `cache_params` - Parameters for cache key and origin forwarding
    pub fn new(
        id: &str,
        unique_name: &str,
        default_ttl: u32,
        min_ttl: u32,
        max_ttl: u32,
        cache_params: ParametersInCacheKeyAndForwardedToOrigin,
    ) -> Self {
        Self {
            id: Id(id.to_string()),
            name: unique_name.to_string(),
            default_ttl,
            min_ttl,
            max_ttl,
            cache_params,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> CachePolicyRef {
        let resource_id = Resource::generate_id("CachePolicy");
        stack_builder.add_resource(CachePolicy {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::CloudFront::CachePolicy".to_string(),
            properties: CachePolicyProperties {
                config: CachePolicyConfig {
                    default_ttl: self.default_ttl,
                    min_ttl: self.min_ttl,
                    max_ttl: self.max_ttl,
                    name: self.name,
                    params_in_cache_key_and_forwarded: self.cache_params,
                },
            },
        });
        CachePolicyRef::new(resource_id)
    }
}

pub enum HttpVersion {
    Http1,
    Http2,
    Http3,
    Http2And3,
}

impl From<HttpVersion> for String {
    fn from(value: HttpVersion) -> Self {
        match value {
            HttpVersion::Http1 => "http1.1".to_string(),
            HttpVersion::Http2 => "http2".to_string(),
            HttpVersion::Http3 => "http3".to_string(),
            HttpVersion::Http2And3 => "http2and3".to_string(),
        }
    }
}

pub enum PriceClass {
    PriceClass100,
    PriceClass200,
    PriceClassAll,
    None,
}

impl From<PriceClass> for String {
    fn from(value: PriceClass) -> Self {
        match value {
            PriceClass::PriceClass100 => "PriceClass_100".to_string(),
            PriceClass::PriceClass200 => "PriceClass_200".to_string(),
            PriceClass::PriceClassAll => "PriceClass_All".to_string(),
            PriceClass::None => "None".to_string(),
        }
    }
}

pub enum OriginProtocolPolicy {
    HttpOnly,
    MatchViewer,
    HttpsOnly,
}

impl From<OriginProtocolPolicy> for String {
    fn from(value: OriginProtocolPolicy) -> Self {
        match value {
            OriginProtocolPolicy::HttpOnly => "http-only".to_string(),
            OriginProtocolPolicy::MatchViewer => "match-viewer".to_string(),
            OriginProtocolPolicy::HttpsOnly => "https-only".to_string(),
        }
    }
}

pub enum IpAddressType {
    IPv4,
    IPv6,
    Dualstack,
}

impl From<IpAddressType> for String {
    fn from(value: IpAddressType) -> Self {
        match value {
            IpAddressType::IPv4 => "ipv4".to_string(),
            IpAddressType::IPv6 => "ipv6".to_string(),
            IpAddressType::Dualstack => "dualstack".to_string(),
        }
    }
}

type_state!(
    OriginState,
    OriginStartState,
    OriginS3OriginState,
    OriginCustomOriginState,
);

// TODO more origins

/// Builder for CloudFront distribution origins.
pub struct OriginBuilder<T: OriginState> {
    phantom_data: PhantomData<T>,
    id: String,
    bucket_arn: Option<Value>,
    bucket_ref: Option<Value>,
    domain_name: Option<Value>,
    connection_attempts: Option<u16>,
    connection_timeout: Option<u16>,
    response_completion_timeout: Option<u16>,
    origin_access_control_id: Option<Value>,
    origin_path: Option<String>,
    s3origin_config: Option<S3OriginConfig>,
    origin_custom_headers: Option<Vec<OriginCustomHeader>>,
    vpc_origin_config: Option<VpcOriginConfig>,
    custom_origin_config: Option<CustomOriginConfig>,
}

impl OriginBuilder<OriginStartState> {
    pub fn new(origin_id: &str) -> Self {
        Self {
            phantom_data: Default::default(),
            id: origin_id.to_string(),
            bucket_arn: None,
            bucket_ref: None,
            domain_name: None,
            connection_attempts: None,
            connection_timeout: None,
            origin_access_control_id: None,
            origin_path: None,
            response_completion_timeout: None,
            s3origin_config: None,
            origin_custom_headers: None,
            vpc_origin_config: None,
            custom_origin_config: None,
        }
    }

    /// Configures an S3 bucket as the origin.
    ///
    /// Automatically creates a bucket policy allowing CloudFront access via Origin Access Control.
    pub fn s3_origin(
        self,
        bucket: &BucketRef,
        oac: &OriginAccessControlRef,
        origin_read_timeout: Option<S3OriginReadTimeout>,
    ) -> OriginBuilder<OriginS3OriginState> {
        let s3origin_config = S3OriginConfig {
            origin_read_timeout: origin_read_timeout.map(|v| v.0),
        };

        let domain = bucket.get_att("RegionalDomainName");

        OriginBuilder {
            phantom_data: Default::default(),
            id: self.id.to_string(),
            bucket_arn: Some(bucket.get_arn()),
            bucket_ref: Some(bucket.get_ref()),
            domain_name: Some(domain),
            connection_attempts: self.connection_attempts,
            connection_timeout: self.connection_timeout,
            origin_access_control_id: Some(oac.get_att("Id")),
            origin_path: self.origin_path,
            response_completion_timeout: self.response_completion_timeout,
            origin_custom_headers: self.origin_custom_headers,
            s3origin_config: Some(s3origin_config),
            vpc_origin_config: None,
            custom_origin_config: None,
        }
    }
    
    // TODO add test
    //  and could also add additional methods for ELB etc. that pass in the ELB, to have extra safety

    /// Configures a custom origin.
    pub fn custom_origin(self, domain: &str, policy: OriginProtocolPolicy) -> OriginBuilder<OriginCustomOriginState> {
        let custom_origin_config = CustomOriginConfig {
            origin_protocol_policy: policy.into(),
            http_port: None,
            https_port: None,
            ip_address_type: None,
            origin_keep_alive_timeout: None,
            origin_read_timeout: None,
            origin_ssl_protocols: None,
        };

        OriginBuilder {
            phantom_data: Default::default(),
            id: self.id.to_string(),
            domain_name: Some(Value::String(domain.to_string())),
            connection_attempts: self.connection_attempts,
            connection_timeout: self.connection_timeout,
            origin_path: self.origin_path,
            response_completion_timeout: self.response_completion_timeout,
            origin_custom_headers: self.origin_custom_headers,
            custom_origin_config: Some(custom_origin_config),
            vpc_origin_config: None,
            origin_access_control_id: None,
            s3origin_config: None,
            bucket_arn: None,
            bucket_ref: None,
        }
    }
}

impl<T: OriginState> OriginBuilder<T> {
    pub fn connection_attempts(self, attempts: ConnectionAttempts) -> Self {
        Self {
            connection_attempts: Some(attempts.0),
            ..self
        }
    }

    pub fn timeouts(self, timeouts: CfConnectionTimeout) -> Self {
        Self {
            connection_timeout: timeouts.0,
            response_completion_timeout: timeouts.1,
            ..self
        }
    }

    pub fn origin_path(self, path: OriginPath) -> Self {
        Self {
            origin_path: Some(path.0),
            ..self
        }
    }

    fn build_internal(self) -> Origin {
        Origin {
            id: self.id,
            s3_bucket_policy: None,
            domain_name: self.domain_name.expect("domain name should be present for cloudfront distribution"),
            connection_attempts: self.connection_attempts,
            connection_timeout: self.connection_timeout,
            origin_access_control_id: self.origin_access_control_id,
            origin_path: self.origin_path,
            response_completion_timeout: self.response_completion_timeout,
            s3origin_config: self.s3origin_config,
            origin_custom_headers: self.origin_custom_headers,
            vpc_origin_config: self.vpc_origin_config,
            custom_origin_config: self.custom_origin_config,
        }
    }
}

impl OriginBuilder<OriginCustomOriginState> {
    pub fn ip_address_type(self, address_type: IpAddressType) -> Self {
        let mut config = self.custom_origin_config.expect("custom config to be present in Custom Origin State");
        config.ip_address_type = Some(address_type.into());

        OriginBuilder {
            custom_origin_config: Some(config),
            ..self
        }
    }
    pub fn http_port(self, port: u16) -> Self {
        let mut config = self.custom_origin_config.expect("custom config to be present in Custom Origin State");
        config.http_port = Some(port);

        OriginBuilder {
            custom_origin_config: Some(config),
            ..self
        }
    }

    pub fn https_port(self, port: u16) -> Self {
        let mut config = self.custom_origin_config.expect("custom config to be present in Custom Origin State");
        config.https_port = Some(port);

        OriginBuilder {
            custom_origin_config: Some(config),
            ..self
        }
    }

    pub fn origin_keep_alive_timeout(self, timeout: u8) -> Self {
        let mut config = self.custom_origin_config.expect("custom config to be present in Custom Origin State");
        config.origin_keep_alive_timeout = Some(timeout);

        OriginBuilder {
            custom_origin_config: Some(config),
            ..self
        }
    }

    pub fn origin_read_timeout(self, timeout: u8) -> Self {
        let mut config = self.custom_origin_config.expect("custom config to be present in Custom Origin State");
        config.origin_read_timeout = Some(timeout);

        OriginBuilder {
            custom_origin_config: Some(config),
            ..self
        }
    }

    pub fn add_origin_ssl_protocol(self, protocol: String) -> Self {
        let mut config = self.custom_origin_config.expect("custom config to be present in Custom Origin State");

        let protocols = if let Some(mut protocols) = config.origin_ssl_protocols {
            protocols.push(protocol);
            protocols
        } else {
            vec![protocol]
        };

        config.origin_ssl_protocols = Some(protocols);

        OriginBuilder {
            custom_origin_config: Some(config),
            ..self
        }
    }

    pub fn build(self) -> Origin {
        self.build_internal()
    }
}

impl OriginBuilder<OriginS3OriginState> {
    pub fn build(mut self) -> Origin {
        let bucket_ref = self.bucket_ref.take().expect("bucket ref to be present in S3 origin state");
        let bucket_arn = self.bucket_arn.take().expect("bucket arn to be present in S3 origin state");

        let bucket_items = vec![join("", vec![bucket_arn, Value::String("/*".to_string())])];
        let statement = StatementBuilder::new(vec![IamAction("s3:GetObject".to_string())], Effect::Allow)
            .resources(bucket_items)
            .principal(Service(ServicePrincipal {
                service: "cloudfront.amazonaws.com".to_string(),
            }))
            .build();
        let doc = PolicyDocumentBuilder::new(vec![statement]).build();
        let bucket_policy_id = format!("{}-website-s3-policy", self.id);
        let (_, s3_policy) = BucketPolicyBuilder::new_with_bucket_ref(bucket_policy_id.as_str(), bucket_ref, doc).raw_build();

        let mut origin = self.build_internal();
        origin.s3_bucket_policy = Some(s3_policy);

        origin
    }
}

pub enum DefaultCacheAllowedMethods {
    GetHead,
    GetHeadOptions,
    All,
}

impl From<DefaultCacheAllowedMethods> for Vec<String> {
    fn from(value: DefaultCacheAllowedMethods) -> Self {
        match value {
            DefaultCacheAllowedMethods::GetHead => vec![Get.into(), Head.into()],
            DefaultCacheAllowedMethods::GetHeadOptions => vec![Get.into(), Head.into(), Options.into()],
            DefaultCacheAllowedMethods::All => vec![
                Get.into(),
                Head.into(),
                Options.into(),
                Put.into(),
                Patch.into(),
                Post.into(),
                Delete.into(),
            ],
        }
    }
}

pub enum DefaultCacheCachedMethods {
    GetHead,
    GetHeadOptions,
}

impl From<DefaultCacheCachedMethods> for Vec<String> {
    fn from(value: DefaultCacheCachedMethods) -> Self {
        match value {
            DefaultCacheCachedMethods::GetHead => vec![Get.into(), Head.into()],
            DefaultCacheCachedMethods::GetHeadOptions => vec![Get.into(), Head.into(), Options.into()],
        }
    }
}

pub enum ViewerProtocolPolicy {
    AllowAll,
    RedirectToHttps,
    HttpsOnly,
}

impl From<ViewerProtocolPolicy> for String {
    fn from(value: ViewerProtocolPolicy) -> Self {
        match value {
            ViewerProtocolPolicy::AllowAll => "allow-all".to_string(),
            ViewerProtocolPolicy::RedirectToHttps => "redirect-to-https".to_string(),
            ViewerProtocolPolicy::HttpsOnly => "https-only".to_string(),
        }
    }
}

/// Builder for CloudFront default cache behavior.
pub struct DefaultCacheBehaviorBuilder {
    target_origin_id: String,
    cache_policy_id: Value,
    viewer_protocol_policy: String,
    allowed_methods: Option<Vec<String>>,
    cached_methods: Option<Vec<String>>,
    compress: Option<bool>,
}

impl DefaultCacheBehaviorBuilder {
    pub fn new(origin: &Origin, policy: &CachePolicyRef, viewer_protocol_policy: ViewerProtocolPolicy) -> Self {
        Self {
            target_origin_id: origin.get_origin_id().to_string(),
            cache_policy_id: policy.get_att("Id"),
            viewer_protocol_policy: viewer_protocol_policy.into(),
            allowed_methods: None,
            cached_methods: None,
            compress: None,
        }
    }

    pub fn allowed_methods(self, methods: DefaultCacheAllowedMethods) -> Self {
        Self {
            allowed_methods: Some(methods.into()),
            target_origin_id: self.target_origin_id,
            cache_policy_id: self.cache_policy_id,
            viewer_protocol_policy: self.viewer_protocol_policy,
            cached_methods: self.cached_methods,
            compress: self.compress,
        }
    }

    pub fn cached_methods(self, methods: DefaultCacheCachedMethods) -> Self {
        Self {
            cached_methods: Some(methods.into()),
            target_origin_id: self.target_origin_id,
            cache_policy_id: self.cache_policy_id,
            viewer_protocol_policy: self.viewer_protocol_policy,
            allowed_methods: self.allowed_methods,
            compress: self.compress,
        }
    }

    pub fn compress(self, compress: bool) -> Self {
        Self {
            compress: Some(compress),
            target_origin_id: self.target_origin_id,
            cache_policy_id: self.cache_policy_id,
            viewer_protocol_policy: self.viewer_protocol_policy,
            allowed_methods: self.allowed_methods,
            cached_methods: self.cached_methods,
        }
    }

    pub fn build(self) -> DefaultCacheBehavior {
        DefaultCacheBehavior {
            target_origin_id: self.target_origin_id,
            cache_policy_id: self.cache_policy_id,
            viewer_protocol_policy: self.viewer_protocol_policy,
            allowed_methods: self.allowed_methods,
            cached_methods: self.cached_methods,
            compress: self.compress,
        }
    }
}

type_state!(
    DistributionState,
    DistributionStartState,
    DistributionOriginState,
);

pub enum SigningBehavior {
    Never,
    NoOverride,
    Always,
}

impl From<SigningBehavior> for String {
    fn from(value: SigningBehavior) -> Self {
        match value {
            SigningBehavior::Never => "never".to_string(),
            SigningBehavior::NoOverride => "no-override".to_string(),
            SigningBehavior::Always => "always".to_string(),
        }
    }
}

pub enum SigningProtocol {
    SigV4,
}

impl From<SigningProtocol> for String {
    fn from(value: SigningProtocol) -> Self {
        match value {
            SigningProtocol::SigV4 => "sigv4".to_string(),
        }
    }
}

pub enum OriginAccessControlType {
    S3,
    MediaStore,
    Lambda,
    MediaPackageV2,
}

impl From<OriginAccessControlType> for String {
    fn from(value: OriginAccessControlType) -> Self {
        match value {
            OriginAccessControlType::S3 => "s3".to_string(),
            OriginAccessControlType::MediaStore => "mediastore".to_string(),
            OriginAccessControlType::Lambda => "lambda".to_string(),
            OriginAccessControlType::MediaPackageV2 => "mediapackagev2".to_string(),
        }
    }
}

/// Builder for CloudFront Origin Access Control.
///
/// Controls access from CloudFront to origins like S3 buckets.
pub struct OriginAccessControlBuilder {
    id: Id,
    name: String,
    origin_access_control_type: OriginAccessControlType,
    signing_behavior: SigningBehavior,
    signing_protocol: SigningProtocol,
}

impl OriginAccessControlBuilder {
    /// Creates a new CloudFront Origin Access Control builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the origin access control
    /// * `name` - Name of the origin access control
    /// * `origin_access_control_type` - Type of origin (S3, MediaStore, Lambda, etc.)
    /// * `signing_behavior` - When to sign requests (Never, NoOverride, Always)
    /// * `signing_protocol` - Protocol for signing requests
    pub fn new(
        id: &str,
        name: &str,
        origin_access_control_type: OriginAccessControlType,
        signing_behavior: SigningBehavior,
        signing_protocol: SigningProtocol,
    ) -> Self {
        Self {
            id: Id(id.to_string()),
            name: name.to_string(),
            origin_access_control_type,
            signing_behavior,
            signing_protocol,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> OriginAccessControlRef {
        let resource_id = Resource::generate_id("OAC");
        stack_builder.add_resource(OriginAccessControl {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::CloudFront::OriginAccessControl".to_string(),
            properties: OriginControlProperties {
                config: OriginAccessControlConfig {
                    name: self.name,
                    origin_access_control_type: self.origin_access_control_type.into(),
                    signing_behavior: self.signing_behavior.into(),
                    signing_protocol: self.signing_protocol.into(),
                },
            },
        });
        OriginAccessControlRef::new(resource_id)
    }
}

/// Builder for CloudFront distributions.
///
/// Creates a CloudFront distribution with origins, cache behaviors, and other configuration.
///
/// # Example
///
/// ```rust,no_run
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::cloudfront::{DistributionBuilder, OriginBuilder, DefaultCacheBehaviorBuilder};
/// use rusty_cdk_core::s3::BucketBuilder;
/// use rusty_cdk_core::wrappers::*;
///
/// let mut stack_builder = StackBuilder::new();
///
/// let bucket = unimplemented!("create a bucket");
/// let oac = unimplemented!("create an origin access control");
/// let policy = unimplemented!("create an origin");
/// let viewer_protocol_policy = unimplemented!("create a viewer protocol");
///
/// let origin = OriginBuilder::new("my-origin").s3_origin(&bucket, &oac, None).build();
/// let cache_behavior = DefaultCacheBehaviorBuilder::new(&origin, &policy, viewer_protocol_policy).build();
///
/// let distribution = DistributionBuilder::new("my-distribution", cache_behavior)
///     .origins(vec![origin])
///     .build(&mut stack_builder);
/// ```
pub struct DistributionBuilder<T: DistributionState> {
    phantom_data: PhantomData<T>,
    id: Id,
    enabled: bool,
    default_cache_behavior: DefaultCacheBehavior,
    price_class: Option<String>,
    http_version: Option<String>,
    aliases: Option<Vec<String>>,
    cnames: Option<Vec<String>>,
    ipv6_enabled: Option<bool>,
    viewer_certificate: Option<ViewerCertificate>,
    cache_behaviors: Option<Vec<CacheBehavior>>,
    default_root_object: Option<String>,
    // TODO add. and either this or the next is required!
    // origin_groups: Option<OriginGroups>,
    origins: Option<Vec<Origin>>,
}

impl DistributionBuilder<DistributionStartState> {
    /// Creates a new CloudFront distribution builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the distribution
    /// * `default_cache_behavior` - Default cache behavior for all requests
    pub fn new(id: &str, default_cache_behavior: DefaultCacheBehavior) -> Self {
        Self {
            phantom_data: Default::default(),
            id: Id(id.to_string()),
            enabled: true,
            default_cache_behavior,
            aliases: None,
            cache_behaviors: None,
            cnames: None,
            default_root_object: None,
            http_version: None,
            ipv6_enabled: None,
            origins: None,
            price_class: None,
            viewer_certificate: None,
        }
    }

    pub fn origins(self, origins: Vec<Origin>) -> DistributionBuilder<DistributionOriginState> {
        DistributionBuilder {
            phantom_data: Default::default(),
            origins: Some(origins),
            id: self.id,
            enabled: self.enabled,
            default_cache_behavior: self.default_cache_behavior,
            price_class: self.price_class,
            http_version: self.http_version,
            aliases: self.aliases,
            cnames: self.cnames,
            ipv6_enabled: self.ipv6_enabled,
            viewer_certificate: self.viewer_certificate,
            cache_behaviors: self.cache_behaviors,
            default_root_object: self.default_root_object,
        }
    }
}

impl DistributionBuilder<DistributionOriginState> {
    pub fn build(mut self, stack_builder: &mut StackBuilder) -> DistributionRef {
        let mut origins = self.origins.take().expect("origins to be present in distribution origin state");
        let resource_id = Resource::generate_id("CloudFrontDistribution");

        origins
            .iter_mut()
            .filter(|o| o.s3_bucket_policy.is_some())
            .map(|s3| {
                let mut policy = s3
                    .s3_bucket_policy
                    .take()
                    .expect("just checked that this was present, only need to use it this one time");
                let distro_id = get_att(&resource_id, "Id");
                let source_arn_value = join(
                    "",
                    vec![
                        Value::String("arn:aws:cloudfront::".to_string()),
                        get_ref("AWS::AccountId"),
                        Value::String(":distribution/".to_string()),
                        distro_id,
                    ],
                );
                let distro_condition = json!({
                    "StringEquals": {
                        "AWS:SourceArn": source_arn_value
                    }
                });
                policy
                    .properties
                    .policy_document
                    .statements
                    .iter_mut()
                    .for_each(|v| v.condition = Some(distro_condition.clone()));
                policy
            })
            .for_each(|p| {
                stack_builder.add_resource(p);
            });

        self.origins = Some(origins);

        self.build_internal(resource_id, stack_builder)
    }
}

impl<T: DistributionState> DistributionBuilder<T> {
    pub fn add_cache_behavior(mut self, behavior: CacheBehavior) -> Self {
        if let Some(mut behaviors) = self.cache_behaviors {
            behaviors.push(behavior);
            self.cache_behaviors = Some(behaviors);
        } else {
            self.cache_behaviors = Some(vec![behavior])
        }
        self
    }

    pub fn aliases(self, aliases: Vec<String>) -> Self {
        Self {
            aliases: Some(aliases),
            ..self
        }
    }

    // could have a regex for this?
    pub fn cnames(self, cnames: Vec<String>) -> Self {
        Self {
            cnames: Some(cnames),
            ..self
        }
    }

    pub fn price_class(self, price_class: PriceClass) -> Self {
        Self {
            price_class: Some(price_class.into()),
            ..self
        }
    }

    pub fn http_version(self, http_version: HttpVersion) -> Self {
        Self {
            http_version: Some(http_version.into()),
            ..self
        }
    }

    pub fn ipv6_enabled(self, enabled: bool) -> Self {
        Self {
            ipv6_enabled: Some(enabled),
            ..self
        }
    }

    pub fn viewer_certificate(self, viewer_certificate: ViewerCertificate) -> Self {
        Self {
            viewer_certificate: Some(viewer_certificate),
            ..self
        }
    }

    pub fn enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }

    pub fn default_root_object(self, default: DefaultRootObject) -> Self {
        Self {
            default_root_object: Some(default.0),
            ..self
        }
    }

    fn build_internal(self, resource_id: String, stack_builder: &mut StackBuilder) -> DistributionRef {
        let config = DistributionConfig {
            enabled: self.enabled,
            default_cache_behavior: self.default_cache_behavior,
            aliases: self.aliases,
            cache_behaviors: self.cache_behaviors,
            cnames: self.cnames,
            default_root_object: self.default_root_object.unwrap_or_default(),
            http_version: self.http_version,
            ipv6_enabled: self.ipv6_enabled,
            price_class: self.price_class,
            viewer_certificate: self.viewer_certificate,
            origins: self.origins,
            origin_groups: None,
        };
        stack_builder.add_resource(Distribution {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::CloudFront::Distribution".to_string(),
            properties: DistributionProperties { config },
        });

        DistributionRef::new(resource_id)
    }
}

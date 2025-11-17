use std::marker::PhantomData;
use serde_json::{json, Value};
use crate::cloudfront::{CacheBehavior, CachePolicy, CachePolicyProperties, CloudFrontDistribution, CloudFrontDistributionProperties, CloudFrontOriginAccessIdentity, CloudFrontOriginAccessIdentityConfig, CloudFrontOriginAccessIdentityProperties, CookiesConfig, DefaultCacheBehavior, DistributionConfig, HeadersConfig, Origin, OriginCustomHeader, ParametersInCacheKeyAndForwardedToOrigin, QueryStringsConfig, S3OriginConfig, ViewerCertificate, VpcOriginConfig};
use crate::iam::{Effect, PolicyDocumentBuilder, ServicePrincipal, StatementBuilder};
use crate::iam::IamPrincipal::{Service};
use crate::intrinsic_functions::{get_att, get_ref, join};
use crate::s3::builder::S3BucketPolicyBuilder;
use crate::s3::dto::{S3Bucket, S3BucketPolicy};
use crate::shared::Id;
use crate::stack::Resource;
use crate::wrappers::{ConnectionAttempts, CfConnectionTimeout, OriginPath, S3OriginReadTimeout, IamAction, DefaultRootObject};

pub struct CloudFrontOriginAccessIdentityBuilder {
    id: Id,
    comment: String,
}

impl CloudFrontOriginAccessIdentityBuilder {
    pub fn new(id: &str, describing_comment: String) -> Self {
        Self {
            id: Id(id.to_string()),
            comment: describing_comment,
        }
    }

    pub fn build(self) -> CloudFrontOriginAccessIdentity {
        CloudFrontOriginAccessIdentity {
            id: self.id,
            resource_id: Resource::generate_id("CloudFrontOriginAccessIdentity"),
            r#type: "AWS::CloudFront::CloudFrontOriginAccessIdentity".to_string(),
            properties: CloudFrontOriginAccessIdentityProperties {
                cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig { comment: self.comment },
            },
        }
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

pub struct CachePolicyBuilder {
    id: Id,
    name: String,
    default_ttl: u32,
    min_ttl: u32,
    max_ttl: u32,
    cache_params: ParametersInCacheKeyAndForwardedToOrigin,
}

impl CachePolicyBuilder {
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

    pub fn build(self) -> CachePolicy {
        CachePolicy {
            id: self.id,
            resource_id: Resource::generate_id("CachePolicy"),
            r#type: "AWS::CloudFront::CachePolicy".to_string(),
            properties: CachePolicyProperties {
                default_ttl: self.default_ttl,
                min_ttl: self.min_ttl,
                max_ttl: self.max_ttl,
                name: self.name,
                params_in_cache_key_and_forwarded: self.cache_params,
            },
        }
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

pub trait OriginState {}

pub struct OriginStartState {}
impl OriginState for OriginStartState {}

pub struct OriginS3OriginState {}
impl OriginState for OriginS3OriginState {}

// TODO other origins
// TODO for s3 origin you need a bucket policy, a CloudFrontOriginAccessIdentity, and a link to the distro
pub struct OriginBuilder<'a, T: OriginState> {
    phantom_data: PhantomData<T>,
    id: String,
    referenced_ids: Vec<String>,
    bucket: Option<&'a S3Bucket>,
    domain_name: String,
    connection_attempts: Option<u16>,
    connection_timeout: Option<u16>,
    response_completion_timeout: Option<u16>,
    origin_access_control_id: Option<String>,
    origin_path: Option<String>,
    s3origin_config: Option<S3OriginConfig>,
    origin_custom_headers: Option<Vec<OriginCustomHeader>>,
    vpc_origin_config: Option<VpcOriginConfig>,
}

impl OriginBuilder<'_, OriginStartState> {
    // could maybe validate domain name better if part of enum
    pub fn new(id: &str, domain_name: &str) -> Self {
        Self {
            phantom_data: Default::default(),
            id: id.to_string(),
            referenced_ids: vec![],
            bucket: None,
            domain_name: domain_name.to_string(),
            connection_attempts: None,
            connection_timeout: None,
            origin_access_control_id: None,
            origin_path: None,
            response_completion_timeout: None,
            s3origin_config: None,
            origin_custom_headers: None,
            vpc_origin_config: None,
        }
    }

    pub fn s3_origin(mut self, bucket: &S3Bucket, origin_read_timeout: Option<S3OriginReadTimeout>) -> OriginBuilder<'_, OriginS3OriginState> {
        self.referenced_ids.push(bucket.get_resource_id().to_string());

        let s3origin_config = S3OriginConfig {
            origin_read_timeout: origin_read_timeout.map(|v| v.0),
        };

        OriginBuilder {
            phantom_data: Default::default(),
            id: self.id.to_string(),
            referenced_ids: self.referenced_ids,
            bucket: Some(bucket),
            domain_name: self.domain_name,
            connection_attempts: self.connection_attempts,
            connection_timeout: self.connection_timeout,
            origin_access_control_id: self.origin_access_control_id,
            origin_path: self.origin_path,
            response_completion_timeout: self.response_completion_timeout,
            origin_custom_headers: self.origin_custom_headers,
            s3origin_config: Some(s3origin_config),
            vpc_origin_config: None,
        }
    }
}


impl<T: OriginState> OriginBuilder<'_, T> {
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

    // TODO better check
    pub fn origin_access_control_id(self, id: String) -> Self {
        Self {
            origin_access_control_id: Some(id),
            ..self
        }
    }

    pub fn origin_path(self, path: OriginPath) -> Self {
        Self {
            origin_access_control_id: Some(path.0),
            ..self
        }
    }

    fn build_internal(self) -> Origin {
        Origin {
            id: self.id,
            referenced_ids: self.referenced_ids,
            s3_bucket_policy: None,
            domain_name: self.domain_name,
            connection_attempts: self.connection_attempts,
            connection_timeout: self.connection_timeout,
            origin_access_control_id: self.origin_access_control_id,
            origin_path: self.origin_path,
            response_completion_timeout: self.response_completion_timeout,
            s3origin_config: self.s3origin_config,
            origin_custom_headers: self.origin_custom_headers,
            vpc_origin_config: self.vpc_origin_config,
        }
    }
}

impl OriginBuilder<'_, OriginS3OriginState> {
    pub fn build(mut self) -> Origin {
        let bucket = self.bucket.take().expect("bucket to be present in S3 origin state");
        
        let bucket_items = vec![join("", vec![bucket.get_arn(), Value::String("/*".to_string())])];
        let statement = StatementBuilder::new(vec![IamAction("s3:GetObject".to_string())], Effect::Allow)
            .resources(bucket_items)
            .principal(Service(ServicePrincipal {
                service: "cloudfront.amazonaws.com".to_string(),
            }))
            .build();
        let doc = PolicyDocumentBuilder::new(vec![statement]);
        let bucket_policy_id = format!("{}-website-s3-policy", self.id);
        let s3_policy = S3BucketPolicyBuilder::new(bucket_policy_id.as_str(), &bucket, doc).build();

        let mut origin = self.build_internal();
        origin.s3_bucket_policy = Some(s3_policy);

        origin
    }
}

pub trait CloudFrontDistributionState {}
pub struct CloudFrontDistributionStartState {}
impl CloudFrontDistributionState for CloudFrontDistributionStartState {}
pub struct CloudFrontDistributionOriginState {}
impl CloudFrontDistributionState for CloudFrontDistributionOriginState {}

pub struct CloudFrontDistributionBuilder<T: CloudFrontDistributionState> {
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
    // origin_groups: Option<OriginGroups>, // TODO add. and either this or the next is required!
    origins: Option<Vec<Origin>>,
}

impl CloudFrontDistributionBuilder<CloudFrontDistributionStartState> {
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
    
    pub fn origins(self, origins: Vec<Origin>) -> CloudFrontDistributionBuilder<CloudFrontDistributionOriginState> {
        CloudFrontDistributionBuilder {
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

impl CloudFrontDistributionBuilder<CloudFrontDistributionStartState> {
    #[must_use]
    pub fn build(mut self) ->  (CloudFrontDistribution, Vec<S3BucketPolicy>) {
        let mut origins = self.origins.take().expect("origins to be present in distribution origin state");

        let policies: Vec<_> = origins.iter_mut().filter(|o| o.s3_bucket_policy.is_some()).map(|s3| {
            let mut policy = s3.s3_bucket_policy.take().expect("just checked that this was present, only need to use it this one time");
            let distro_id = get_att(&self.id, "Id");
            let source_arn_value = join("", vec![Value::String("arn:aws:cloudfront::".to_string()), get_ref("AWS::AccountId"), Value::String(":distribution/".to_string()), distro_id]);
            let distro_condition = json!({
                "StringEquals": {
                    "AWS:SourceArn": source_arn_value
                }
            });
            policy.properties.policy_document.statements.iter_mut().for_each(|v| {
                v.condition = Some(distro_condition.clone())
            });
            policy
        }).collect();

        self.origins = Some(origins);

        let distro = self.build_internal();

        (distro, policies)
    }
}

impl<T: CloudFrontDistributionState> CloudFrontDistributionBuilder<T> {
    pub fn add_cache_behavior(mut self, behavior: CacheBehavior) -> Self {
        if let Some(mut behaviors) = self.cache_behaviors {
            behaviors.push(behavior);
            self.cache_behaviors = Some(behaviors);
        } else {
            self.cache_behaviors = Some(vec![behavior])
        }
        self
    }

    // TODO probably can limit possible values this further
    pub fn aliases(self, aliases: Vec<String>) -> Self {
        Self {
            aliases: Some(aliases),
            ..self
        }
    }

    // TODO probably can limit possible values this further
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

    fn build_internal(self) -> CloudFrontDistribution {
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
        CloudFrontDistribution {
            id: self.id,
            resource_id: Resource::generate_id("CloudFrontDistribution"),
            r#type: "AWS::CloudFront::Distribution".to_string(),
            properties: CloudFrontDistributionProperties { config },
        }
    }
}

use crate::cloudfront::{
    CacheBehavior, CachePolicy, CachePolicyProperties, CloudFrontDistribution, CloudFrontDistributionProperties,
    CloudFrontOriginAccessIdentity, CloudFrontOriginAccessIdentityConfig, CloudFrontOriginAccessIdentityProperties, CookiesConfig,
    DefaultCacheBehavior, DistributionConfig, HeadersConfig, Origin, OriginGroups, ParametersInCacheKeyAndForwardedToOrigin,
    QueryStringsConfig, ViewerCertificate,
};
use crate::shared::Id;
use crate::stack::Resource;

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

pub struct CloudFrontDistributionBuilder {
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
    
    default_root_object: Option<String>, //  => requires some special work if empty?
    origin_groups: Option<OriginGroups>, // TODO either this or the next is required!
    origins: Option<Vec<Origin>>,
}

impl CloudFrontDistributionBuilder {
    pub fn new(id: &str, default_cache_behavior: DefaultCacheBehavior) -> Self {
        Self {
            id: Id(id.to_string()),
            enabled: true,
            default_cache_behavior,
            aliases: None,
            cache_behaviors: None,
            cnames: None,
            default_root_object: None,
            http_version: None,
            ipv6_enabled: None,
            origin_groups: None,
            origins: None,
            price_class: None,
            viewer_certificate: None,
        }
    }

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

    pub fn build(self) -> CloudFrontDistribution {
        let config = DistributionConfig {
            enabled: self.enabled,
            default_cache_behavior: self.default_cache_behavior,
            aliases: self.aliases,
            cache_behaviors: self.cache_behaviors,
            cnames: self.cnames,
            default_root_object: self.default_root_object,
            http_version: self.http_version,
            ipv6_enabled: self.ipv6_enabled,
            origin_groups: self.origin_groups,
            origins: self.origins,
            price_class: self.price_class,
            viewer_certificate: self.viewer_certificate,
        };
        CloudFrontDistribution {
            id: self.id,
            resource_id: Resource::generate_id("CloudFrontDistribution"),
            r#type: "AWS::CloudFront::Distribution".to_string(),
            properties: CloudFrontDistributionProperties { config },
        }
    }
}

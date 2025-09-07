use std::marker::PhantomData;
use std::time::Duration;
use crate::s3::dto;
use crate::s3::dto::{CorsConfiguration, CorsRule, LifecycleConfiguration, PublicAccessBlockConfiguration, RedirectAllRequestsTo, S3Bucket, S3BucketProperties, WebsiteConfiguration};
use crate::shared::http::{HttpMethod, Protocol};
use crate::shared::Id;
use crate::stack::Resource;
use crate::wrappers::BucketName;

pub enum VersioningConfiguration {
    Enabled,
    Suspended,
}

impl From<VersioningConfiguration> for String {
    fn from(value: VersioningConfiguration) -> Self {
        match value {
            VersioningConfiguration::Enabled => "Enabled".to_string(),
            VersioningConfiguration::Suspended => "Suspended".to_string()
        }
    }
}

// TODO bucket encryption
// TODO notifications will require custom work to avoid circular dependencies... Maybe borrow code from cdk?

pub trait S3BucketBuilderState {}

pub struct StartState {}
impl S3BucketBuilderState for StartState {}

pub struct WebsiteState {}
impl S3BucketBuilderState for WebsiteState {}

pub struct S3BucketBuilder<T: S3BucketBuilderState> {
    phantom_data: PhantomData<T>,
    id: Id,
    name: Option<String>,
    access: Option<PublicAccessBlockConfiguration>,
    versioning_configuration: Option<VersioningConfiguration>,
    lifecycle_configuration: Option<LifecycleConfiguration>,
    index_document: Option<String>,
    error_document: Option<String>,
    redirect_all_requests_to: Option<(String, Option<Protocol>)>,
    cors_config: Option<CorsConfiguration>,
}

impl S3BucketBuilder<StartState> {
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            phantom_data: Default::default(),
            name: None,
            access: None,
            versioning_configuration: None,
            lifecycle_configuration: None,
            index_document: None,
            error_document: None,
            redirect_all_requests_to: None,
            cors_config: None,
        }
    }

    pub fn build(self) -> S3Bucket {
        self.build_internal(false)
    }
}

impl<T: S3BucketBuilderState> S3BucketBuilder<T> {
    pub fn name(self, name: BucketName) -> Self {
        Self {
            name: Some(name.0),
            ..self
        }
    }

    pub fn versioning_configuration(self, config: VersioningConfiguration) -> Self {
        Self {
            versioning_configuration: Some(config),
            ..self
        }
    }

    pub fn lifecycle_configuration(self, config: LifecycleConfiguration) -> Self {
        Self {
            lifecycle_configuration: Some(config),
            ..self
        }
    }

    pub fn public_access_block_configuration(self, access: PublicAccessBlockConfiguration) -> Self {
        Self {
            access: Some(access),
            ..self
        }
    }

    pub fn website(self) -> S3BucketBuilder<WebsiteState> {
        S3BucketBuilder {
            phantom_data: Default::default(),
            id: self.id,
            name: self.name,
            access: self.access,
            versioning_configuration: self.versioning_configuration,
            lifecycle_configuration: self.lifecycle_configuration,
            index_document: self.index_document,
            error_document: self.error_document,
            redirect_all_requests_to: self.redirect_all_requests_to,
            cors_config: self.cors_config,
        }
    }

    fn build_internal(self, website: bool) -> S3Bucket {
        let id = Resource::generate_id("S3Bucket");

        let versioning_configuration = self.versioning_configuration.map(|c| {
            dto::VersioningConfiguration {
                status: c.into(),
            }
        });

        let website_configuration = if website {
            let redirect_all_requests_to = self.redirect_all_requests_to.map(|r| {
                RedirectAllRequestsTo {
                    host_name: r.0,
                    protocol: r.1.map(Into::into),
                }
            });

            Some(WebsiteConfiguration {
                index_document: self.index_document,
                error_document: self.error_document,
                redirect_all_requests_to,
            })
        } else {
            None
        };

        let properties = S3BucketProperties {
            bucket_name: self.name,
            cors_configuration: self.cors_config,
            lifecycle_configuration: self.lifecycle_configuration,
            public_access_block_configuration: self.access,
            versioning_configuration,
            website_configuration,
            bucket_encryption: None,
            notification_configuration: None,
        };

        S3Bucket {
            id: self.id,
            resource_id: id,
            r#type: "AWS::S3::Bucket".to_string(),
            properties,
        }
    }
}

impl S3BucketBuilder<WebsiteState> {
    pub fn index_document(self, doc: String) -> Self {
        Self {
            index_document: Some(doc),
            ..self
        }
    }
    pub fn error_document(self, error: String) -> Self {
        Self {
            error_document: Some(error),
            ..self
        }
    }

    pub fn redirect_all(self, hostname: String, protocol: Option<Protocol>) -> Self {
        Self {
            redirect_all_requests_to: Some((hostname, protocol)),
            ..self
        }
    }

    pub fn cors_config(self, config: CorsConfiguration) -> Self {
        Self {
            cors_config: Some(config),
            ..self
        }
    }

    pub fn build(self) -> S3Bucket {
        self.build_internal(true)
    }
}

pub struct CorsConfigurationBuilder {
    rules: Vec<CorsRule>
}

impl CorsConfigurationBuilder {
    pub fn new(cors_rules: Vec<CorsRule>) -> CorsConfiguration {
        CorsConfiguration {
            cors_rules,
        }
    }
}

pub struct CorsRuleBuilder {
    allow_origins: Vec<String>,
    allow_methods: Vec<HttpMethod>,
    allow_headers: Option<Vec<String>>,
    expose_headers: Option<Vec<String>>,
    max_age: Option<u64>,
}

impl CorsRuleBuilder {
    pub fn new(allow_origins: Vec<String>, allow_methods: Vec<HttpMethod>) -> Self {
        Self {
            allow_origins,
            allow_methods,
            allow_headers: None,
            expose_headers: None,
            max_age: None,
        }
    }

    pub fn allow_headers(self, headers: Vec<String>) -> Self {
        Self {
            allow_headers: Some(headers),
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

    pub fn build(self) -> CorsRule {
        CorsRule {
            allowed_headers: self.allow_headers,
            allowed_methods: self.allow_methods.into_iter().map(Into::into).collect(),
            allowed_origins: self.allow_origins,
            exposed_headers: self.expose_headers,
            max_age: self.max_age,
        }
    }
}

// TODO
pub struct LifecycleConfigurationBuilder {}

pub struct PublicAccessBlockConfigurationBuilder {
    block_public_acls: Option<bool>,
    block_public_policy: Option<bool>,
    ignore_public_acls: Option<bool>,
    restrict_public_buckets: Option<bool>,
}

impl PublicAccessBlockConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            block_public_acls: None,
            block_public_policy: None,
            ignore_public_acls: None,
            restrict_public_buckets: None,
        }
    }

    pub fn block_public_acls(self, config: bool) -> Self {
        Self {
            block_public_acls: Some(config),
            ..self
        }
    }

    pub fn block_public_policy(self, config: bool) -> Self {
        Self {
            block_public_policy: Some(config),
            ..self
        }
    }

    pub fn ignore_public_acls(self, config: bool) -> Self {
        Self {
            ignore_public_acls: Some(config),
            ..self
        }
    }

    pub fn restrict_public_buckets(self, config: bool) -> Self {
        Self {
            restrict_public_buckets: Some(config),
            ..self
        }
    }

    pub fn build(self) -> PublicAccessBlockConfiguration {
        PublicAccessBlockConfiguration {
            block_public_acls: self.ignore_public_acls,
            block_public_policy: self.block_public_policy,
            ignore_public_acls: self.ignore_public_acls,
            restrict_public_buckets: self.restrict_public_buckets,
        }
    }
}


use crate::iam::{Effect, PolicyDocument, PolicyDocumentBuilder, ServicePrincipal, PrincipalWrapper, StatementBuilder};
use crate::s3::dto;
use crate::s3::dto::{
    BucketEncryption, CorsConfiguration, CorsRule, LifecycleConfiguration, LifecycleRule, LifecycleRuleTransition,
    NonCurrentVersionTransition, PublicAccessBlockConfiguration, RedirectAllRequestsTo, S3Bucket, S3BucketPolicy, S3BucketPolicyProperties,
    S3BucketProperties, ServerSideEncryptionByDefault, ServerSideEncryptionRule, WebsiteConfiguration,
};
use crate::shared::http::{HttpMethod, Protocol};
use crate::shared::Id;
use crate::stack::Resource;
use crate::wrappers::{BucketName, IamAction, S3LifecycleObjectSizes};
use serde_json::Value;
use std::marker::PhantomData;
use std::time::Duration;
use crate::intrinsic_functions::{join};

// TODO notifications will require custom work to avoid circular dependencies
//  CDK approach with custom resources is one way
//  other way would be for the deploy to do extra work... but then the cloudformation template can only work correctly with our deploy method

pub struct S3BucketPolicyBuilder {
    id: Id,
    bucket_name: Value,
    policy_document: PolicyDocument,
}

impl S3BucketPolicyBuilder {
    pub fn new(id: &str, bucket_name: Value, policy_document: PolicyDocument) -> Self {
        Self {
            id: Id(id.to_string()),
            bucket_name,
            policy_document,
        }
    }

    #[must_use]
    pub fn build(self) -> S3BucketPolicy {
        let resource_id = Resource::generate_id("S3BucketPolicy");
        
        S3BucketPolicy {
            id: self.id,
            resource_id,
            r#type: "AWS::S3::BucketPolicy".to_string(),
            properties: S3BucketPolicyProperties {
                bucket_name: self.bucket_name,
                policy_document: self.policy_document,
            },
        }
    }
}

pub enum VersioningConfiguration {
    Enabled,
    Suspended,
}

impl From<VersioningConfiguration> for String {
    fn from(value: VersioningConfiguration) -> Self {
        match value {
            VersioningConfiguration::Enabled => "Enabled".to_string(),
            VersioningConfiguration::Suspended => "Suspended".to_string(),
        }
    }
}

pub enum S3Encryption {
    S3Managed,
    KmsManaged,
    DsseManaged,
    // KMS, => add, this requires creating a kms key and passing it to the bucket
    // DSSE, => add, similar
}

impl From<S3Encryption> for String {
    fn from(value: S3Encryption) -> Self {
        match value {
            S3Encryption::S3Managed => "AES256".to_string(),
            S3Encryption::KmsManaged => "aws:kms".to_string(),
            S3Encryption::DsseManaged => "aws:kms:dsse".to_string(),
        }
    }
}

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
    bucket_encryption: Option<S3Encryption>,
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
            bucket_encryption: None,
        }
    }

    #[must_use]
    pub fn build(self) -> S3Bucket {
        let (bucket, _) = self.build_internal(false);
        bucket
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

    pub fn encryption(self, encryption: S3Encryption) -> Self {
        Self {
            bucket_encryption: Some(encryption),
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
            bucket_encryption: self.bucket_encryption,
        }
    }

    fn build_internal(self, website: bool) -> (S3Bucket, Option<S3BucketPolicy>) {
        let resource_id = Resource::generate_id("S3Bucket");

        let versioning_configuration = self
            .versioning_configuration
            .map(|c| dto::VersioningConfiguration { status: c.into() });

        let website_configuration = if website {
            let redirect_all_requests_to = self.redirect_all_requests_to.map(|r| RedirectAllRequestsTo {
                host_name: r.0,
                protocol: r.1.map(Into::into),
            });

            Some(WebsiteConfiguration {
                index_document: self.index_document,
                error_document: self.error_document,
                redirect_all_requests_to,
            })
        } else {
            None
        };

        let access = if self.access.is_none() && website {
            // turning this off is required for an S3 website
            Some(PublicAccessBlockConfiguration {
                block_public_acls: Some(false),
                block_public_policy: Some(false),
                ignore_public_acls: Some(false),
                restrict_public_buckets: Some(false),
            })
        } else {
            self.access
        };

        let encryption = self.bucket_encryption.map(|v| {
            let rule = ServerSideEncryptionRule {
                server_side_encryption_by_default: ServerSideEncryptionByDefault {
                    sse_algorithm: v.into(),
                    kms_master_key_id: None,
                },
                bucket_key_enabled: None,
            };

            BucketEncryption {
                server_side_encryption_configuration: vec![rule],
            }
        });

        let properties = S3BucketProperties {
            bucket_name: self.name,
            cors_configuration: self.cors_config,
            lifecycle_configuration: self.lifecycle_configuration,
            public_access_block_configuration: access,
            versioning_configuration,
            website_configuration,
            bucket_encryption: encryption,
            notification_configuration: None,
        };

        let bucket = S3Bucket {
            id: self.id.clone(),
            resource_id,
            r#type: "AWS::S3::Bucket".to_string(),
            properties,
        };
        
        let policy = if website {
            // website needs a policy to allow GETs
            let bucket_resource = vec![join("", vec![bucket.get_arn(), Value::String("/*".to_string())])];
            let statement = StatementBuilder::new(vec![IamAction("s3:GetObject".to_string())], Effect::Allow)
                .resources(bucket_resource)
                .principal(PrincipalWrapper::StringPrincipal("*".to_string()))
                .build();
            let doc = PolicyDocumentBuilder::new(vec![statement]);
            let bucket_policy_id = format!("{}-website-s3-policy", self.id);
            let s3_policy = S3BucketPolicyBuilder::new(bucket_policy_id.as_str(), bucket.get_ref(), doc).build();
            Some(s3_policy)
        } else {
            None
        };

        (bucket, policy)
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

    #[must_use]
    pub fn build(self) -> (S3Bucket, S3BucketPolicy) {
        let (bucket, policy) = self.build_internal(true);
        (bucket, policy.expect("for website, bucket policy should always be present"))
    }
}

pub struct CorsConfigurationBuilder {
    rules: Vec<CorsRule>,
}

impl CorsConfigurationBuilder {
    pub fn new(cors_rules: Vec<CorsRule>) -> CorsConfiguration {
        CorsConfiguration { cors_rules }
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

    #[must_use]
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

pub enum TransitionDefaultMinimumObjectSize {
    VariesByStorageClass,
    AllStorageClasses128k,
}

impl From<TransitionDefaultMinimumObjectSize> for String {
    fn from(value: TransitionDefaultMinimumObjectSize) -> Self {
        match value {
            TransitionDefaultMinimumObjectSize::VariesByStorageClass => "varies_by_storage_class".to_string(),
            TransitionDefaultMinimumObjectSize::AllStorageClasses128k => "all_storage_classes_128K".to_string(),
        }
    }
}

pub enum LifecycleStorageClass {
    IntelligentTiering,
    OneZoneIA,
    StandardIA,
    GlacierDeepArchive,
    Glacier,
    GlacierInstantRetrieval,
}

impl From<LifecycleStorageClass> for String {
    fn from(value: LifecycleStorageClass) -> Self {
        match value {
            LifecycleStorageClass::GlacierDeepArchive => "DEEP_ARCHIVE".to_string(),
            LifecycleStorageClass::Glacier => "GLACIER".to_string(),
            LifecycleStorageClass::GlacierInstantRetrieval => "GLACIER_IR".to_string(),
            LifecycleStorageClass::IntelligentTiering => "INTELLIGENT_TIERING".to_string(),
            LifecycleStorageClass::OneZoneIA => "ONEZONE_IA".to_string(),
            LifecycleStorageClass::StandardIA => "STANDARD_IA".to_string(),
        }
    }
}

pub struct LifecycleRuleTransitionBuilder {
    storage_class: LifecycleStorageClass,
    transition_in_days: Option<u32>, // TODO should validate that it's >30 for standard and onezone => macro that combines both...
}

impl LifecycleRuleTransitionBuilder {
    pub fn new(storage_class: LifecycleStorageClass) -> Self {
        Self {
            storage_class,
            transition_in_days: None,
        }
    }

    pub fn transition_in_days(self, days: u32) -> Self {
        Self {
            transition_in_days: Some(days),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> LifecycleRuleTransition {
        LifecycleRuleTransition {
            storage_class: self.storage_class.into(),
            transition_in_days: self.transition_in_days.unwrap_or(0),
        }
    }
}

pub struct NonCurrentVersionTransitionBuilder {
    storage_class: LifecycleStorageClass,
    transition_in_days: u32,
    newer_non_current_versions: Option<u32>,
}

impl NonCurrentVersionTransitionBuilder {
    pub fn new(storage_class: LifecycleStorageClass, transition_in_days: u32) -> Self {
        Self {
            storage_class,
            transition_in_days,
            newer_non_current_versions: None,
        }
    }

    pub fn newer_non_current_versions(self, versions: u32) -> Self {
        Self {
            newer_non_current_versions: Some(versions),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> NonCurrentVersionTransition {
        NonCurrentVersionTransition {
            storage_class: self.storage_class.into(),
            transition_in_days: self.transition_in_days,
            newer_non_current_versions: self.newer_non_current_versions,
        }
    }
}

pub enum LifecycleRuleStatus {
    Enabled,
    Disabled,
}

impl From<LifecycleRuleStatus> for String {
    fn from(value: LifecycleRuleStatus) -> Self {
        match value {
            LifecycleRuleStatus::Enabled => "Enabled".to_string(),
            LifecycleRuleStatus::Disabled => "Disabled".to_string(),
        }
    }
}

pub struct LifecycleRuleBuilder {
    id: Option<String>,
    status: LifecycleRuleStatus,
    expiration_in_days: Option<u16>, // TODO expiration must be > than expiration in transition (ow boy...)
    prefix: Option<String>,
    object_size_greater_than: Option<u32>,
    object_size_less_than: Option<u32>,
    abort_incomplete_multipart_upload: Option<u16>,
    non_current_version_expiration: Option<u16>,
    transitions: Option<Vec<LifecycleRuleTransition>>,
    non_current_version_transitions: Option<Vec<NonCurrentVersionTransition>>,
}

// TODO
impl LifecycleRuleBuilder {
    pub fn new(status: LifecycleRuleStatus) -> Self {
        Self {
            status,
            id: None,
            expiration_in_days: None,
            prefix: None,
            object_size_greater_than: None,
            object_size_less_than: None,
            abort_incomplete_multipart_upload: None,
            non_current_version_expiration: None,
            transitions: None,
            non_current_version_transitions: None,
        }
    }

    pub fn id(self, id: String) -> Self {
        Self { id: Some(id), ..self }
    }

    pub fn expiration_in_days(self, days: u16) -> Self {
        Self {
            expiration_in_days: Some(days),
            ..self
        }
    }

    pub fn prefix(self, prefix: String) -> Self {
        Self {
            prefix: Some(prefix),
            ..self
        }
    }

    pub fn object_size(self, sizes: S3LifecycleObjectSizes) -> Self {
        Self {
            object_size_less_than: sizes.0,
            object_size_greater_than: sizes.1,
            ..self
        }
    }

    pub fn abort_incomplete_multipart_upload(self, days: u16) -> Self {
        Self {
            abort_incomplete_multipart_upload: Some(days),
            ..self
        }
    }

    pub fn non_current_version_expiration(self, days: u16) -> Self {
        Self {
            non_current_version_expiration: Some(days),
            ..self
        }
    }

    pub fn add_transition(mut self, transition: LifecycleRuleTransition) -> Self {
        if let Some(mut transitions) = self.transitions {
            transitions.push(transition);
            self.transitions = Some(transitions);
        } else {
            self.transitions = Some(vec![transition]);
        }

        Self { ..self }
    }

    pub fn add_non_current_version_transitions(mut self, transition: NonCurrentVersionTransition) -> Self {
        if let Some(mut transitions) = self.non_current_version_transitions {
            transitions.push(transition);
            self.non_current_version_transitions = Some(transitions);
        } else {
            self.non_current_version_transitions = Some(vec![transition]);
        }

        Self { ..self }
    }

    pub fn build(self) -> LifecycleRule {
        LifecycleRule {
            id: self.id,
            status: self.status.into(),
            expiration_in_days: self.expiration_in_days,
            prefix: self.prefix,
            object_size_greater_than: self.object_size_greater_than,
            object_size_less_than: self.object_size_less_than,
            transitions: self.transitions,
            abort_incomplete_multipart_upload: self.abort_incomplete_multipart_upload,
            non_current_version_expiration: self.non_current_version_expiration,
            non_current_version_transitions: self.non_current_version_transitions,
        }
    }
}

pub struct LifecycleConfigurationBuilder {
    rules: Vec<LifecycleRule>,
    transition_minimum_size: Option<TransitionDefaultMinimumObjectSize>,
}

impl LifecycleConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            rules: vec![],
            transition_minimum_size: None,
        }
    }

    pub fn transition_minimum_size(self, size: TransitionDefaultMinimumObjectSize) -> Self {
        Self {
            transition_minimum_size: Some(size),
            ..self
        }
    }

    pub fn add_rule(mut self, rule: LifecycleRule) -> Self {
        self.rules.push(rule);
        self
    }

    #[must_use]
    pub fn build(self) -> LifecycleConfiguration {
        LifecycleConfiguration {
            rules: self.rules,
            transition_minimum_size: self.transition_minimum_size.map(|v| v.into()),
        }
    }
}

pub struct PublicAccessBlockConfigurationBuilder {
    block_public_acls: Option<bool>,
    block_public_policy: Option<bool>,
    ignore_public_acls: Option<bool>,
    restrict_public_buckets: Option<bool>,
}

impl Default for PublicAccessBlockConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
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

    #[must_use]
    pub fn build(self) -> PublicAccessBlockConfiguration {
        PublicAccessBlockConfiguration {
            block_public_acls: self.ignore_public_acls,
            block_public_policy: self.block_public_policy,
            ignore_public_acls: self.ignore_public_acls,
            restrict_public_buckets: self.restrict_public_buckets,
        }
    }
}

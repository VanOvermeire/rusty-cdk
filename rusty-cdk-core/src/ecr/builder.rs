use std::marker::PhantomData;

use crate::ecr::{
    EncryptionConfiguration, ImageScanningConfiguration, ImageTagMutabilityExclusionFilter, LifecyclePolicy, PublicRepository,
    PublicRepositoryProperties, PublicRepositoryRef, PublicRepositoryType, PullThroughCacheRule, PullThroughCacheRuleProperties,
    PullThroughCacheRuleRef, PullThroughCacheRuleType, PullTimeUpdateExclusion, PullTimeUpdateExclusionProperties,
    PullTimeUpdateExclusionRef, PullTimeUpdateExclusionType, RegistryPolicy, RegistryPolicyProperties, RegistryPolicyRef,
    RegistryPolicyType, RegistryScanningConfiguration, RegistryScanningConfigurationProperties, RegistryScanningConfigurationRef,
    RegistryScanningConfigurationType, ReplicationConfiguration, ReplicationConfigurationProperties, ReplicationConfigurationRef,
    ReplicationConfigurationReplicationConfiguration, ReplicationConfigurationType, ReplicationDestination, ReplicationRule, Repository,
    RepositoryCatalogData, RepositoryFilter, RepositoryProperties, RepositoryRef, RepositoryType, Rule, ScanningRule, SigningConfiguration,
    SigningConfigurationProperties, SigningConfigurationRef, SigningConfigurationType,
};
use crate::iam::RoleRef;
use crate::iam::Statement;
use crate::kms::KeyRef;
use crate::secretsmanager::SecretRef;
use crate::shared::Id;
use crate::shared::Region;
use crate::stack::{Resource, StackBuilder};
use crate::type_state;
use crate::wrappers::EcrRepositoryName;
use crate::wrappers::ImageTagMutabilityExclusionFilterValue;
use crate::wrappers::RepoAboutText;
use crate::wrappers::RepoDescription;
use crate::wrappers::RepoPrefix;
use crate::wrappers::URL;
use serde_json::Value;
use serde_json::json;

pub struct PublicRepositoryBuilder {
    id: Id,
    repository_catalog_data: Option<RepositoryCatalogData>,
    repository_policy_text: Option<Value>,
    repository_name: Option<String>,
}

impl PublicRepositoryBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            repository_catalog_data: None,
            repository_policy_text: None,
            repository_name: None,
        }
    }

    pub fn repository_catalog_data(self, repository_catalog_data: RepositoryCatalogData) -> Self {
        Self {
            repository_catalog_data: Some(repository_catalog_data),
            ..self
        }
    }

    pub fn repository_policy_text(self, repository_policy_text: Value) -> Self {
        Self {
            repository_policy_text: Some(repository_policy_text),
            ..self
        }
    }

    pub fn repository_name(self, repository_name: EcrRepositoryName) -> Self {
        Self {
            repository_name: Some(repository_name.0),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> PublicRepositoryRef {
        let resource_id = Resource::generate_id("PublicRepository");

        let resource = PublicRepository {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: PublicRepositoryType::PublicRepositoryType,
            properties: PublicRepositoryProperties {
                repository_catalog_data: self.repository_catalog_data,
                repository_policy_text: self.repository_policy_text,
                repository_name: self.repository_name,
            },
        };
        stack_builder.add_resource(resource);

        PublicRepositoryRef::internal_new(resource_id)
    }
}

pub struct RepositoryCatalogDataBuilder {
    about_text: Option<String>,
    architectures: Option<Vec<String>>,
    operating_systems: Option<Vec<String>>,
    repository_description: Option<String>,
    usage_text: Option<String>,
}

impl RepositoryCatalogDataBuilder {
    pub fn new() -> Self {
        Self {
            about_text: None,
            architectures: None,
            operating_systems: None,
            repository_description: None,
            usage_text: None,
        }
    }

    pub fn about_text(self, about_text: RepoAboutText) -> Self {
        Self {
            about_text: Some(about_text.0),
            ..self
        }
    }

    // should enforce max of 50
    pub fn architectures(self, architectures: Vec<String>) -> Self {
        Self {
            architectures: Some(architectures),
            ..self
        }
    }

    // should enforce max of 50
    pub fn operating_systems(self, operating_systems: Vec<String>) -> Self {
        Self {
            operating_systems: Some(operating_systems),
            ..self
        }
    }

    pub fn repository_description(self, repository_description: RepoDescription) -> Self {
        Self {
            repository_description: Some(repository_description.0),
            ..self
        }
    }

    pub fn usage_text(self, usage_text: RepoAboutText) -> Self {
        Self {
            usage_text: Some(usage_text.0),
            ..self
        }
    }

    pub fn build(self) -> RepositoryCatalogData {
        RepositoryCatalogData {
            about_text: self.about_text,
            architectures: self.architectures,
            operating_systems: self.operating_systems,
            repository_description: self.repository_description,
            usage_text: self.usage_text,
        }
    }
}

pub struct PullThroughCacheRuleBuilder {
    id: Id,
    upstream_repository_prefix: Option<String>,
    credential_arn: Option<Value>,
    upstream_registry_url: Option<String>,
    custom_role_arn: Option<Value>,
    ecr_repository_prefix: Option<String>,
    upstream_registry: Option<String>,
}

pub enum UpstreamRegistry {
    Ecr,
    EcrPublic,
    Quay,
    K8s,
    DockerHub,
    GithubContainerRegistry,
    AzureContainerRegistry,
    GitlabContainerRegistry,
}

impl From<UpstreamRegistry> for String {
    fn from(registry: UpstreamRegistry) -> Self {
        match registry {
            UpstreamRegistry::Ecr => "ecr",
            UpstreamRegistry::EcrPublic => "ecr-public",
            UpstreamRegistry::Quay => "quay",
            UpstreamRegistry::K8s => "k8s",
            UpstreamRegistry::DockerHub => "docker-hub",
            UpstreamRegistry::GithubContainerRegistry => "github-container-registry",
            UpstreamRegistry::AzureContainerRegistry => "azure-container-registry",
            UpstreamRegistry::GitlabContainerRegistry => "gitlab-container-registry",
        }
        .to_string()
    }
}

impl PullThroughCacheRuleBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            upstream_repository_prefix: None,
            credential_arn: None,
            upstream_registry_url: None,
            custom_role_arn: None,
            ecr_repository_prefix: None,
            upstream_registry: None,
        }
    }

    pub fn upstream_repository_prefix(self, upstream_repository_prefix: RepoPrefix) -> Self {
        Self {
            upstream_repository_prefix: Some(upstream_repository_prefix.0),
            ..self
        }
    }

    pub fn credential_arn(self, credentials: &SecretRef) -> Self {
        Self {
            credential_arn: Some(credentials.get_arn()),
            ..self
        }
    }

    pub fn upstream_registry_url(self, upstream_registry_url: URL) -> Self {
        Self {
            // registry url does not need a prefix
            upstream_registry_url: Some(upstream_registry_url.0.replace("https://", "").replace("http://", "")),
            ..self
        }
    }

    pub fn custom_role_arn(self, custom_role_arn: &RoleRef) -> Self {
        Self {
            custom_role_arn: Some(custom_role_arn.get_arn()),
            ..self
        }
    }

    pub fn ecr_repository_prefix(self, ecr_repository_prefix: RepoPrefix) -> Self {
        Self {
            ecr_repository_prefix: Some(ecr_repository_prefix.0),
            ..self
        }
    }

    pub fn upstream_registry(self, upstream_registry: UpstreamRegistry) -> Self {
        Self {
            upstream_registry: Some(upstream_registry.into()),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> PullThroughCacheRuleRef {
        let resource_id = Resource::generate_id("PullThroughCacheRule");

        let resource = PullThroughCacheRule {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: PullThroughCacheRuleType::PullThroughCacheRuleType,
            properties: PullThroughCacheRuleProperties {
                upstream_repository_prefix: self.upstream_repository_prefix,
                credential_arn: self.credential_arn,
                upstream_registry_url: self.upstream_registry_url,
                custom_role_arn: self.custom_role_arn,
                ecr_repository_prefix: self.ecr_repository_prefix,
                upstream_registry: self.upstream_registry,
            },
        };
        stack_builder.add_resource(resource);

        PullThroughCacheRuleRef::internal_new(resource_id)
    }
}

// TODO should also accept a user
pub enum PullTimeUpdateExclusionPrincipals<'a> {
    Role(&'a RoleRef),
}

pub struct PullTimeUpdateExclusionBuilder {
    id: Id,
    principal_arn: Value,
}

impl PullTimeUpdateExclusionBuilder {
    pub fn new(id: &str, principal: PullTimeUpdateExclusionPrincipals) -> Self {
        let arn = match principal {
            PullTimeUpdateExclusionPrincipals::Role(role) => role.get_arn(),
        };

        Self {
            id: Id(id.to_string()),
            principal_arn: arn,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> PullTimeUpdateExclusionRef {
        let resource_id = Resource::generate_id("PullTimeUpdateExclusion");

        let resource = PullTimeUpdateExclusion {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: PullTimeUpdateExclusionType::PullTimeUpdateExclusionType,
            properties: PullTimeUpdateExclusionProperties {
                principal_arn: self.principal_arn,
            },
        };
        stack_builder.add_resource(resource);

        PullTimeUpdateExclusionRef::internal_new(resource_id)
    }
}

pub struct RegistryPolicyBuilder {
    id: Id,
    statements: Vec<Statement>,
}

impl RegistryPolicyBuilder {
    pub fn new(id: &str, statements: Vec<Statement>) -> Self {
        Self {
            id: Id(id.to_string()),
            statements,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> RegistryPolicyRef {
        let resource_id = Resource::generate_id("RegistryPolicy");
        let policy_text = json!({
            "Version":"2012-10-17",
            "Statement": self.statements,
        });

        let resource = RegistryPolicy {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: RegistryPolicyType::RegistryPolicyType,
            properties: RegistryPolicyProperties { policy_text },
        };
        stack_builder.add_resource(resource);

        RegistryPolicyRef::internal_new(resource_id)
    }
}

pub enum BasicScanFrequency {
    Manual,
    OnPush,
}

impl From<BasicScanFrequency> for String {
    fn from(value: BasicScanFrequency) -> Self {
        match value {
            BasicScanFrequency::Manual => "MANUAL".to_string(),
            BasicScanFrequency::OnPush => "SCAN_ON_PUSH".to_string(),
        }
    }
}

pub enum EnhancedScanFrequency {
    OnPush,
    Continuous,
}

impl From<EnhancedScanFrequency> for String {
    fn from(value: EnhancedScanFrequency) -> Self {
        match value {
            EnhancedScanFrequency::OnPush => "SCAN_ON_PUSH".to_string(),
            EnhancedScanFrequency::Continuous => "CONTINUOUS_SCAN".to_string(),
        }
    }
}

type_state!(
    RegistryScanningConfigurationState,
    RegistryScanningConfigurationStartState,
    RegistryScanningConfigurationBasicState,
    RegistryScanningConfigurationEnhancedState,
);

pub struct RegistryScanningConfigurationBuilder<T: RegistryScanningConfigurationState> {
    phantom_data: PhantomData<T>,
    id: Id,
    scan_type: String,
    rules: Vec<ScanningRule>,
}

// enforce max of two scanning rules
impl RegistryScanningConfigurationBuilder<RegistryScanningConfigurationStartState> {
    pub fn new_basic_scan_type(id: &str) -> RegistryScanningConfigurationBuilder<RegistryScanningConfigurationBasicState> {
        RegistryScanningConfigurationBuilder {
            phantom_data: Default::default(),
            id: Id(id.to_string()),
            scan_type: "BASIC".to_string(),
            rules: vec![],
        }
    }

    pub fn new_enhanced_scan_type(id: &str) -> RegistryScanningConfigurationBuilder<RegistryScanningConfigurationEnhancedState> {
        RegistryScanningConfigurationBuilder {
            phantom_data: Default::default(),
            id: Id(id.to_string()),
            scan_type: "ENHANCED".to_string(),
            rules: vec![],
        }
    }
}

impl RegistryScanningConfigurationBuilder<RegistryScanningConfigurationBasicState> {
    pub fn scanning_rule(mut self, scan_frequency: BasicScanFrequency, repository_filters: Vec<RepositoryFilter>) -> Self {
        let scanning_rule = ScanningRuleBuilder::new(scan_frequency.into(), repository_filters).build();
        self.rules.push(scanning_rule);
        self
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> RegistryScanningConfigurationRef {
        self.build_internal(stack_builder)
    }
}

impl RegistryScanningConfigurationBuilder<RegistryScanningConfigurationEnhancedState> {
    pub fn scanning_rule(mut self, scan_frequency: EnhancedScanFrequency, repository_filters: Vec<RepositoryFilter>) -> Self {
        let scanning_rule = ScanningRuleBuilder::new(scan_frequency.into(), repository_filters).build();
        self.rules.push(scanning_rule);
        self
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> RegistryScanningConfigurationRef {
        self.build_internal(stack_builder)
    }
}

impl<T: RegistryScanningConfigurationState> RegistryScanningConfigurationBuilder<T> {
    pub fn build_internal(self, stack_builder: &mut StackBuilder) -> RegistryScanningConfigurationRef {
        let resource_id = Resource::generate_id("RegistryScanningConfiguration");

        let resource = RegistryScanningConfiguration {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: RegistryScanningConfigurationType::RegistryScanningConfigurationType,
            properties: RegistryScanningConfigurationProperties {
                scan_type: self.scan_type,
                rules: self.rules,
            },
        };
        stack_builder.add_resource(resource);

        RegistryScanningConfigurationRef::internal_new(resource_id)
    }
}

/// Use the RegistryScanningConfigurationBuilder `scanning_rule` to set scanning rules
pub(crate) struct ScanningRuleBuilder {
    scan_frequency: String,
    repository_filters: Vec<RepositoryFilter>,
}

impl ScanningRuleBuilder {
    // enforce max of 100 rules
    pub(crate) fn new(scan_frequency: String, repository_filters: Vec<RepositoryFilter>) -> Self {
        Self {
            scan_frequency,
            repository_filters,
        }
    }

    pub(crate) fn build(self) -> ScanningRule {
        ScanningRule {
            scan_frequency: self.scan_frequency,
            repository_filters: self.repository_filters,
        }
    }
}

pub struct ReplicationConfigurationBuilder {
    id: Id,
    rules: Vec<ReplicationRule>,
}

impl ReplicationConfigurationBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            rules: vec![],
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> ReplicationConfigurationRef {
        let resource_id = Resource::generate_id("ReplicationConfiguration");

        let resource = ReplicationConfiguration {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: ReplicationConfigurationType::ReplicationConfigurationType,
            properties: ReplicationConfigurationProperties {
                replication_configuration: ReplicationConfigurationReplicationConfiguration { rules: self.rules },
            },
        };
        stack_builder.add_resource(resource);

        ReplicationConfigurationRef::internal_new(resource_id)
    }
}

// enforce max 100 for both
pub struct ReplicationRuleBuilder {
    destinations: Vec<ReplicationDestination>,
    repository_filters: Option<Vec<RepositoryFilter>>,
}

impl ReplicationRuleBuilder {
    pub fn new(destination: ReplicationDestination) -> Self {
        Self {
            destinations: vec![destination],
            repository_filters: None,
        }
    }

    pub fn add_destination(mut self, destination: ReplicationDestination) -> Self {
        self.destinations.push(destination);
        self
    }

    pub fn add_repository_filter(mut self, repository_filter: RepositoryFilter) -> Self {
        let mut filters = self.repository_filters.unwrap_or_default();
        filters.push(repository_filter);
        self.repository_filters = Some(filters);
        self
    }

    pub fn build(self) -> ReplicationRule {
        ReplicationRule {
            destinations: self.destinations,
            repository_filters: self.repository_filters,
        }
    }
}

pub struct ReplicationDestinationBuilder {
    region: String,
    // TODO account id lookup
    registry_id: String,
}

impl ReplicationDestinationBuilder {
    pub fn new(region: String, registry_id: Region) -> Self {
        Self {
            region,
            registry_id: registry_id.into(),
        }
    }

    pub fn build(self) -> ReplicationDestination {
        ReplicationDestination {
            region: self.region,
            registry_id: self.registry_id,
        }
    }
}

pub enum ImageTagMutability {
    Mutable,
    Immutable,
    MutableWithExclusion,
    ImmutableWithExclusion,
}

impl From<ImageTagMutability> for String {
    fn from(mutability: ImageTagMutability) -> Self {
        match mutability {
            ImageTagMutability::Mutable => "MUTABLE".to_string(),
            ImageTagMutability::Immutable => "IMMUTABLE".to_string(),
            ImageTagMutability::MutableWithExclusion => "MUTABLE_WITH_EXCLUSION".to_string(),
            ImageTagMutability::ImmutableWithExclusion => "IMMUTABLE_WITH_EXCLUSION".to_string(),
        }
    }
}

pub struct RepositoryBuilder {
    id: Id,
    image_tag_mutability: Option<String>,
    repository_policy_text: Option<Value>,
    image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    encryption_configuration: Option<EncryptionConfiguration>,
    lifecycle_policy: Option<LifecyclePolicy>,
    empty_on_delete: Option<bool>,
    image_scanning_configuration: Option<bool>,
    repository_name: Option<String>,
}

impl RepositoryBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            image_tag_mutability: None,
            repository_policy_text: None,
            image_tag_mutability_exclusion_filters: None,
            encryption_configuration: None,
            lifecycle_policy: None,
            empty_on_delete: None,
            image_scanning_configuration: None,
            repository_name: None,
        }
    }

    pub fn image_tag_mutability(self, image_tag_mutability: ImageTagMutability) -> Self {
        Self {
            image_tag_mutability: Some(image_tag_mutability.into()),
            ..self
        }
    }

    pub fn repository_policy_text(self, repository_policy_text: Value) -> Self {
        Self {
            repository_policy_text: Some(repository_policy_text),
            ..self
        }
    }

    // enforce max 5
    pub fn add_tag_mutability_exclusion_filter(mut self, tag_mutability_exclusion_filter: ImageTagMutabilityExclusionFilter) -> Self {
        let mut filters = self.image_tag_mutability_exclusion_filters.unwrap_or_default();
        filters.push(tag_mutability_exclusion_filter);
        self.image_tag_mutability_exclusion_filters = Some(filters);
        self
    }

    pub fn encryption_configuration(self, encryption_configuration: EncryptionConfiguration) -> Self {
        Self {
            encryption_configuration: Some(encryption_configuration),
            ..self
        }
    }

    pub fn lifecycle_policy(self, lifecycle_policy: LifecyclePolicy) -> Self {
        Self {
            lifecycle_policy: Some(lifecycle_policy),
            ..self
        }
    }

    pub fn empty_on_delete(self, empty_on_delete: bool) -> Self {
        Self {
            empty_on_delete: Some(empty_on_delete),
            ..self
        }
    }

    pub fn image_scanning_configuration(self, scan_on_push: bool) -> Self {
        Self {
            image_scanning_configuration: Some(scan_on_push),
            ..self
        }
    }

    pub fn repository_name(self, repository_name: EcrRepositoryName) -> Self {
        Self {
            repository_name: Some(repository_name.0),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> RepositoryRef {
        let resource_id = Resource::generate_id("Repository");

        let resource = Repository {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: RepositoryType::RepositoryType,
            properties: RepositoryProperties {
                image_tag_mutability: self.image_tag_mutability,
                repository_policy_text: self.repository_policy_text,
                image_tag_mutability_exclusion_filters: self.image_tag_mutability_exclusion_filters,
                encryption_configuration: self.encryption_configuration,
                lifecycle_policy: self.lifecycle_policy,
                empty_on_delete: self.empty_on_delete,
                image_scanning_configuration: self
                    .image_scanning_configuration
                    .map(|scan_on_push| ImageScanningConfigurationBuilder::new().scan_on_push(scan_on_push).build()),
                repository_name: self.repository_name,
            },
        };
        stack_builder.add_resource(resource);

        RepositoryRef::internal_new(resource_id)
    }
}

pub enum EncryptionConfigurationType<'a> {
    Aes256,
    Kms(&'a KeyRef),
    KmsDsse(&'a KeyRef),
}

pub struct EncryptionConfigurationBuilder {
    encryption_type: String,
    kms_key: Option<Value>,
}

impl EncryptionConfigurationBuilder {
    pub fn new(encryption: EncryptionConfigurationType) -> Self {
        match encryption {
            EncryptionConfigurationType::Aes256 => Self {
                encryption_type: "AES256".to_string(),
                kms_key: None,
            },
            EncryptionConfigurationType::Kms(key) => Self {
                encryption_type: "KMS".to_string(),
                kms_key: Some(key.get_arn()),
            },
            EncryptionConfigurationType::KmsDsse(key) => Self {
                encryption_type: "KMS_DSSE".to_string(),
                kms_key: Some(key.get_arn()),
            },
        }
    }

    pub fn build(self) -> EncryptionConfiguration {
        EncryptionConfiguration {
            encryption_type: self.encryption_type,
            kms_key: self.kms_key,
        }
    }
}

pub struct LifecyclePolicyBuilder {
    registry_id: Option<String>,
    lifecycle_policy_text: Option<String>,
}

impl LifecyclePolicyBuilder {
    pub fn new() -> Self {
        Self {
            registry_id: None,
            lifecycle_policy_text: None,
        }
    }

    // TODO == account id, need lookup
    pub fn registry_id(self, registry_id: String) -> Self {
        Self {
            registry_id: Some(registry_id),
            ..self
        }
    }

    pub fn lifecycle_policy_text(self, lifecycle_policy_text: String) -> Self {
        Self {
            lifecycle_policy_text: Some(lifecycle_policy_text),
            ..self
        }
    }

    pub fn build(self) -> LifecyclePolicy {
        LifecyclePolicy {
            registry_id: self.registry_id,
            lifecycle_policy_text: self.lifecycle_policy_text,
        }
    }
}

pub(crate) struct ImageScanningConfigurationBuilder {
    scan_on_push: Option<bool>,
}

impl ImageScanningConfigurationBuilder {
    pub(crate) fn new() -> Self {
        Self { scan_on_push: None }
    }

    pub(crate) fn scan_on_push(self, scan_on_push: bool) -> Self {
        Self {
            scan_on_push: Some(scan_on_push),
            ..self
        }
    }

    pub(crate) fn build(self) -> ImageScanningConfiguration {
        ImageScanningConfiguration {
            scan_on_push: self.scan_on_push,
        }
    }
}

// enforce max 50 rules
pub struct SigningConfigurationBuilder {
    id: Id,
    rules: Vec<Rule>,
}

impl SigningConfigurationBuilder {
    pub fn new(id: &str, rules: Vec<Rule>) -> Self {
        Self {
            id: Id(id.to_string()),
            rules,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> SigningConfigurationRef {
        let resource_id = Resource::generate_id("SigningConfiguration");

        let resource = SigningConfiguration {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: SigningConfigurationType::SigningConfigurationType,
            properties: SigningConfigurationProperties { rules: self.rules },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        SigningConfigurationRef::internal_new(resource_id)
    }
}

pub enum ImageTagMutabilityExclusionFilterType {
    Wildcard,
}

impl From<ImageTagMutabilityExclusionFilterType> for String {
    fn from(value: ImageTagMutabilityExclusionFilterType) -> Self {
        match value {
            ImageTagMutabilityExclusionFilterType::Wildcard => "WILDCARD".to_string(),
        }
    }
}

pub struct ImageTagMutabilityExclusionFilterBuilder {
    image_tag_mutability_exclusion_filter_value: String,
    image_tag_mutability_exclusion_filter_type: String,
}

impl ImageTagMutabilityExclusionFilterBuilder {
    pub fn new(
        image_tag_mutability_exclusion_filter_type: ImageTagMutabilityExclusionFilterType,
        image_tag_mutability_exclusion_filter_value: ImageTagMutabilityExclusionFilterValue,
    ) -> Self {
        Self {
            image_tag_mutability_exclusion_filter_value: image_tag_mutability_exclusion_filter_value.0,
            image_tag_mutability_exclusion_filter_type: image_tag_mutability_exclusion_filter_type.into(),
        }
    }

    pub fn build(self) -> ImageTagMutabilityExclusionFilter {
        ImageTagMutabilityExclusionFilter {
            image_tag_mutability_exclusion_filter_value: self.image_tag_mutability_exclusion_filter_value,
            image_tag_mutability_exclusion_filter_type: self.image_tag_mutability_exclusion_filter_type,
        }
    }
}

pub struct RuleBuilder {
    signing_profile_arn: String,
    repository_filters: Option<Vec<RepositoryFilter>>,
}

impl RuleBuilder {
    // TODO signer lookup
    pub fn new(signing_profile_arn: String) -> Self {
        Self {
            signing_profile_arn,
            repository_filters: None,
        }
    }

    // Enforce max 100
    pub fn add_repository_filter(mut self, repository_filter: RepositoryFilter) -> Self {
        let mut filters = self.repository_filters.unwrap_or_default();
        filters.push(repository_filter);
        self.repository_filters = Some(filters);
        self
    }

    pub fn build(self) -> Rule {
        Rule {
            signing_profile_arn: self.signing_profile_arn,
            repository_filters: self.repository_filters,
        }
    }
}

pub enum RepositoryFilterType {
    PrefixMatch,
}

impl From<RepositoryFilterType> for String {
    fn from(filter_type: RepositoryFilterType) -> Self {
        match filter_type {
            RepositoryFilterType::PrefixMatch => "PREFIX_MATCH".to_string(),
        }
    }
}

pub struct RepositoryFilterBuilder {
    filter: String,
    filter_type: String,
}

impl RepositoryFilterBuilder {
    pub fn new(filter_type: RepositoryFilterType, filter: String) -> Self {
        Self {
            filter,
            filter_type: filter_type.into(),
        }
    }

    pub fn build(self) -> RepositoryFilter {
        RepositoryFilter {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

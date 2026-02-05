use crate::shared::Id;
use crate::{dto_methods, ref_struct};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PublicRepositoryType {
    #[serde(rename = "AWS::ECR::PublicRepository")]
    PublicRepositoryType,
}

ref_struct!(PublicRepositoryRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRepository {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: PublicRepositoryType,
    #[serde(rename = "Properties")]
    pub(crate) properties: PublicRepositoryProperties,
}
dto_methods!(PublicRepository);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRepositoryProperties {
    #[serde(rename = "RepositoryCatalogData", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_catalog_data: Option<RepositoryCatalogData>,
    #[serde(rename = "RepositoryPolicyText", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_policy_text: Option<Value>,
    #[serde(rename = "RepositoryName", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryCatalogData {
    #[serde(rename = "AboutText", skip_serializing_if = "Option::is_none")]
    pub(crate) about_text: Option<String>,
    #[serde(rename = "Architectures", skip_serializing_if = "Option::is_none")]
    pub(crate) architectures: Option<Vec<String>>,
    #[serde(rename = "OperatingSystems", skip_serializing_if = "Option::is_none")]
    pub(crate) operating_systems: Option<Vec<String>>,
    #[serde(rename = "RepositoryDescription", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_description: Option<String>,
    #[serde(rename = "UsageText", skip_serializing_if = "Option::is_none")]
    pub(crate) usage_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PullThroughCacheRuleType {
    #[serde(rename = "AWS::ECR::PullThroughCacheRule")]
    PullThroughCacheRuleType,
}

ref_struct!(PullThroughCacheRuleRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct PullThroughCacheRule {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: PullThroughCacheRuleType,
    #[serde(rename = "Properties")]
    pub(crate) properties: PullThroughCacheRuleProperties,
}
dto_methods!(PullThroughCacheRule);

#[derive(Debug, Serialize, Deserialize)]
pub struct PullThroughCacheRuleProperties {
    #[serde(rename = "UpstreamRepositoryPrefix", skip_serializing_if = "Option::is_none")]
    pub(crate) upstream_repository_prefix: Option<String>,
    #[serde(rename = "CredentialArn", skip_serializing_if = "Option::is_none")]
    pub(crate) credential_arn: Option<Value>,
    #[serde(rename = "UpstreamRegistryUrl", skip_serializing_if = "Option::is_none")]
    pub(crate) upstream_registry_url: Option<String>,
    #[serde(rename = "CustomRoleArn", skip_serializing_if = "Option::is_none")]
    pub(crate) custom_role_arn: Option<Value>,
    #[serde(rename = "EcrRepositoryPrefix", skip_serializing_if = "Option::is_none")]
    pub(crate) ecr_repository_prefix: Option<String>,
    #[serde(rename = "UpstreamRegistry", skip_serializing_if = "Option::is_none")]
    pub(crate) upstream_registry: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PullTimeUpdateExclusionType {
    #[serde(rename = "AWS::ECR::PullTimeUpdateExclusion")]
    PullTimeUpdateExclusionType,
}

ref_struct!(PullTimeUpdateExclusionRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct PullTimeUpdateExclusion {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: PullTimeUpdateExclusionType,
    #[serde(rename = "Properties")]
    pub(crate) properties: PullTimeUpdateExclusionProperties,
}
dto_methods!(PullTimeUpdateExclusion);

#[derive(Debug, Serialize, Deserialize)]
pub struct PullTimeUpdateExclusionProperties {
    #[serde(rename = "PrincipalArn")]
    pub(crate) principal_arn: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum RegistryPolicyType {
    #[serde(rename = "AWS::ECR::RegistryPolicy")]
    RegistryPolicyType,
}

ref_struct!(RegistryPolicyRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryPolicy {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: RegistryPolicyType,
    #[serde(rename = "Properties")]
    pub(crate) properties: RegistryPolicyProperties,
}
dto_methods!(RegistryPolicy);

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryPolicyProperties {
    #[serde(rename = "PolicyText")]
    pub(crate) policy_text: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum RegistryScanningConfigurationType {
    #[serde(rename = "AWS::ECR::RegistryScanningConfiguration")]
    RegistryScanningConfigurationType,
}

ref_struct!(RegistryScanningConfigurationRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryScanningConfiguration {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: RegistryScanningConfigurationType,
    #[serde(rename = "Properties")]
    pub(crate) properties: RegistryScanningConfigurationProperties,
}
dto_methods!(RegistryScanningConfiguration);

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryScanningConfigurationProperties {
    #[serde(rename = "ScanType")]
    pub(crate) scan_type: String,
    #[serde(rename = "Rules")]
    pub(crate) rules: Vec<ScanningRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanningRule {
    #[serde(rename = "ScanFrequency")]
    pub(crate) scan_frequency: String,
    #[serde(rename = "RepositoryFilters")]
    pub(crate) repository_filters: Vec<RepositoryFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ReplicationConfigurationType {
    #[serde(rename = "AWS::ECR::ReplicationConfiguration")]
    ReplicationConfigurationType,
}

ref_struct!(ReplicationConfigurationRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationConfiguration {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: ReplicationConfigurationType,
    #[serde(rename = "Properties")]
    pub(crate) properties: ReplicationConfigurationProperties,
}
dto_methods!(ReplicationConfiguration);

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationConfigurationProperties {
    #[serde(rename = "ReplicationConfiguration")]
    pub(crate) replication_configuration: ReplicationConfigurationReplicationConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationConfigurationReplicationConfiguration {
    #[serde(rename = "Rules")]
    pub(crate) rules: Vec<ReplicationRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationRule {
    #[serde(rename = "Destinations")]
    pub(crate) destinations: Vec<ReplicationDestination>, // An array of objects representing the destination for a replication rule., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">100</code>
    #[serde(rename = "RepositoryFilters", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_filters: Option<Vec<RepositoryFilter>>, // An array of objects representing the filters for a replication rule. Specifying a            repository filter for a replication rule provides a method for controlling which            repositories in a private registry are replicated., Minimum: <code class="code">0</code>, Maximum: <code class="code">100</code>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationDestination {
    #[serde(rename = "Region")]
    pub(crate) region: String, // The Region to replicate to., Required: Yes, Pattern: <code class="code">[0-9a-z-]<span>{</span>2,25}</code>
    #[serde(rename = "RegistryId")]
    pub(crate) registry_id: String, // The AWS account ID of the Amazon ECR private registry to replicate to. When configuring            cross-Region replication within your own registry, specify your own account ID., Required: Yes, Pattern: <code class="code">^[0-9]<span>{</span>12}$</code>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum RepositoryType {
    #[serde(rename = "AWS::ECR::Repository")]
    RepositoryType,
}

ref_struct!(RepositoryRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: RepositoryType,
    #[serde(rename = "Properties")]
    pub(crate) properties: RepositoryProperties,
}
dto_methods!(Repository);

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryProperties {
    #[serde(rename = "ImageTagMutability", skip_serializing_if = "Option::is_none")]
    pub(crate) image_tag_mutability: Option<String>,
    #[serde(rename = "RepositoryPolicyText", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_policy_text: Option<Value>,
    #[serde(rename = "ImageTagMutabilityExclusionFilters", skip_serializing_if = "Option::is_none")]
    pub(crate) image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "EncryptionConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "LifecyclePolicy", skip_serializing_if = "Option::is_none")]
    pub(crate) lifecycle_policy: Option<LifecyclePolicy>,
    #[serde(rename = "EmptyOnDelete", skip_serializing_if = "Option::is_none")]
    pub(crate) empty_on_delete: Option<bool>,
    #[serde(rename = "ImageScanningConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) image_scanning_configuration: Option<ImageScanningConfiguration>,
    #[serde(rename = "RepositoryName", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageTagMutabilityExclusionFilter {
    #[serde(rename = "ImageTagMutabilityExclusionFilterValue")]
    pub(crate) image_tag_mutability_exclusion_filter_value: String, // Property description not available., Required: Yes, Pattern: <code class="code">^[0-9a-zA-Z._*-]<span>{</span>1,128}</code>, Minimum: <code class="code">1</code>, Maximum: <code class="code">128</code>
    #[serde(rename = "ImageTagMutabilityExclusionFilterType")]
    pub(crate) image_tag_mutability_exclusion_filter_type: String, // Property description not available., Required: Yes, Allowed values: <code class="code">WILDCARD</code>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "EncryptionType")]
    pub(crate) encryption_type: String,
    #[serde(rename = "KmsKey", skip_serializing_if = "Option::is_none")]
    pub(crate) kms_key: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecyclePolicy {
    #[serde(rename = "RegistryId", skip_serializing_if = "Option::is_none")]
    pub(crate) registry_id: Option<String>, // The AWS account ID associated with the registry that contains the repository. If you            do  not specify a registry, the default registry is assumed., Pattern: <code class="code">^[0-9]<span>{</span>12}$</code>, Minimum: <code class="code">12</code>, Maximum: <code class="code">12</code>
    #[serde(rename = "LifecyclePolicyText", skip_serializing_if = "Option::is_none")]
    pub(crate) lifecycle_policy_text: Option<String>, // The JSON repository policy text to apply to the repository., Minimum: <code class="code">100</code>, Maximum: <code class="code">30720</code>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageScanningConfiguration {
    #[serde(rename = "ScanOnPush", skip_serializing_if = "Option::is_none")]
    pub(crate) scan_on_push: Option<bool>, // The setting that determines whether images are scanned after being pushed to a            repository. If set to <code class="code">true</code>, images will be scanned after being pushed. If            this parameter is not specified, it will default to <code class="code">false</code> and images will            not be scanned unless a scan is manually started.
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SigningConfigurationType {
    #[serde(rename = "AWS::ECR::SigningConfiguration")]
    SigningConfigurationType,
}

ref_struct!(SigningConfigurationRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct SigningConfiguration {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: SigningConfigurationType,
    #[serde(rename = "Properties")]
    pub(crate) properties: SigningConfigurationProperties,
}
dto_methods!(SigningConfiguration);

#[derive(Debug, Serialize, Deserialize)]
pub struct SigningConfigurationProperties {
    #[serde(rename = "Rules")]
    pub(crate) rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    #[serde(rename = "SigningProfileArn")]
    pub(crate) signing_profile_arn: String,
    #[serde(rename = "RepositoryFilters", skip_serializing_if = "Option::is_none")]
    pub(crate) repository_filters: Option<Vec<RepositoryFilter>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryFilter {
    #[serde(rename = "Filter")]
    pub(crate) filter: String,
    #[serde(rename = "FilterType")]
    pub(crate) filter_type: String,
}

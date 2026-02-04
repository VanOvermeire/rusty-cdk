use crate::iam::PolicyDocument;
use crate::shared::{Id, UpdateDeletePolicyDTO};
use crate::{dto_methods, ref_struct};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum BucketPolicyType {
    #[serde(rename = "AWS::S3::BucketPolicy")]
    BucketPolicyType,
}

ref_struct!(BucketPolicyRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketPolicy {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: BucketPolicyType,
    #[serde(rename = "Properties")]
    pub(crate) properties: S3BucketPolicyProperties,
}
dto_methods!(BucketPolicy);

#[derive(Debug, Serialize, Deserialize)]
pub struct S3BucketPolicyProperties {
    #[serde(rename = "Bucket")]
    pub(crate) bucket_name: Value,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: PolicyDocument,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum BucketType {
    #[serde(rename = "AWS::S3::Bucket")]
    BucketType,
}

ref_struct!(BucketRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: BucketType,
    #[serde(rename = "Properties")]
    pub(super) properties: BucketProperties,
    #[serde(flatten)]
    pub(crate) update_delete_policy_dto: UpdateDeletePolicyDTO,
}
dto_methods!(Bucket);

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketProperties {
    #[serde(rename = "AbacStatus", skip_serializing_if = "Option::is_none")]
    pub(super) abac_status: Option<String>,
    #[serde(rename = "AccelerateConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) accelerate_configuration: Option<AccelerateConfiguration>,
    #[serde(rename = "BucketName", skip_serializing_if = "Option::is_none")]
    pub(super) bucket_name: Option<String>,
    #[serde(rename = "BucketEncryption", skip_serializing_if = "Option::is_none")]
    pub(super) bucket_encryption: Option<BucketEncryption>,
    #[serde(rename = "CorsConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) cors_configuration: Option<CorsConfiguration>,
    #[serde(rename = "IntelligentTieringConfigurations", skip_serializing_if = "Option::is_none")]
    pub(super) intelligent_tiering_configurations: Option<Vec<IntelligentTieringConfiguration>>,
    #[serde(rename = "LifecycleConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) lifecycle_configuration: Option<LifecycleConfiguration>,
    #[serde(rename = "MetadataConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) metadata_configuration: Option<MetadataConfiguration>,
    // notification_configuration is handled by a custom resource
    #[serde(rename = "PublicAccessBlockConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    #[serde(rename = "VersioningConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) versioning_configuration: Option<VersioningConfig>,
    #[serde(rename = "WebsiteConfiguration", skip_serializing_if = "Option::is_none")]
    pub(super) website_configuration: Option<WebsiteConfiguration>,
    // to add //
    // "AnalyticsConfigurations" : [ AnalyticsConfiguration, ... ],
    // "InventoryConfigurations" : [ InventoryConfiguration, ... ],
    // "ReplicationConfiguration" : ReplicationConfiguration,
    // "LoggingConfiguration" : LoggingConfiguration,
    // "MetricsConfigurations" : [ MetricsConfiguration, ... ],

    // less important //
    // "ObjectLockConfiguration" : ObjectLockConfiguration,
    // "ObjectLockEnabled" : Boolean,
    // "OwnershipControls" : OwnershipControls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataConfiguration {
    #[serde(rename = "Destination")]
    pub(super) destination: Option<MetadataDestination>,
    #[serde(rename = "InventoryTableConfiguration")]
    pub(super) inventory_table_configuration: Option<InventoryTableConfiguration>,
    #[serde(rename = "JournalTableConfiguration")]
    pub(super) journal_table_configuration: JournalTableConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataDestination {
    #[serde(rename = "TableBucketArn", skip_serializing_if = "Option::is_none")]
    pub(super) table_bucket_arn: Option<Value>,
    #[serde(rename = "TableBucketType")]
    pub(super) table_bucket_type: String,
    #[serde(rename = "TableNamespace", skip_serializing_if = "Option::is_none")]
    pub(super) table_namespace: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryTableConfiguration {
    #[serde(rename = "ConfigurationState")]
    pub(super) configuration_state: String,
    #[serde(rename = "TableArn", skip_serializing_if = "Option::is_none")]
    pub(super) table_arn: Option<Value>,
    #[serde(rename = "TableName", skip_serializing_if = "Option::is_none")]
    pub(super) table_name: Option<String>,
    // #[serde(rename = "EncryptionConfiguration", skip_serializing_if = "Option::is_none")]
    // pub(super) encryption_configuration: MetadataTableEncryptionConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalTableConfiguration {
    #[serde(rename = "RecordExpiration")]
    pub(super) record_expiration: RecordExpiration,
    #[serde(rename = "TableArn", skip_serializing_if = "Option::is_none")]
    pub(super) table_arn: Option<Value>,
    #[serde(rename = "TableName", skip_serializing_if = "Option::is_none")]
    pub(super) table_name: Option<String>,
    // #[serde(rename = "EncryptionConfiguration", skip_serializing_if = "Option::is_none")]
    // pub(super) encryption_configuration: MetadataTableEncryptionConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordExpiration {
    #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
    pub(super) days: Option<u32>,
    #[serde(rename = "Expiration")]
    pub(super) expiration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligentTieringConfiguration {
    #[serde(rename = "Id")]
    pub(super) id: String,
    #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
    pub(super) prefix: Option<String>,
    #[serde(rename = "Status")]
    pub(super) status: String,
    #[serde(rename = "TagFilters", skip_serializing_if = "Option::is_none")]
    pub(super) tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "Tierings")]
    pub(super) tierings: Vec<Tiering>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tiering {
    #[serde(rename = "AccessTier")]
    pub(super) access_tier: String,
    #[serde(rename = "Days")]
    pub(super) days: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagFilter {
    #[serde(rename = "Key")]
    pub(super) key: String,
    #[serde(rename = "Value")]
    pub(super) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccelerateConfiguration {
    #[serde(rename = "AccelerationStatus")]
    pub(super) acceleration_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketEncryption {
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub(super) server_side_encryption_configuration: Vec<ServerSideEncryptionRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ServerSideEncryptionByDefault")]
    pub(super) server_side_encryption_by_default: ServerSideEncryptionByDefault,
    #[serde(rename = "BucketKeyEnabled", skip_serializing_if = "Option::is_none")]
    pub(super) bucket_key_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub(super) sse_algorithm: String,
    #[serde(rename = "KMSMasterKeyID", skip_serializing_if = "Option::is_none")]
    pub(super) kms_master_key_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorsConfiguration {
    #[serde(rename = "CorsRules")]
    pub(super) cors_rules: Vec<CorsRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorsRule {
    #[serde(rename = "AllowedHeaders", skip_serializing_if = "Option::is_none")]
    pub(super) allowed_headers: Option<Vec<String>>,
    #[serde(rename = "AllowedMethods")]
    pub(super) allowed_methods: Vec<String>,
    #[serde(rename = "AllowedOrigins")]
    pub(super) allowed_origins: Vec<String>,
    #[serde(rename = "ExposedHeaders", skip_serializing_if = "Option::is_none")]
    pub(super) exposed_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge", skip_serializing_if = "Option::is_none")]
    pub(super) max_age: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub(super) rules: Vec<LifecycleRule>,
    #[serde(rename = "TransitionDefaultMinimumObjectSize", skip_serializing_if = "Option::is_none")]
    pub(super) transition_minimum_size: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleRuleTransition {
    #[serde(rename = "StorageClass")]
    pub(super) storage_class: String,
    #[serde(rename = "TransitionInDays")]
    pub(super) transition_in_days: u16, // will become optional once `TransitionDate` is added!
                                        // #[serde(rename = "TransitionDate")]
                                        // pub(super transition_date: String => add and check the regex
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonCurrentVersionTransition {
    #[serde(rename = "StorageClass")]
    pub(super) storage_class: String,
    #[serde(rename = "TransitionInDays")]
    pub(super) transition_in_days: u32,
    #[serde(rename = "NewerNoncurrentVersions")]
    pub(super) newer_non_current_versions: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleRule {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub(super) id: Option<String>,
    #[serde(rename = "Status")]
    pub(super) status: String,
    #[serde(rename = "ExpirationInDays", skip_serializing_if = "Option::is_none")]
    pub(super) expiration_in_days: Option<u16>,
    #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
    pub(super) prefix: Option<String>,
    #[serde(rename = "ObjectSizeGreaterThan", skip_serializing_if = "Option::is_none")]
    pub(super) object_size_greater_than: Option<u32>,
    #[serde(rename = "ObjectSizeLessThan", skip_serializing_if = "Option::is_none")]
    pub(super) object_size_less_than: Option<u32>,
    #[serde(rename = "AbortIncompleteMultipartUpload", skip_serializing_if = "Option::is_none")]
    pub(super) abort_incomplete_multipart_upload: Option<u16>,
    #[serde(rename = "NoncurrentVersionExpiration", skip_serializing_if = "Option::is_none")]
    pub(super) non_current_version_expiration: Option<u16>,
    #[serde(rename = "Transitions", skip_serializing_if = "Option::is_none")]
    pub(super) transitions: Option<Vec<LifecycleRuleTransition>>,
    #[serde(rename = "NoncurrentVersionTransitions", skip_serializing_if = "Option::is_none")]
    pub(super) non_current_version_transitions: Option<Vec<NonCurrentVersionTransition>>,
    // #[serde(rename = "ExpiredObjectDeleteMarker", skip_serializing_if = "Option::is_none")]
    // pub(super) expire_object_delete_marker: Option<bool> => cannot be specified with ExpirationInDays, ExpirationDate, or TagFilters.
    // "ExpirationDate": String => check the regex
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicConfiguration {
    #[serde(rename = "Event")]
    pub(super) event: String,
    #[serde(rename = "Topic")]
    pub(super) topic: String,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub(super) filter: Option<NotificationFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueConfiguration {
    #[serde(rename = "Event")]
    pub(super) event: String,
    #[serde(rename = "Queue")]
    pub(super) queue: String,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub(super) filter: Option<NotificationFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LambdaConfiguration {
    #[serde(rename = "Event")]
    pub(super) event: String,
    #[serde(rename = "Function")]
    pub(super) function: String,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub(super) filter: Option<NotificationFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationFilter {
    #[serde(rename = "S3Key", skip_serializing_if = "Option::is_none")]
    pub(super) s3_key: Option<S3KeyFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3KeyFilter {
    #[serde(rename = "Rules")]
    pub(super) rules: Vec<FilterRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterRule {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "Value")]
    pub(super) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "BlockPublicAcls", skip_serializing_if = "Option::is_none")]
    pub(super) block_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicPolicy", skip_serializing_if = "Option::is_none")]
    pub(super) block_public_policy: Option<bool>,
    #[serde(rename = "IgnorePublicAcls", skip_serializing_if = "Option::is_none")]
    pub(super) ignore_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets", skip_serializing_if = "Option::is_none")]
    pub(super) restrict_public_buckets: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersioningConfig {
    #[serde(rename = "Status")]
    pub(super) status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteConfiguration {
    #[serde(rename = "IndexDocument", skip_serializing_if = "Option::is_none")]
    pub(super) index_document: Option<String>,
    #[serde(rename = "ErrorDocument", skip_serializing_if = "Option::is_none")]
    pub(super) error_document: Option<String>,
    #[serde(rename = "RedirectAllRequestsTo", skip_serializing_if = "Option::is_none")]
    pub(super) redirect_all_requests_to: Option<RedirectAllRequestsTo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectAllRequestsTo {
    #[serde(rename = "HostName")]
    pub(super) host_name: String,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub(super) protocol: Option<String>,
}

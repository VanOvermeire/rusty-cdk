use serde::Serialize;
use serde_json::Value;
use crate::iam::PolicyDocument;
use crate::intrinsic_functions::{get_arn, get_att, get_ref};
use crate::shared::Id;

#[derive(Debug, Serialize)]
pub struct BucketPolicy {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(skip)]
    pub(crate) referenced_ids: Vec<String>,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: S3BucketPolicyProperties,
}

impl BucketPolicy {
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }

    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }

    pub fn get_referenced_ids(&self) -> Vec<&str> {
        self.referenced_ids.iter().map(|r| r.as_str()).collect()
    }
}

#[derive(Debug, Serialize)]
pub struct S3BucketPolicyProperties {
    #[serde(rename = "Bucket")]
    pub(crate) bucket_name: Value,
    #[serde(rename = "PolicyDocument")]
    pub(crate) policy_document: PolicyDocument,
}

#[derive(Debug, Serialize)]
pub struct Bucket {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: BucketProperties,
}

impl Bucket {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }

    pub fn get_arn(&self) -> Value {
        get_arn(self.get_resource_id())
    }
    
    pub fn get_att(&self, att: &str) -> Value {
        get_att(self.get_resource_id(), att)
    }
}

#[derive(Debug, Serialize)]
pub struct BucketProperties {
    #[serde(rename = "BucketName", skip_serializing_if = "Option::is_none")]
    pub(crate) bucket_name: Option<String>,
    #[serde(rename = "BucketEncryption", skip_serializing_if = "Option::is_none")]
    pub(crate) bucket_encryption: Option<BucketEncryption>,
    #[serde(rename = "CorsConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) cors_configuration: Option<CorsConfiguration>,
    #[serde(rename = "LifecycleConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) lifecycle_configuration: Option<LifecycleConfiguration>,
    #[serde(rename = "NotificationConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) notification_configuration: Option<NotificationConfiguration>,
    #[serde(rename = "PublicAccessBlockConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    #[serde(rename = "VersioningConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) versioning_configuration: Option<VersioningConfiguration>,
    #[serde(rename = "WebsiteConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) website_configuration: Option<WebsiteConfiguration>,
}

#[derive(Debug, Serialize)]
pub struct BucketEncryption {
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub(crate) server_side_encryption_configuration: Vec<ServerSideEncryptionRule>,
}

#[derive(Debug, Serialize)]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ServerSideEncryptionByDefault")]
    pub(crate) server_side_encryption_by_default: ServerSideEncryptionByDefault,
    #[serde(rename = "BucketKeyEnabled", skip_serializing_if = "Option::is_none")]
    pub(crate) bucket_key_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub(crate) sse_algorithm: String,
    #[serde(rename = "KMSMasterKeyID", skip_serializing_if = "Option::is_none")]
    pub(crate) kms_master_key_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CorsConfiguration {
    #[serde(rename = "CorsRules")]
    pub(crate) cors_rules: Vec<CorsRule>,
}

#[derive(Debug, Serialize)]
pub struct CorsRule {
    #[serde(rename = "AllowedHeaders", skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_headers: Option<Vec<String>>,
    #[serde(rename = "AllowedMethods")]
    pub(crate) allowed_methods: Vec<String>,
    #[serde(rename = "AllowedOrigins")]
    pub(crate) allowed_origins: Vec<String>,
    #[serde(rename = "ExposedHeaders", skip_serializing_if = "Option::is_none")]
    pub(crate) exposed_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge", skip_serializing_if = "Option::is_none")]
    pub(crate) max_age: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub(crate) rules: Vec<LifecycleRule>,
    #[serde(rename = "TransitionDefaultMinimumObjectSize", skip_serializing_if = "Option::is_none")]
    pub(crate) transition_minimum_size: Option<String>
}

#[derive(Debug, Serialize)]
pub struct LifecycleRuleTransition {
    #[serde(rename = "StorageClass")]
    pub(crate) storage_class: String,
    #[serde(rename = "TransitionInDays")]
    pub(crate) transition_in_days: u32, // will become optional once `TransitionDate` is added
    // #[serde(rename = "TransitionDate")]
    // pub(crate) transition_date: String => add and check the regex
}

#[derive(Debug, Serialize)]
pub struct NonCurrentVersionTransition {
    #[serde(rename = "StorageClass")]
    pub(crate) storage_class: String,
    #[serde(rename = "TransitionInDays")]
    pub(crate) transition_in_days: u32,
    #[serde(rename = "NewerNoncurrentVersions")]
    pub(crate) newer_non_current_versions: Option<u32>
}

#[derive(Debug, Serialize)]
pub struct LifecycleRule {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,
    #[serde(rename = "Status")]
    pub(crate) status: String,
    #[serde(rename = "ExpirationInDays", skip_serializing_if = "Option::is_none")]
    pub(crate) expiration_in_days: Option<u16>,
    #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
    pub(crate) prefix: Option<String>,
    #[serde(rename = "ObjectSizeGreaterThan", skip_serializing_if = "Option::is_none")]
    pub(crate) object_size_greater_than: Option<u32>,
    #[serde(rename = "ObjectSizeLessThan", skip_serializing_if = "Option::is_none")]
    pub(crate) object_size_less_than: Option<u32>,
    #[serde(rename = "AbortIncompleteMultipartUpload", skip_serializing_if = "Option::is_none")]
    pub(crate) abort_incomplete_multipart_upload: Option<u16>,
    #[serde(rename = "NoncurrentVersionExpiration", skip_serializing_if = "Option::is_none")]
    pub(crate) non_current_version_expiration: Option<u16>,
    #[serde(rename = "Transitions", skip_serializing_if = "Option::is_none")]
    pub(crate) transitions: Option<Vec<LifecycleRuleTransition>>,
    #[serde(rename = "NoncurrentVersionTransitions", skip_serializing_if = "Option::is_none")]
    pub(crate) non_current_version_transitions: Option<Vec<NonCurrentVersionTransition>>
    // #[serde(rename = "ExpiredObjectDeleteMarker", skip_serializing_if = "Option::is_none")]
    // pub(crate) expire_object_delete_marker: Option<bool> => cannot be specified with ExpirationInDays, ExpirationDate, or TagFilters.
    // "ExpirationDate": String => check the regex
}

#[derive(Debug, Serialize)]
pub struct NotificationConfiguration {
    #[serde(rename = "TopicConfigurations", skip_serializing_if = "Option::is_none")]
    pub(crate) topic_configurations: Option<Vec<TopicConfiguration>>,
    #[serde(rename = "QueueConfigurations", skip_serializing_if = "Option::is_none")]
    pub(crate) queue_configurations: Option<Vec<QueueConfiguration>>, // fifo not allowed!
    #[serde(rename = "LambdaConfigurations", skip_serializing_if = "Option::is_none")]
    pub(crate) lambda_configurations: Option<Vec<LambdaConfiguration>>,
}

#[derive(Debug, Serialize)]
pub struct TopicConfiguration {
    #[serde(rename = "Event")]
    pub(crate) event: String,
    #[serde(rename = "Topic")]
    pub(crate) topic: String,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<NotificationFilter>,
}

#[derive(Debug, Serialize)]
pub struct QueueConfiguration {
    #[serde(rename = "Event")]
    pub(crate) event: String,
    #[serde(rename = "Queue")]
    pub(crate) queue: String,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<NotificationFilter>,
}

#[derive(Debug, Serialize)]
pub struct LambdaConfiguration {
    #[serde(rename = "Event")]
    pub(crate) event: String,
    #[serde(rename = "Function")]
    pub(crate) function: String,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<NotificationFilter>,
}

#[derive(Debug, Serialize)]
pub struct NotificationFilter {
    #[serde(rename = "S3Key", skip_serializing_if = "Option::is_none")]
    pub(crate) s3_key: Option<S3KeyFilter>,
}

#[derive(Debug, Serialize)]
pub struct S3KeyFilter {
    #[serde(rename = "Rules")]
    pub(crate) rules: Vec<FilterRule>,
}

#[derive(Debug, Serialize)]
pub struct FilterRule {
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "Value")]
    pub(crate) value: String,
}

#[derive(Debug, Serialize)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "BlockPublicAcls", skip_serializing_if = "Option::is_none")]
    pub(crate) block_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicPolicy", skip_serializing_if = "Option::is_none")]
    pub(crate) block_public_policy: Option<bool>,
    #[serde(rename = "IgnorePublicAcls", skip_serializing_if = "Option::is_none")]
    pub(crate) ignore_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets", skip_serializing_if = "Option::is_none")]
    pub(crate) restrict_public_buckets: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct VersioningConfiguration {
    #[serde(rename = "Status")]
    pub(crate) status: String,
}

#[derive(Debug, Serialize)]
pub struct WebsiteConfiguration {
    #[serde(rename = "IndexDocument", skip_serializing_if = "Option::is_none")]
    pub(crate) index_document: Option<String>,
    #[serde(rename = "ErrorDocument", skip_serializing_if = "Option::is_none")]
    pub(crate) error_document: Option<String>,
    #[serde(rename = "RedirectAllRequestsTo", skip_serializing_if = "Option::is_none")]
    pub(crate) redirect_all_requests_to: Option<RedirectAllRequestsTo>,
}

#[derive(Debug, Serialize)]
pub struct RedirectAllRequestsTo {
    #[serde(rename = "HostName")]
    pub(crate) host_name: String,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub(crate) protocol: Option<String>,
}

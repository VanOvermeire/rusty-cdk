use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;
use crate::shared::Id;

#[derive(Debug, Serialize)]
pub struct S3Bucket {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: S3BucketProperties,
}

impl S3Bucket {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_resource_id())
    }
}

#[derive(Debug, Serialize)]
pub struct S3BucketProperties {
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
}

#[derive(Debug, Serialize)]
pub struct LifecycleRule {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,
    #[serde(rename = "Status")]
    pub(crate) status: String,
    #[serde(rename = "ExpirationInDays", skip_serializing_if = "Option::is_none")]
    pub(crate) expiration_in_days: Option<u32>,
    #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
    pub(crate) prefix: Option<String>,
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

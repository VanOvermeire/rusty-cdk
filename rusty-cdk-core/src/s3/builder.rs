use crate::custom_resource::{BucketNotificationBuilder, BUCKET_NOTIFICATION_HANDLER_CODE};
use crate::iam::{CustomPermission, Effect, Permission, PolicyDocument, PolicyDocumentBuilder, PrincipalBuilder, StatementBuilder};
use crate::intrinsic::join;
use crate::lambda::{Architecture, Runtime};
use crate::lambda::{Code, FunctionBuilder, FunctionRef, PermissionBuilder};
use crate::s3::{
    dto, AccelerateConfiguration, IntelligentTieringConfiguration, InventoryTableConfiguration,
    JournalTableConfiguration, MetadataConfiguration, MetadataDestination, RecordExpiration, TagFilter, Tiering,
};
use crate::s3::{
    Bucket, BucketEncryption, BucketPolicy, BucketPolicyRef, BucketProperties, BucketRef, CorsConfiguration, CorsRule,
    LifecycleConfiguration, LifecycleRule, LifecycleRuleTransition, NonCurrentVersionTransition, PublicAccessBlockConfiguration,
    RedirectAllRequestsTo, S3BucketPolicyProperties, ServerSideEncryptionByDefault, ServerSideEncryptionRule, WebsiteConfiguration,
};
use crate::shared::http::{HttpMethod, Protocol};
use crate::shared::{DeletionPolicy, Id, UpdateDeletePolicyDTO, UpdateReplacePolicy, QUEUE_POLICY_ID_SUFFIX, TOPIC_POLICY_ID_SUFFIX};
use crate::sns::{TopicPolicyBuilder, TopicRef};
use crate::sqs::{QueuePolicyBuilder, QueueRef};
use crate::stack::{Resource, StackBuilder};
use crate::type_state;
use crate::wrappers::{
    BucketName, BucketTiering, IamAction, LambdaPermissionAction, LifecycleTransitionInDays, Memory, RecordExpirationDays,
    S3LifecycleObjectSizes, Timeout,
};
use serde_json::{json, Value};
use std::marker::PhantomData;
use std::time::Duration;

/// Builder for S3 bucket policies.
///
/// Creates a policy document that controls access to an S3 bucket.
///
/// # Example
///
/// ```rust,no_run
/// use serde_json::Value;
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::s3::BucketPolicyBuilder;
/// use rusty_cdk_core::iam::{PolicyDocumentBuilder, StatementBuilder, Effect, PrincipalBuilder};
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_core::s3::BucketBuilder;
/// use rusty_cdk_macros::iam_action;
///
/// let mut stack_builder = StackBuilder::new();
/// let bucket = unimplemented!("create a bucket");
///
/// let resources = vec![Value::String("*".to_string())];
/// let statement = StatementBuilder::new(
///         vec![iam_action!("s3:GetObject")],
///         Effect::Allow
///     )
///     .principal(PrincipalBuilder::new().normal("*").build())
///     .resources(resources)
///     .build();
///
/// let policy_doc = PolicyDocumentBuilder::new(vec![statement]).build();
/// let policy = BucketPolicyBuilder::new("bucket-policy", &bucket, policy_doc)
///     .build(&mut stack_builder);
/// ```
pub struct BucketPolicyBuilder {
    id: Id,
    bucket_name: Value,
    policy_document: PolicyDocument,
}

impl BucketPolicyBuilder {
    /// Creates a new S3 bucket policy builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the bucket policy
    /// * `bucket` - Reference to the S3 bucket
    /// * `policy_document` - IAM policy document controlling access
    pub fn new(id: &str, bucket: &BucketRef, policy_document: PolicyDocument) -> Self {
        Self {
            id: Id(id.to_string()),
            bucket_name: bucket.get_ref(),
            policy_document,
        }
    }

    pub(crate) fn new_with_bucket_ref(id: &str, bucket_name: Value, policy_document: PolicyDocument) -> Self {
        Self {
            id: Id(id.to_string()),
            bucket_name,
            policy_document,
        }
    }

    pub(crate) fn raw_build(self) -> (String, BucketPolicy) {
        let resource_id = Resource::generate_id("S3BucketPolicy");
        let policy = BucketPolicy {
            id: self.id,
            resource_id: resource_id.to_string(),
            r#type: "AWS::S3::BucketPolicy".to_string(),
            properties: S3BucketPolicyProperties {
                bucket_name: self.bucket_name,
                policy_document: self.policy_document,
            },
        };
        (resource_id, policy)
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> BucketPolicyRef {
        let (resource_id, policy) = self.raw_build();
        stack_builder.add_resource(policy);
        BucketPolicyRef::new(resource_id)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Encryption {
    S3Managed,
    KmsManaged,
    DsseManaged,
    // KMS, => add, this requires creating a kms key and passing it to the bucket
    // DSSE, => add, similar
}

impl From<Encryption> for String {
    fn from(value: Encryption) -> Self {
        match value {
            Encryption::S3Managed => "AES256".to_string(),
            Encryption::KmsManaged => "aws:kms".to_string(),
            Encryption::DsseManaged => "aws:kms:dsse".to_string(),
        }
    }
}

pub enum NotificationDestination<'a> {
    Lambda(&'a FunctionRef, NotificationEventType),
    Sns(&'a TopicRef, NotificationEventType),
    Sqs(&'a QueueRef, NotificationEventType),
}

#[derive(Debug, Clone)]
pub enum NotificationEventType {
    ObjectCreated,
    ObjectCreatedPut,
    ObjectCreatedPost,
    ObjectCreatedCopy,
    ObjectCreatedCompleteMultipartUpload,
    ObjectRemoved,
    ObjectRemovedDelete,
    ObjectRemovedDeleteMarkerCreated,
    ObjectRestorePost,
    ObjectRestoreCompleted,
    ObjectRestoreDelete,
    ReducedRedundancyLostObject,
    ReplicationOperationFailedReplication,
    ReplicationOperationMissedThreshold,
    ReplicationOperationReplicatedAfterThreshold,
    ReplicationOperationNotTracked,
    LifecycleExpiration,
    LifecycleExpirationDelete,
    LifecycleExpirationDeleteMarkerCreated,
    LifecycleTransition,
    IntelligentTiering,
    ObjectTagging,
    ObjectTaggingPut,
    ObjectTaggingDelete,
    ObjectAclPut,
    ObjectRestore,
    REPLICATION,
}

#[derive(Debug, Clone)]
pub enum AbacStatus {
    Enabled,
    Disabled,
}

impl From<AbacStatus> for String {
    fn from(value: AbacStatus) -> Self {
        match value {
            AbacStatus::Enabled => "Enabled".to_string(),
            AbacStatus::Disabled => "Disabled".to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AccelerationStatus {
    Enabled,
    Suspended,
}

impl From<AccelerationStatus> for String {
    fn from(value: AccelerationStatus) -> Self {
        match value {
            AccelerationStatus::Enabled => "Enabled".to_string(),
            AccelerationStatus::Suspended => "Suspended".to_string(),
        }
    }
}

impl From<NotificationEventType> for String {
    fn from(value: NotificationEventType) -> Self {
        match value {
            NotificationEventType::ObjectCreated => "s3:ObjectCreated:*".to_string(),
            NotificationEventType::ObjectCreatedPut => "s3:ObjectCreated:Put".to_string(),
            NotificationEventType::ObjectCreatedPost => "s3:ObjectCreated:Post".to_string(),
            NotificationEventType::ObjectCreatedCopy => "s3:ObjectCreated:Copy".to_string(),
            NotificationEventType::ObjectCreatedCompleteMultipartUpload => "s3:ObjectCreated:CompleteMultipartUpload".to_string(),
            NotificationEventType::ObjectRemoved => "s3:ObjectRemoved:*".to_string(),
            NotificationEventType::ObjectRemovedDelete => "s3:ObjectRemoved:Delete".to_string(),
            NotificationEventType::ObjectRemovedDeleteMarkerCreated => "s3:ObjectRemoved:DeleteMarkerCreated".to_string(),
            NotificationEventType::ObjectRestorePost => "s3:ObjectRestore:Post".to_string(),
            NotificationEventType::ObjectRestoreCompleted => "s3:ObjectRestore:Completed".to_string(),
            NotificationEventType::ObjectRestoreDelete => "s3:ObjectRestore:Delete".to_string(),
            NotificationEventType::ReducedRedundancyLostObject => "s3:ReducedRedundancyLostObject".to_string(),
            NotificationEventType::ReplicationOperationFailedReplication => "s3:Replication:OperationFailedReplication".to_string(),
            NotificationEventType::ReplicationOperationMissedThreshold => "s3:Replication:OperationMissedThreshold".to_string(),
            NotificationEventType::ReplicationOperationReplicatedAfterThreshold => {
                "s3:Replication:OperationReplicatedAfterThreshold".to_string()
            }
            NotificationEventType::ReplicationOperationNotTracked => "s3:Replication:OperationNotTracked".to_string(),
            NotificationEventType::LifecycleExpiration => "s3:LifecycleExpiration:*".to_string(),
            NotificationEventType::LifecycleExpirationDelete => "s3:LifecycleExpiration:Delete".to_string(),
            NotificationEventType::LifecycleExpirationDeleteMarkerCreated => "s3:LifecycleExpiration:DeleteMarkerCreated".to_string(),
            NotificationEventType::LifecycleTransition => "s3:LifecycleTransition".to_string(),
            NotificationEventType::IntelligentTiering => "s3:IntelligentTiering".to_string(),
            NotificationEventType::ObjectTagging => "s3:ObjectTagging:*".to_string(),
            NotificationEventType::ObjectTaggingPut => "s3:ObjectTagging:Put".to_string(),
            NotificationEventType::ObjectTaggingDelete => "s3:ObjectTagging:Delete".to_string(),
            NotificationEventType::ObjectAclPut => "s3:ObjectAcl:Put".to_string(),
            NotificationEventType::ObjectRestore => "s3:ObjectRestore:*".to_string(),
            NotificationEventType::REPLICATION => "s3:Replication:*".to_string(),
        }
    }
}

type_state!(BucketBuilderState, StartState, WebsiteState,);

/// Builder for S3 buckets.
///
/// Provides configuration for S3 buckets including versioning, lifecycle rules, encryption, CORS, and static website hosting.
///
/// # Example
///
/// ```rust,compile_fail
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::s3::{BucketBuilder, VersioningConfig, Encryption, VersioningConfiguration};
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::bucket_name;
///
/// let mut stack_builder = StackBuilder::new();
///
/// // Create a simple bucket
/// let bucket = BucketBuilder::new("my-bucket")
///     .name(bucket_name!("my-unique-bucket"))
///     .versioning_configuration(VersioningConfiguration::Enabled)
///     .encryption(Encryption::S3Managed)
///     .build(&mut stack_builder);
///
/// // Create a website bucket
/// let (website_bucket, policy) = BucketBuilder::new("website-bucket")
///     .website("index.html")
///     .error_document("error.html")
///     .build(&mut stack_builder);
/// ```
pub struct BucketBuilder<T: BucketBuilderState> {
    phantom_data: PhantomData<T>,
    id: Id,
    abac_status: Option<String>,
    acceleration_status: Option<String>,
    name: Option<String>,
    access: Option<PublicAccessBlockConfiguration>,
    intelligent_tiering_configurations: Option<Vec<IntelligentTieringConfiguration>>,
    metadata_configuration: Option<MetadataConfiguration>,
    versioning_configuration: Option<VersioningConfiguration>,
    lifecycle_configuration: Option<LifecycleConfiguration>,
    index_document: Option<String>,
    error_document: Option<String>,
    redirect_all_requests_to: Option<(String, Option<Protocol>)>,
    cors_config: Option<CorsConfiguration>,
    bucket_encryption: Option<Encryption>,
    bucket_notification_lambda_destinations: Vec<(Value, String)>,
    bucket_notification_sns_destinations: Vec<(Id, Value, String)>,
    bucket_notification_sqs_destinations: Vec<(Id, Value, Value, String)>,
    deletion_policy: Option<String>,
    update_replace_policy: Option<String>,
}

impl BucketBuilder<StartState> {
    /// Creates a new S3 bucket builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the bucket
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            phantom_data: Default::default(),
            abac_status: None,
            acceleration_status: None,
            name: None,
            access: None,
            intelligent_tiering_configurations: None,
            metadata_configuration: None,
            versioning_configuration: None,
            lifecycle_configuration: None,
            index_document: None,
            error_document: None,
            redirect_all_requests_to: None,
            cors_config: None,
            bucket_encryption: None,
            bucket_notification_lambda_destinations: vec![],
            bucket_notification_sns_destinations: vec![],
            bucket_notification_sqs_destinations: vec![],
            deletion_policy: None,
            update_replace_policy: None,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> BucketRef {
        let (bucket, _) = self.build_internal(false, stack_builder);
        bucket
    }
}

impl<T: BucketBuilderState> BucketBuilder<T> {
    pub fn name(self, name: BucketName) -> Self {
        Self {
            name: Some(name.0),
            ..self
        }
    }

    pub fn abac_status(self, abac_status: AbacStatus) -> Self {
        Self {
            abac_status: Some(abac_status.into()),
            ..self
        }
    }

    pub fn acceleration_status(self, acceleration_status: AccelerationStatus) -> Self {
        Self {
            acceleration_status: Some(acceleration_status.into()),
            ..self
        }
    }

    pub fn metadata_configuration(self, config: MetadataConfiguration) -> Self {
        Self {
            metadata_configuration: Some(config),
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

    pub fn encryption(self, encryption: Encryption) -> Self {
        Self {
            bucket_encryption: Some(encryption),
            ..self
        }
    }

    pub fn update_replace_and_deletion_policy(self, update_replace_policy: UpdateReplacePolicy, deletion_policy: DeletionPolicy) -> Self {
        Self {
            deletion_policy: Some(deletion_policy.into()),
            update_replace_policy: Some(update_replace_policy.into()),
            ..self
        }
    }

    pub fn add_intelligent_tiering(mut self, tiering: IntelligentTieringConfiguration) -> Self {
        if let Some(mut config) = self.intelligent_tiering_configurations {
            config.push(tiering);
            self.intelligent_tiering_configurations = Some(config);
        } else {
            self.intelligent_tiering_configurations = Some(vec![tiering]);
        }
        self
    }

    pub fn add_notification(mut self, destination: NotificationDestination) -> Self {
        match destination {
            NotificationDestination::Lambda(l, e) => self.bucket_notification_lambda_destinations.push((l.get_arn(), e.into())),
            NotificationDestination::Sns(s, e) => {
                self.bucket_notification_sns_destinations
                    .push((s.get_id().clone(), s.get_ref(), e.into()))
            }
            NotificationDestination::Sqs(q, e) => {
                self.bucket_notification_sqs_destinations
                    .push((q.get_id().clone(), q.get_ref(), q.get_arn(), e.into()))
            }
        }
        self
    }

    /// Configures the bucket for static website hosting.
    ///
    /// Automatically disables public access blocks and creates a bucket policy
    /// allowing public GetObject access.
    pub fn website<I: Into<String>>(self, index_document: I) -> BucketBuilder<WebsiteState> {
        BucketBuilder {
            phantom_data: Default::default(),
            id: self.id,
            abac_status: self.abac_status,
            acceleration_status: self.acceleration_status,
            name: self.name,
            access: self.access,
            intelligent_tiering_configurations: self.intelligent_tiering_configurations,
            metadata_configuration: self.metadata_configuration,
            versioning_configuration: self.versioning_configuration,
            lifecycle_configuration: self.lifecycle_configuration,
            index_document: Some(index_document.into()),
            error_document: self.error_document,
            redirect_all_requests_to: self.redirect_all_requests_to,
            cors_config: self.cors_config,
            bucket_encryption: self.bucket_encryption,
            bucket_notification_lambda_destinations: self.bucket_notification_lambda_destinations,
            bucket_notification_sns_destinations: self.bucket_notification_sns_destinations,
            bucket_notification_sqs_destinations: self.bucket_notification_sqs_destinations,
            deletion_policy: self.deletion_policy,
            update_replace_policy: self.update_replace_policy,
        }
    }

    fn build_internal(self, website: bool, stack_builder: &mut StackBuilder) -> (BucketRef, Option<BucketPolicyRef>) {
        let resource_id = Resource::generate_id("S3Bucket");

        let versioning_configuration = self.versioning_configuration.map(|c| dto::VersioningConfig { status: c.into() });

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
            // required for an S3 website
            Some(PublicAccessBlockConfiguration {
                block_public_policy: Some(false),
                restrict_public_buckets: Some(false),
                block_public_acls: Some(true),
                ignore_public_acls: Some(true),
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

        let properties = BucketProperties {
            abac_status: self.abac_status,
            accelerate_configuration: self.acceleration_status.map(|v| AccelerateConfiguration { acceleration_status: v }),
            bucket_name: self.name,
            cors_configuration: self.cors_config,
            intelligent_tiering_configurations: self.intelligent_tiering_configurations,
            lifecycle_configuration: self.lifecycle_configuration,
            public_access_block_configuration: access,
            versioning_configuration,
            website_configuration,
            bucket_encryption: encryption,
            metadata_configuration: self.metadata_configuration,
        };

        stack_builder.add_resource(Bucket {
            id: self.id.clone(),
            resource_id: resource_id.clone(),
            r#type: "AWS::S3::Bucket".to_string(),
            properties,
            update_delete_policy_dto: UpdateDeletePolicyDTO {
                deletion_policy: self.deletion_policy,
                update_replace_policy: self.update_replace_policy,
            },
        });

        let bucket = BucketRef::new(resource_id);

        let policy = if website {
            // website needs a policy to allow GETs
            let bucket_resource = vec![join("", vec![bucket.get_arn(), Value::String("/*".to_string())])];
            let statement = StatementBuilder::new(vec![IamAction("s3:GetObject".to_string())], Effect::Allow)
                .resources(bucket_resource)
                .principal(PrincipalBuilder::new().normal("*").build())
                .build();
            let policy_doc = PolicyDocumentBuilder::new(vec![statement]).build();
            let bucket_policy_id = Id::generate_id(&self.id, "S3Policy");
            let s3_policy = BucketPolicyBuilder::new(&bucket_policy_id, &bucket, policy_doc).build(stack_builder);
            Some(s3_policy)
        } else {
            None
        };

        for (i, (arn, event)) in self.bucket_notification_lambda_destinations.into_iter().enumerate() {
            let permission_id = Id::generate_id(&self.id, format!("LambdaDestPerm{}", i).as_str());
            let permission = PermissionBuilder::new(
                &permission_id,
                LambdaPermissionAction("lambda:InvokeFunction".to_string()),
                arn.clone(),
                "s3.amazonaws.com",
            )
            .source_arn(bucket.get_arn())
            .current_account()
            .build(stack_builder);
            let handler = Self::notification_handler(&self.id, "Lambda", i, stack_builder);
            let notification_id = Id::generate_id(&self.id, &format!("LambdaNotification{}", i));
            BucketNotificationBuilder::new(
                &notification_id,
                handler.get_arn(),
                bucket.get_ref(),
                event,
                Some(permission.get_id().clone()),
            )
            .lambda(arn)
            .build(stack_builder);
        }

        for (i, (id, reference, event)) in self.bucket_notification_sns_destinations.into_iter().enumerate() {
            let handler = Self::notification_handler(&self.id, "SNS", i, stack_builder);

            let bucket_arn = bucket.get_arn();
            let condition = json!({
                "ArnLike": {
                    "aws:SourceArn": bucket_arn
                }
            });
            let principal = PrincipalBuilder::new().service("s3.amazonaws.com".to_string()).build();
            let statement = StatementBuilder::new(vec![IamAction("sns:Publish".to_string())], Effect::Allow)
                .principal(principal)
                .condition(condition)
                .resources(vec![reference.clone()])
                .build();

            let topic_policy_id = Id::generate_id(&id, TOPIC_POLICY_ID_SUFFIX);
            let topic_policy_ref_id = match stack_builder.get_resource(&topic_policy_id) {
                None => {
                    // there's no queue policy. add ours
                    let doc = PolicyDocumentBuilder::new(vec![statement]).build();
                    let topic_policy_id = Id::generate_id(&self.id, &format!("SNSDestinationPolicy{}", i));
                    TopicPolicyBuilder::new_with_values(topic_policy_id.clone(), doc, vec![reference.clone()]).build(stack_builder);
                    topic_policy_id
                }
                Some(Resource::TopicPolicy(pol)) => {
                    // there's a policy, add the required permissions
                    pol.properties.doc.statements.push(statement);
                    topic_policy_id
                }
                _ => unreachable!("topic policy id should point to optional topic policy"),
            };

            let notification_id = Id::generate_id(&self.id, &format!("SNSNotification{}", i));
            BucketNotificationBuilder::new(
                &notification_id,
                handler.get_arn(),
                bucket.get_ref(),
                event,
                Some(topic_policy_ref_id),
            )
            .sns(reference)
            .build(stack_builder);
        }

        for (i, (id, reference, arn, event)) in self.bucket_notification_sqs_destinations.into_iter().enumerate() {
            let handler = Self::notification_handler(&self.id, "SQS", i, stack_builder);

            let bucket_arn = bucket.get_arn();
            let condition = json!({
                "ArnLike": {
                    "aws:SourceArn": bucket_arn
                }
            });
            let principal = PrincipalBuilder::new().service("s3.amazonaws.com".to_string()).build();
            let statement = StatementBuilder::new(
                vec![
                    IamAction("sqs:GetQueueAttributes".to_string()),
                    IamAction("sqs:GetQueueUrl".to_string()),
                    IamAction("sqs:SendMessage".to_string()),
                ],
                Effect::Allow,
            )
            .principal(principal)
            .condition(condition)
            .resources(vec![arn.clone()])
            .build();

            let queue_policy_id = Id::generate_id(&id, QUEUE_POLICY_ID_SUFFIX);

            let queue_policy_ref_id = match stack_builder.get_resource(&queue_policy_id) {
                None => {
                    // there's no queue policy. add ours
                    let doc = PolicyDocumentBuilder::new(vec![statement]).build();
                    let queue_policy_id = Id::generate_id(&self.id, &format!("SQSDestinationPolicy{}", i));
                    QueuePolicyBuilder::new_with_values(queue_policy_id.clone(), doc, vec![reference.clone()]).build(stack_builder);
                    queue_policy_id
                }
                Some(Resource::QueuePolicy(pol)) => {
                    // there's a policy, add the required permissions
                    pol.properties.doc.statements.push(statement);
                    queue_policy_id
                }
                _ => unreachable!("queue policy id should point to optional queue policy"),
            };

            let notification_id = Id::generate_id(&self.id, format!("SQSNotification{}", i).as_str());
            BucketNotificationBuilder::new(
                &notification_id,
                handler.get_arn(),
                bucket.get_ref(),
                event,
                Some(queue_policy_ref_id),
            )
            .sqs(arn)
            .build(stack_builder);
        }

        (bucket, policy)
    }

    fn notification_handler(id: &Id, target: &str, num: usize, stack_builder: &mut StackBuilder) -> FunctionRef {
        let handler_id = Id::generate_id(id, &format!("{}Handler{}", target, num));
        let (handler, ..) = FunctionBuilder::new(&handler_id, Architecture::X86_64, Memory(128), Timeout(300))
            .code(Code::Inline(BUCKET_NOTIFICATION_HANDLER_CODE.to_string()))
            .handler("index.handler")
            .runtime(Runtime::Python313)
            .add_permission(Permission::Custom(CustomPermission::new(
                "NotificationPermission",
                StatementBuilder::new(vec![IamAction("s3:PutBucketNotification".to_string())], Effect::Allow)
                    .all_resources()
                    .build(),
            )))
            .build(stack_builder);
        handler
    }
}

impl BucketBuilder<WebsiteState> {
    pub fn error_document<I: Into<String>>(self, error: I) -> Self {
        Self {
            error_document: Some(error.into()),
            ..self
        }
    }

    pub fn redirect_all<I: Into<String>>(self, hostname: I, protocol: Option<Protocol>) -> Self {
        Self {
            redirect_all_requests_to: Some((hostname.into(), protocol)),
            ..self
        }
    }

    pub fn cors_config(self, config: CorsConfiguration) -> Self {
        Self {
            cors_config: Some(config),
            ..self
        }
    }

    /// Builds the website bucket and adds it to the stack.
    ///
    /// Returns both the bucket and the automatically created bucket policy
    /// that allows public read access.
    pub fn build(self, stack_builder: &mut StackBuilder) -> (BucketRef, BucketPolicyRef) {
        let (bucket, policy) = self.build_internal(true, stack_builder);
        (bucket, policy.expect("for website, bucket policy should always be present"))
    }
}

/// Builder for S3 CORS configuration.
pub struct CorsConfigurationBuilder {
    rules: Vec<CorsRule>,
}

impl CorsConfigurationBuilder {
    pub fn new(rules: Vec<CorsRule>) -> CorsConfigurationBuilder {
        CorsConfigurationBuilder { rules }
    }

    pub fn build(self) -> CorsConfiguration {
        CorsConfiguration { cors_rules: self.rules }
    }
}

/// Builder for individual CORS rules.
pub struct CorsRuleBuilder {
    allow_origins: Vec<String>,
    allow_methods: Vec<HttpMethod>,
    allow_headers: Option<Vec<String>>,
    expose_headers: Option<Vec<String>>,
    max_age: Option<u64>,
}

impl CorsRuleBuilder {
    pub fn new<T: Into<String>>(allow_origins: Vec<T>, allow_methods: Vec<HttpMethod>) -> Self {
        Self {
            allow_origins: allow_origins.into_iter().map(Into::into).collect(),
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

/// Builder for S3 lifecycle rule transitions.
///
/// Configures automatic transitions of objects to different storage classes.
pub struct LifecycleRuleTransitionBuilder {
    storage_class: LifecycleStorageClass,
    transition_in_days: Option<u16>,
}

impl LifecycleRuleTransitionBuilder {
    pub fn new(storage_class: LifecycleStorageClass) -> Self {
        Self {
            storage_class,
            transition_in_days: None,
        }
    }

    pub fn transition_in_days(self, days: LifecycleTransitionInDays) -> Self {
        Self {
            transition_in_days: Some(days.0),
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

/// Builder for non-current version transitions in versioned buckets.
///
/// Configures automatic transitions for previous versions of objects.
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

#[derive(Debug, Clone)]
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

/// Builder for S3 lifecycle rules.
///
/// Defines rules for automatic object expiration and transitions between storage classes.
pub struct LifecycleRuleBuilder {
    id: Option<String>,
    status: LifecycleRuleStatus,
    expiration_in_days: Option<u16>, // expiration must be > than expiration in transition (ow boy...)
    prefix: Option<String>,
    object_size_greater_than: Option<u32>,
    object_size_less_than: Option<u32>,
    abort_incomplete_multipart_upload: Option<u16>,
    non_current_version_expiration: Option<u16>,
    transitions: Option<Vec<LifecycleRuleTransition>>,
    non_current_version_transitions: Option<Vec<NonCurrentVersionTransition>>,
}

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

    pub fn id<T: Into<String>>(self, id: T) -> Self {
        Self {
            id: Some(id.into()),
            ..self
        }
    }

    pub fn expiration_in_days(self, days: u16) -> Self {
        Self {
            expiration_in_days: Some(days),
            ..self
        }
    }

    pub fn prefix<T: Into<String>>(self, prefix: T) -> Self {
        Self {
            prefix: Some(prefix.into()),
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

/// Builder for S3 lifecycle configuration.
///
/// Combines multiple lifecycle rules into a configuration for a bucket.
pub struct LifecycleConfigurationBuilder {
    rules: Vec<LifecycleRule>,
    transition_minimum_size: Option<TransitionDefaultMinimumObjectSize>,
}

impl Default for LifecycleConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
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

/// Builder for S3 public access block configuration.
///
/// Controls public access to the bucket at the bucket level.
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
            block_public_acls: self.block_public_acls,
            block_public_policy: self.block_public_policy,
            ignore_public_acls: self.ignore_public_acls,
            restrict_public_buckets: self.restrict_public_buckets,
        }
    }
}

pub enum IntelligentTieringStatus {
    Enabled,
    Disabled,
}

impl From<IntelligentTieringStatus> for String {
    fn from(value: IntelligentTieringStatus) -> String {
        match value {
            IntelligentTieringStatus::Enabled => "Enabled".to_string(),
            IntelligentTieringStatus::Disabled => "Disabled".to_string(),
        }
    }
}

pub struct TagFilterBuilder {
    key: String,
    value: String,
}

impl TagFilterBuilder {
    pub fn new<T: Into<String>>(key: T, value: T) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn build(self) -> TagFilter {
        TagFilter {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct IntelligentTieringConfigurationBuilder {
    id: String,
    status: String,
    prefix: Option<String>,
    tag_filters: Option<Vec<TagFilter>>,
    tierings: Vec<Tiering>,
}

impl IntelligentTieringConfigurationBuilder {
    pub fn new(id: &str, status: IntelligentTieringStatus, tierings: Vec<BucketTiering>) -> Self {
        IntelligentTieringConfigurationBuilder {
            id: id.to_string(),
            status: status.into(),
            prefix: None,
            tag_filters: None,
            tierings: tierings
                .into_iter()
                .map(|t| Tiering {
                    access_tier: t.0,
                    days: t.1,
                })
                .collect(),
        }
    }

    pub fn prefix<T: Into<String>>(self, prefix: T) -> Self {
        Self {
            prefix: Some(prefix.into()),
            ..self
        }
    }

    pub fn add_tag_filter(mut self, tag_filter: TagFilter) -> Self {
        if let Some(mut filters) = self.tag_filters {
            filters.push(tag_filter);
            self.tag_filters = Some(filters);
        } else {
            self.tag_filters = Some(vec![tag_filter]);
        }

        self
    }

    pub fn build(self) -> IntelligentTieringConfiguration {
        IntelligentTieringConfiguration {
            id: self.id,
            prefix: self.prefix,
            status: self.status,
            tag_filters: self.tag_filters,
            tierings: self.tierings,
        }
    }
}

pub enum TableBucketType {
    Aws,
    Customer,
}

impl From<TableBucketType> for String {
    fn from(value: TableBucketType) -> String {
        match value {
            TableBucketType::Aws => "aws".to_string(),
            TableBucketType::Customer => "customer".to_string(),
        }
    }
}

pub enum Expiration {
    Enabled,
    Disabled,
}

impl From<Expiration> for String {
    fn from(value: Expiration) -> String {
        match value {
            Expiration::Enabled => "ENABLED".to_string(),
            Expiration::Disabled => "DISABLED".to_string(),
        }
    }
}

pub struct RecordExpirationBuilder {
    days: Option<u32>,
    expiration: String,
}

impl RecordExpirationBuilder {
    pub fn new(expiration: Expiration) -> Self {
        Self {
            expiration: expiration.into(),
            days: None,
        }
    }

    pub fn days(self, days: RecordExpirationDays) -> Self {
        Self {
            days: Some(days.0),
            ..self
        }
    }

    pub fn build(self) -> RecordExpiration {
        RecordExpiration {
            days: self.days,
            expiration: self.expiration,
        }
    }
}

pub struct JournalTableConfigurationBuilder {
    record_expiration: RecordExpiration,
    table_arn: Option<Value>,
    table_name: Option<String>,
}

impl JournalTableConfigurationBuilder {
    pub fn new(record_expiration: RecordExpiration) -> Self {
        Self {
            record_expiration,
            table_arn: None,
            table_name: None,
        }
    }

    pub fn table_name<T: Into<String>>(self, name: T) -> Self {
        Self {
            table_name: Some(name.into()),
            ..self
        }
    }

    pub fn table_arn(self, arn: Value) -> Self {
        Self {
            table_arn: Some(arn),
            ..self
        }
    }

    pub fn build(self) -> JournalTableConfiguration {
        JournalTableConfiguration {
            record_expiration: self.record_expiration,
            table_arn: self.table_arn,
            table_name: self.table_name,
        }
    }
}

pub struct MetadataDestinationBuilder {
    table_bucket_type: String,
    table_bucket_arn: Option<Value>,
    table_namespace: Option<String>,
}

impl MetadataDestinationBuilder {
    pub fn new(table_bucket_type: TableBucketType) -> Self {
        Self {
            table_bucket_type: table_bucket_type.into(),
            table_bucket_arn: None,
            table_namespace: None,
        }
    }

    pub fn table_bucket_arn(self, table_bucket_arn: Value) -> Self {
        Self {
            table_bucket_arn: Some(table_bucket_arn),
            ..self
        }
    }

    pub fn table_namespace<T: Into<String>>(self, table_namespace: T) -> Self {
        Self {
            table_namespace: Some(table_namespace.into()),
            ..self
        }
    }

    pub fn build(self) -> MetadataDestination {
        MetadataDestination {
            table_bucket_arn: self.table_bucket_arn,
            table_bucket_type: self.table_bucket_type,
            table_namespace: self.table_namespace,
        }
    }
}

pub enum ConfigurationState {
    Enabled,
    Disabled,
}

impl From<ConfigurationState> for String {
    fn from(value: ConfigurationState) -> String {
        match value {
            ConfigurationState::Enabled => "ENABLED".to_string(),
            ConfigurationState::Disabled => "DISABLED".to_string(),
        }
    }
}

pub struct InventoryTableConfigurationBuilder {
    configuration_state: String,
    table_arn: Option<Value>,
    table_name: Option<String>,
}

impl InventoryTableConfigurationBuilder {
    pub fn new(configuration_state: ConfigurationState) -> Self {
        Self {
            configuration_state: configuration_state.into(),
            table_arn: None,
            table_name: None,
        }
    }

    pub fn table_arn(self, table_arn: Value) -> Self {
        Self {
            table_arn: Some(table_arn),
            ..self
        }
    }

    pub fn table_name<T: Into<String>>(self, table_name: T) -> Self {
        Self {
            table_name: Some(table_name.into()),
            ..self
        }
    }

    pub fn build(self) -> InventoryTableConfiguration {
        InventoryTableConfiguration {
            configuration_state: self.configuration_state,
            table_arn: self.table_arn,
            table_name: self.table_name,
        }
    }
}

pub struct MetadataConfigurationBuilder {
    destination: Option<MetadataDestination>,
    inventory_table_configuration: Option<InventoryTableConfiguration>,
    journal_table_configuration: JournalTableConfiguration,
}

impl MetadataConfigurationBuilder {
    pub fn new(journal_table_configuration: JournalTableConfiguration) -> Self {
        MetadataConfigurationBuilder {
            journal_table_configuration,
            destination: None,
            inventory_table_configuration: None,
        }
    }

    pub fn destination(self, destination: MetadataDestination) -> Self {
        Self {
            destination: Some(destination),
            ..self
        }
    }

    pub fn inventory_table_configuration(self, inventory_table_configuration: InventoryTableConfiguration) -> Self {
        Self {
            inventory_table_configuration: Some(inventory_table_configuration),
            ..self
        }
    }

    pub fn build(self) -> MetadataConfiguration {
        MetadataConfiguration {
            destination: self.destination,
            inventory_table_configuration: self.inventory_table_configuration,
            journal_table_configuration: self.journal_table_configuration,
        }
    }
}

use crate::shared::Id;
use crate::{dto_methods, ref_struct};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DBClusterType {
    #[serde(rename = "AWS::DocDB::DBCluster")]
    DBClusterType,
}

ref_struct!(DBClusterRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBCluster {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: DBClusterType,
    #[serde(rename = "Properties")]
    pub(crate) properties: DBClusterProperties,
}
dto_methods!(DBCluster);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBClusterProperties {
    #[serde(rename = "AvailabilityZones", skip_serializing_if = "Option::is_none")]
    pub(crate) availability_zones: Option<Vec<String>>,
    #[serde(rename = "ManageMasterUserPassword", skip_serializing_if = "Option::is_none")]
    pub(crate) manage_master_user_password: Option<bool>,
    #[serde(rename = "DBSubnetGroupName", skip_serializing_if = "Option::is_none")]
    pub(crate) db_subnet_group_name: Option<String>,
    #[serde(rename = "StorageEncrypted", skip_serializing_if = "Option::is_none")]
    pub(crate) storage_encrypted: Option<bool>,
    #[serde(rename = "RotateMasterUserPassword", skip_serializing_if = "Option::is_none")]
    pub(crate) rotate_master_user_password: Option<bool>,
    #[serde(rename = "RestoreToTime", skip_serializing_if = "Option::is_none")]
    pub(crate) restore_to_time: Option<String>,
    #[serde(rename = "DeletionProtection", skip_serializing_if = "Option::is_none")]
    pub(crate) deletion_protection: Option<bool>,
    #[serde(rename = "ServerlessV2ScalingConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "VpcSecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub(crate) vpc_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SnapshotIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) snapshot_identifier: Option<String>,
    #[serde(rename = "UseLatestRestorableTime", skip_serializing_if = "Option::is_none")]
    pub(crate) use_latest_restorable_time: Option<bool>,
    #[serde(rename = "EnableCloudwatchLogsExports", skip_serializing_if = "Option::is_none")]
    pub(crate) enable_cloudwatch_logs_exports: Option<Vec<String>>,
    #[serde(rename = "GlobalClusterIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) global_cluster_identifier: Option<Value>,
    #[serde(rename = "NetworkType", skip_serializing_if = "Option::is_none")]
    pub(crate) network_type: Option<String>,
    #[serde(rename = "PreferredBackupWindow", skip_serializing_if = "Option::is_none")]
    pub(crate) preferred_backup_window: Option<String>,
    #[serde(rename = "BackupRetentionPeriod", skip_serializing_if = "Option::is_none")]
    pub(crate) backup_retention_period: Option<u16>,
    #[serde(rename = "RestoreType", skip_serializing_if = "Option::is_none")]
    pub(crate) restore_type: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId", skip_serializing_if = "Option::is_none")]
    pub(crate) master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUserPassword", skip_serializing_if = "Option::is_none")]
    pub(crate) master_user_password: Option<String>,
    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub(crate) port: Option<u16>,
    #[serde(rename = "StorageType", skip_serializing_if = "Option::is_none")]
    pub(crate) storage_type: Option<String>,
    #[serde(rename = "MasterUsername", skip_serializing_if = "Option::is_none")]
    pub(crate) master_username: Option<String>,
    #[serde(rename = "EngineVersion", skip_serializing_if = "Option::is_none")]
    pub(crate) engine_version: Option<String>,
    #[serde(rename = "KmsKeyId", skip_serializing_if = "Option::is_none")]
    pub(crate) kms_key_id: Option<String>,
    #[serde(rename = "SourceDBClusterIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) source_db_cluster_identifier: Option<Value>,
    #[serde(rename = "DBClusterIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) db_cluster_identifier: Option<Value>,
    #[serde(rename = "PreferredMaintenanceWindow", skip_serializing_if = "Option::is_none")]
    pub(crate) preferred_maintenance_window: Option<String>,
    #[serde(rename = "DBClusterParameterGroupName", skip_serializing_if = "Option::is_none")]
    pub(crate) db_cluster_parameter_group_name: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DBClusterParameterGroupType {
    #[serde(rename = "AWS::DocDB::DBClusterParameterGroup")]
    DBClusterParameterGroupType,
}

ref_struct!(DBClusterParameterGroupRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBClusterParameterGroup {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: DBClusterParameterGroupType,
    #[serde(rename = "Properties")]
    pub(crate) properties: DBClusterParameterGroupProperties,
}
dto_methods!(DBClusterParameterGroup);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBClusterParameterGroupProperties {
    #[serde(rename = "Description")]
    pub(crate) description: String,
    #[serde(rename = "Family")]
    pub(crate) family: String,
    #[serde(rename = "Parameters")]
    pub(crate) parameters: Value,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DBInstanceType {
    #[serde(rename = "AWS::DocDB::DBInstance")]
    DBInstanceType,
}

ref_struct!(DBInstanceRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBInstance {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: DBInstanceType,
    #[serde(rename = "Properties")]
    pub(crate) properties: DBInstanceProperties,
}
dto_methods!(DBInstance);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBInstanceProperties {
    #[serde(rename = "CertificateRotationRestart", skip_serializing_if = "Option::is_none")]
    pub(crate) certificate_rotation_restart: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    pub(crate) db_cluster_identifier: Value,
    #[serde(rename = "DBInstanceIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) db_instance_identifier: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    pub(crate) db_instance_class: String,
    #[serde(rename = "CACertificateIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) ca_certificate_identifier: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow", skip_serializing_if = "Option::is_none")]
    pub(crate) preferred_maintenance_window: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade", skip_serializing_if = "Option::is_none")]
    pub(crate) auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone", skip_serializing_if = "Option::is_none")]
    pub(crate) availability_zone: Option<String>,
    #[serde(rename = "EnablePerformanceInsights", skip_serializing_if = "Option::is_none")]
    pub(crate) enable_performance_insights: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DBSubnetGroupType {
    #[serde(rename = "AWS::DocDB::DBSubnetGroup")]
    DBSubnetGroupType,
}

ref_struct!(DBSubnetGroupRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBSubnetGroup {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: DBSubnetGroupType,
    #[serde(rename = "Properties")]
    pub(crate) properties: DBSubnetGroupProperties,
}
dto_methods!(DBSubnetGroup);

#[derive(Debug, Serialize, Deserialize)]
pub struct DBSubnetGroupProperties {
    #[serde(rename = "DBSubnetGroupName", skip_serializing_if = "Option::is_none")]
    pub(crate) db_subnet_group_name: Option<String>,
    #[serde(rename = "SubnetIds")]
    pub(crate) subnet_ids: Vec<String>,
    #[serde(rename = "DBSubnetGroupDescription")]
    pub(crate) db_subnet_group_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum EventSubscriptionType {
    #[serde(rename = "AWS::DocDB::EventSubscription")]
    EventSubscriptionType,
}

ref_struct!(EventSubscriptionRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubscription {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: EventSubscriptionType,
    #[serde(rename = "Properties")]
    pub(crate) properties: EventSubscriptionProperties,
}
dto_methods!(EventSubscription);

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubscriptionProperties {
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,
    #[serde(rename = "SnsTopicArn")]
    pub(crate) sns_topic_arn: Value,
    #[serde(rename = "EventCategories", skip_serializing_if = "Option::is_none")]
    pub(crate) event_categories: Option<Vec<String>>,
    #[serde(rename = "SourceIds", skip_serializing_if = "Option::is_none")]
    pub(crate) source_ids: Option<Vec<Value>>,
    #[serde(rename = "SourceType", skip_serializing_if = "Option::is_none")]
    pub(crate) source_type: Option<String>,
    #[serde(rename = "SubscriptionName", skip_serializing_if = "Option::is_none")]
    pub(crate) subscription_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum GlobalClusterType {
    #[serde(rename = "AWS::DocDB::GlobalCluster")]
    GlobalClusterType,
}

ref_struct!(GlobalClusterRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalCluster {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: GlobalClusterType,
    #[serde(rename = "Properties")]
    pub(crate) properties: GlobalClusterProperties,
}
dto_methods!(GlobalCluster);

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalClusterProperties {
    #[serde(rename = "Engine", skip_serializing_if = "Option::is_none")]
    pub(crate) engine: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    pub(crate) global_cluster_identifier: String,
    #[serde(rename = "SourceDBClusterIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) source_db_cluster_identifier: Option<Value>,
    #[serde(rename = "StorageEncrypted", skip_serializing_if = "Option::is_none")]
    pub(crate) storage_encrypted: Option<bool>,
    #[serde(rename = "DeletionProtection", skip_serializing_if = "Option::is_none")]
    pub(crate) deletion_protection: Option<bool>,
    #[serde(rename = "EngineVersion", skip_serializing_if = "Option::is_none")]
    pub(crate) engine_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerlessV2ScalingConfiguration {
    #[serde(rename = "MaxCapacity")]
    pub(crate) max_capacity: f32,
    #[serde(rename = "MinCapacity")]
    pub(crate) min_capacity: f32,
}

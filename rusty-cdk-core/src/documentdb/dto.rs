use serde::{Deserialize, Serialize};
use crate::shared::Id;
use serde_json::Value;
use crate::{dto_methods,ref_struct};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DocumentDBType {
    #[serde(rename = "AWS::DocDB::DBCluster")]
    DocumentDBType
}

ref_struct!(DocumentDBRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDB {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: DocumentDBType,
    #[serde(rename = "Properties")]
    pub(super) properties: DocumentDBProperties,
}
dto_methods!(DocumentDB);

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentDBProperties {
    #[serde(rename = "BackupRetentionPeriod", skip_serializing_if = "Option::is_none")]
    pub(crate) backup_retention_period: Option<u8>, // 1 - 35
    #[serde(rename = "DBClusterIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) cluster_identifier: Option<String>, // 1-63 numbers, hyphens, starts with number, no two consecutive hyphens
    #[serde(rename = "DBClusterParameterGroupName", skip_serializing_if = "Option::is_none")]
    pub(crate) cluster_param_group_name: Option<Value>,
    #[serde(rename = "DeletionProtection", skip_serializing_if = "Option::is_none")]
    pub(crate) deletion_protection: Option<bool>,
    #[serde(rename = "CopyTagsToSnapshot", skip_serializing_if = "Option::is_none")]
    pub(crate) copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "KmsKeyId", skip_serializing_if = "Option::is_none")]
    pub(crate) kms_key_id: Option<Value>,
    #[serde(rename = "ManageMasterUserPassword", skip_serializing_if = "Option::is_none")]
    pub(crate) manage_master_user_password: Option<bool>, // can't combine with master user password
    #[serde(rename = "MasterUsername", skip_serializing_if = "Option::is_none")]
    pub(crate) master_user: Option<String>,
    #[serde(rename = "MasterUserPassword", skip_serializing_if = "Option::is_none")]
    pub(crate) master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId", skip_serializing_if = "Option::is_none")]
    pub(crate) master_user_secret_kms_key_id: Option<Value>,
    #[serde(rename = "NetworkType", skip_serializing_if = "Option::is_none")]
    pub(crate) network_type: Option<String>, // IPV4 | DUAL
    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub(crate) port: Option<u16>, // max port number?
    #[serde(rename = "RestoreToTime", skip_serializing_if = "Option::is_none")]
    pub(crate) retore_to_time: Option<String>, // 2015-03-07T23:45:00Z example (UTC); Cannot be specified if the UseLatestRestorableTime parameter is true; Must be specified if the UseLatestRestorableTime parameter is not provided; Cannot be specified if the RestoreType parameter is copy-on-write.
    #[serde(rename = "RestoreType", skip_serializing_if = "Option::is_none")]
    pub(crate) restore_type: Option<String>, // full-copy copy-on-write
    #[serde(rename = "RotateMasterUserPassword", skip_serializing_if = "Option::is_none")]
    pub(crate) rotate_master_user_pass: Option<bool>, 
    #[serde(rename = "ServerlessV2ScalingConfiguration", skip_serializing_if = "Option::is_none")]
    pub(crate) serverless_scaling_config: Option<String>, 
    #[serde(rename = "SnapshotIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) snapshot_identifier: Option<String>, // should match id of exsiting snapshot...
    #[serde(rename = "SourceDBClusterIdentifier", skip_serializing_if = "Option::is_none")]
    pub(crate) source_db_cluster_identifier: Option<String>, // must be existing cluster
    #[serde(rename = "StorageEncrypted", skip_serializing_if = "Option::is_none")]
    pub(crate) storage_encrypted: Option<bool>, // if true, and source is unencrypted, should have kms key. source encrypted? this should be encrypted
    #[serde(rename = "StorageType", skip_serializing_if = "Option::is_none")]
    pub(crate) storage_type: Option<String>, // standard | iopt1
    #[serde(rename = "UseLatestRestorableTime", skip_serializing_if = "Option::is_none")]
    pub(crate) use_latest_restorable_time: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub(crate) vpc_security_group_ids: Option<Vec<String>>, // need security groups...
    
    // PreferredBackupWindow -> UTC, hh24:mi-hh24:mi, != maintenance window, at least 30 min
    // PreferredMaintenanceWindow -> UTC, ddd:hh24:mi-ddd:hh24:mi, 30m
    // GlobalClusterIdentifier
    // EngineVersion
    // DBSubnetGroupName
    // #[serde(rename = "AvailabilityZones", skip_serializing_if = "Option::is_none")]
    // pub(crate) availability_zones: Option<Vec<String>>,
    // EnableCloudwatchLogsExports
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerlessV2ScalingConfiguration {
    #[serde(rename = "MaxCapacity")]
    pub(crate) max_capacity: f32, // half increments (32, 32.5 etc.)
    #[serde(rename = "MinCapacity")]
    pub(crate) min_capacity: f32, // same
}
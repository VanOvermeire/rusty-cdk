use std::marker::PhantomData;

use crate::documentdb::{
    DBCluster, DBClusterParameterGroup, DBClusterParameterGroupProperties, DBClusterParameterGroupRef, DBClusterParameterGroupType,
    DBClusterProperties, DBClusterRef, DBClusterType, DBInstance, DBInstanceProperties, DBInstanceRef, DBInstanceType, DBSubnetGroup,
    DBSubnetGroupProperties, DBSubnetGroupRef, DBSubnetGroupType, EventSubscription, EventSubscriptionProperties, EventSubscriptionRef,
    EventSubscriptionType, GlobalCluster, GlobalClusterProperties, GlobalClusterRef, GlobalClusterType,
};
use crate::documentdb::{ServerlessV2ScalingConfiguration};
use crate::kms::KeyRef;
use crate::shared::{AvailabilityZone, Id, Region};
use crate::sns::TopicRef;
use crate::stack::{Resource, StackBuilder};
use crate::type_state;
use crate::wrappers::{DocDBSubnetGroupName, DocDBSubscriptionName, DocDbCapacityUnits, DocDbInstanceClass, DocDbMasterPassword, DocDbMasterUsername};
use serde_json::Value;

pub enum NetworkType {
    IPV4,
    Dual,
}

impl From<NetworkType> for String {
    fn from(network_type: NetworkType) -> Self {
        match network_type {
            NetworkType::IPV4 => "IPV4".to_string(),
            NetworkType::Dual => "DUAL".to_string(),
        }
    }
}

pub enum StorageType {
    Standard,
    Iopt1,
}

impl From<StorageType> for String {
    fn from(storage_type: StorageType) -> Self {
        match storage_type {
            StorageType::Standard => "standard".to_string(),
            StorageType::Iopt1 => "iopt1".to_string(),
        }
    }
}

pub enum RestoreType {
    FullCopy,
    CopyOnWrite,
}

impl From<RestoreType> for String {
    fn from(restore_type: RestoreType) -> Self {
        match restore_type {
            RestoreType::FullCopy => "full-copy".to_string(),
            RestoreType::CopyOnWrite => "copy-on-write".to_string(),
        }
    }
}

pub enum EngineVersion {
    // V3 also exists, but is deprecated
    V4,
    V5,
}

impl From<EngineVersion> for String {
    fn from(engine_version: EngineVersion) -> Self {
        match engine_version {
            EngineVersion::V4 => "Version 4.0".to_string(),
            EngineVersion::V5 => "Version 5.0".to_string(),
        }
    }
}

pub enum CloudwatchLogExport {
    Audit,
    Profiler,
}

impl From<CloudwatchLogExport> for String {
    fn from(log_export: CloudwatchLogExport) -> Self {
        match log_export {
            CloudwatchLogExport::Audit => "audit".to_string(),
            CloudwatchLogExport::Profiler => "profiler".to_string(),
        }
    }
}

type_state!(
    DbClusterState,
    DbClusterStartState,
    DbClusterManualPasswordState,
    DbClusterAutomaticPasswordState,
);

// TODO validation for restore time, backup window, maintenance window
// TODO latest restorable time not together with restore time
pub struct DBClusterBuilder<T: DbClusterState> {
    phantom: PhantomData<T>,
    id: Id,
    availability_zones: Option<Vec<String>>,
    manage_master_user_password: Option<bool>,
    rotate_master_user_password: Option<bool>,
    master_user_secret_kms_key_id: Option<Value>,
    db_subnet_group_name: Option<String>,
    storage_encrypted: Option<bool>,
    restore_to_time: Option<String>,
    use_latest_restorable_time: Option<bool>,
    deletion_protection: Option<bool>,
    serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    vpc_security_group_ids: Option<Vec<String>>,
    snapshot_identifier: Option<String>,
    enable_cloudwatch_logs_exports: Option<Vec<String>>,
    global_cluster_identifier: Option<Value>,
    network_type: Option<String>,
    backup_retention_period: Option<u16>,
    restore_type: Option<String>,
    master_username: Option<String>,
    master_user_password: Option<String>,
    port: Option<u16>,
    storage_type: Option<String>,
    engine_version: Option<String>,
    kms_key_id: Option<Value>,
    source_db_cluster_identifier: Option<Value>,
    db_cluster_identifier: Option<Value>,
    preferred_backup_window: Option<String>,
    preferred_maintenance_window: Option<String>,
    db_cluster_parameter_group_name: Option<Value>,
}

impl DBClusterBuilder<DbClusterStartState> {
    pub fn new(id: &str) -> Self {
        DBClusterBuilder {
            phantom: Default::default(),
            id: Id(id.to_string()),
            availability_zones: None,
            manage_master_user_password: None,
            db_subnet_group_name: None,
            storage_encrypted: None,
            rotate_master_user_password: None,
            restore_to_time: None,
            deletion_protection: None,
            serverless_v2_scaling_configuration: None,
            vpc_security_group_ids: None,
            snapshot_identifier: None,
            use_latest_restorable_time: None,
            enable_cloudwatch_logs_exports: None,
            global_cluster_identifier: None,
            network_type: None,
            preferred_backup_window: None,
            backup_retention_period: None,
            restore_type: None,
            master_user_secret_kms_key_id: None,
            master_user_password: None,
            port: None,
            storage_type: None,
            master_username: None,
            engine_version: None,
            kms_key_id: None,
            source_db_cluster_identifier: None,
            db_cluster_identifier: None,
            preferred_maintenance_window: None,
            db_cluster_parameter_group_name: None,
        }
    }
}

impl DBClusterBuilder<DbClusterAutomaticPasswordState> {
    pub fn manage_master_user_password(self, manage_master_user_password: bool) -> Self {
        Self {
            manage_master_user_password: Some(manage_master_user_password),
            ..self
        }
    }
    
    pub fn rotate_master_user_password(self, rotate_master_user_password: bool) -> Self {
        Self {
            rotate_master_user_password: Some(rotate_master_user_password),
            ..self
        }
    }
    
    pub fn master_user_secret_kms_key_id(self, master_user_secret_kms_key_id: &KeyRef) -> Self {
        Self {
            master_user_secret_kms_key_id: Some(master_user_secret_kms_key_id.get_arn()),
            ..self
        }
    }
    
    pub fn build(self, stack_builder: &mut StackBuilder) -> DBClusterRef {
        self.build_internal(stack_builder)
    }
}

impl DBClusterBuilder<DbClusterManualPasswordState> {
    pub fn master_user_password(self, master_user_password: DocDbMasterPassword) -> Self {
        Self {
            master_user_password: Some(master_user_password.0),
            ..self
        }
    }
    
    pub fn build(self, stack_builder: &mut StackBuilder) -> DBClusterRef {
        self.build_internal(stack_builder)
    }
}

impl<T: DbClusterState> DBClusterBuilder<T> {
    pub fn availability_zones(self, availability_zones: Vec<String>) -> Self {
        Self {
            availability_zones: Some(availability_zones),
            ..self
        }
    }

    pub fn db_subnet_group_name(self, db_subnet_group_name: String) -> Self {
        Self {
            db_subnet_group_name: Some(db_subnet_group_name),
            ..self
        }
    }

    pub fn storage_encrypted(self, storage_encrypted: bool) -> Self {
        Self {
            storage_encrypted: Some(storage_encrypted),
            ..self
        }
    }

    pub fn restore_to_time(self, restore_to_time: String) -> Self {
        Self {
            restore_to_time: Some(restore_to_time),
            ..self
        }
    }

    pub fn deletion_protection(self, deletion_protection: bool) -> Self {
        Self {
            deletion_protection: Some(deletion_protection),
            ..self
        }
    }

    pub fn serverless_v2_scaling_configuration(self, serverless_v2_scaling_configuration: ServerlessV2ScalingConfiguration) -> Self {
        Self {
            serverless_v2_scaling_configuration: Some(serverless_v2_scaling_configuration),
            ..self
        }
    }

    pub fn vpc_security_group_ids(self, vpc_security_group_ids: Vec<String>) -> Self {
        Self {
            vpc_security_group_ids: Some(vpc_security_group_ids),
            ..self
        }
    }

    pub fn snapshot_identifier(self, snapshot_identifier: String) -> Self {
        Self {
            snapshot_identifier: Some(snapshot_identifier),
            ..self
        }
    }

    pub fn use_latest_restorable_time(self, use_latest_restorable_time: bool) -> Self {
        Self {
            use_latest_restorable_time: Some(use_latest_restorable_time),
            ..self
        }
    }

    pub fn enable_cloudwatch_logs_exports(self, enable_cloudwatch_logs_exports: Vec<CloudwatchLogExport>) -> Self {
        Self {
            enable_cloudwatch_logs_exports: Some(enable_cloudwatch_logs_exports.into_iter().map(Into::into).collect()),
            ..self
        }
    }

    pub fn global_cluster_identifier(self, global_cluster_identifier: &GlobalClusterRef) -> Self {
        Self {
            global_cluster_identifier: Some(global_cluster_identifier.get_ref()),
            ..self
        }
    }

    pub fn network_type(self, network_type: NetworkType) -> Self {
        Self {
            network_type: Some(network_type.into()),
            ..self
        }
    }

    pub fn preferred_backup_window(self, preferred_backup_window: String) -> Self {
        Self {
            preferred_backup_window: Some(preferred_backup_window),
            ..self
        }
    }

    pub fn backup_retention_period(self, backup_retention_period: u16) -> Self {
        Self {
            backup_retention_period: Some(backup_retention_period),
            ..self
        }
    }

    pub fn restore_type(self, restore_type: RestoreType) -> Self {
        Self {
            restore_type: Some(restore_type.into()),
            ..self
        }
    }

    pub fn port(self, port: u16) -> Self {
        Self { port: Some(port), ..self }
    }

    pub fn storage_type(self, storage_type: StorageType) -> Self {
        Self {
            storage_type: Some(storage_type.into()),
            ..self
        }
    }

    pub fn master_username(self, master_username: DocDbMasterUsername) -> Self {
        Self {
            master_username: Some(master_username.0),
            ..self
        }
    }

    pub fn engine_version(self, engine_version: EngineVersion) -> Self {
        Self {
            engine_version: Some(engine_version.into()),
            ..self
        }
    }

    pub fn kms_key_id(self, kms_key_id: &KeyRef) -> Self {
        // in theory, you can also pass in a key id, but ARN works for all use cases
        Self {
            kms_key_id: Some(kms_key_id.get_arn()),
            ..self
        }
    }

    pub fn source_db_cluster_identifier(self, source_db_cluster_identifier: &DBClusterRef) -> Self {
        Self {
            source_db_cluster_identifier: Some(source_db_cluster_identifier.get_ref()),
            ..self
        }
    }

    pub fn db_cluster_identifier(self, db_cluster_identifier: &DBClusterRef) -> Self {
        Self {
            db_cluster_identifier: Some(db_cluster_identifier.get_ref()),
            ..self
        }
    }

    pub fn preferred_maintenance_window(self, preferred_maintenance_window: String) -> Self {
        Self {
            preferred_maintenance_window: Some(preferred_maintenance_window),
            ..self
        }
    }

    pub fn db_cluster_parameter_group_name(self, db_cluster_parameter_group_name: &DBClusterParameterGroupRef) -> Self {
        Self {
            db_cluster_parameter_group_name: Some(db_cluster_parameter_group_name.get_ref()),
            ..self
        }
    }
    
    fn build_internal(self, stack_builder: &mut StackBuilder) -> DBClusterRef {
        let resource_id = Resource::generate_id("DBCluster");

        let resource = DBCluster {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: DBClusterType::DBClusterType,
            properties: DBClusterProperties {
                availability_zones: self.availability_zones,
                manage_master_user_password: self.manage_master_user_password,
                db_subnet_group_name: self.db_subnet_group_name,
                storage_encrypted: self.storage_encrypted,
                rotate_master_user_password: self.rotate_master_user_password,
                restore_to_time: self.restore_to_time,
                deletion_protection: self.deletion_protection,
                serverless_v2_scaling_configuration: self.serverless_v2_scaling_configuration,
                vpc_security_group_ids: self.vpc_security_group_ids,
                snapshot_identifier: self.snapshot_identifier,
                use_latest_restorable_time: self.use_latest_restorable_time,
                enable_cloudwatch_logs_exports: self.enable_cloudwatch_logs_exports,
                global_cluster_identifier: self.global_cluster_identifier,
                network_type: self.network_type,
                preferred_backup_window: self.preferred_backup_window,
                backup_retention_period: self.backup_retention_period,
                restore_type: self.restore_type,
                master_user_secret_kms_key_id: self.master_user_secret_kms_key_id,
                master_user_password: self.master_user_password,
                port: self.port,
                storage_type: self.storage_type,
                master_username: self.master_username,
                engine_version: self.engine_version,
                kms_key_id: self.kms_key_id,
                source_db_cluster_identifier: self.source_db_cluster_identifier,
                db_cluster_identifier: self.db_cluster_identifier,
                preferred_maintenance_window: self.preferred_maintenance_window,
                db_cluster_parameter_group_name: self.db_cluster_parameter_group_name,
            },
        };
        stack_builder.add_resource(resource);

        DBClusterRef::internal_new(resource_id)
    }
}

pub struct DBClusterParameterGroupBuilder {
    id: Id,
    description: String,
    family: String,
    parameters: Value,
    name: Option<String>,
}

impl DBClusterParameterGroupBuilder {
    pub fn new(id: &str, description: String, family: String, parameters: Value) -> Self {
        Self {
            id: Id(id.to_string()),
            description,
            family,
            parameters,
            name: None,
        }
    }

    pub fn name(self, name: String) -> Self {
        Self { name: Some(name), ..self }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> DBClusterParameterGroupRef {
        let resource_id = Resource::generate_id("DBClusterParameterGroup");

        let resource = DBClusterParameterGroup {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: DBClusterParameterGroupType::DBClusterParameterGroupType,
            properties: DBClusterParameterGroupProperties {
                description: self.description,
                family: self.family,
                parameters: self.parameters,
                name: self.name,
            },
        };
        stack_builder.add_resource(resource);

        DBClusterParameterGroupRef::internal_new(resource_id)
    }
}

pub struct DBInstanceBuilder {
    id: Id,
    certificate_rotation_restart: Option<bool>,
    db_cluster_identifier: Value,
    // 1-63  letters, numbers, or hyphens.
    db_instance_identifier: Option<String>,
    db_instance_class: String,
    ca_certificate_identifier: Option<String>,
    // (UTC)., Format: <code class="code">ddd:hh24:mi-ddd:hh24:mi</code>, The default is a 30-minute window selected at random from an 8-hour block of time for            each AWS Region, occurring on a random day of the week., Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun, Constraints: Minimum 30-minute window.
    preferred_maintenance_window: Option<String>,
    availability_zone: Option<String>,
    enable_performance_insights: Option<bool>,
}

impl DBInstanceBuilder {
    // TODO should probably also accept global cluster ref
    pub fn new(id: &str, db_cluster_identifier: &DBClusterRef, db_instance_class: DocDbInstanceClass) -> Self {
        Self {
            id: Id(id.to_string()),
            certificate_rotation_restart: None,
            db_cluster_identifier: db_cluster_identifier.get_ref(),
            db_instance_identifier: None,
            db_instance_class: db_instance_class.0,
            ca_certificate_identifier: None,
            preferred_maintenance_window: None,
            availability_zone: None,
            enable_performance_insights: None,
        }
    }

    pub fn certificate_rotation_restart(self, certificate_rotation_restart: bool) -> Self {
        Self {
            certificate_rotation_restart: Some(certificate_rotation_restart),
            ..self
        }
    }

    pub fn db_instance_identifier(self, db_instance_identifier: String) -> Self {
        Self {
            db_instance_identifier: Some(db_instance_identifier),
            ..self
        }
    }

    pub fn ca_certificate_identifier(self, ca_certificate_identifier: String) -> Self {
        Self {
            ca_certificate_identifier: Some(ca_certificate_identifier),
            ..self
        }
    }

    pub fn preferred_maintenance_window(self, preferred_maintenance_window: String) -> Self {
        Self {
            preferred_maintenance_window: Some(preferred_maintenance_window),
            ..self
        }
    }

    pub fn availability_zone(self, availability_zone: (Region, AvailabilityZone)) -> Self {
        let region: String = availability_zone.0.into();
        let az: String = availability_zone.1.into();
        Self {
            availability_zone: Some(format!("{}{}", region, az)),
            ..self
        }
    }

    pub fn enable_performance_insights(self, enable_performance_insights: bool) -> Self {
        Self {
            enable_performance_insights: Some(enable_performance_insights),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> DBInstanceRef {
        let resource_id = Resource::generate_id("DBInstance");

        let resource = DBInstance {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: DBInstanceType::DBInstanceType,
            properties: DBInstanceProperties {
                certificate_rotation_restart: self.certificate_rotation_restart,
                db_cluster_identifier: self.db_cluster_identifier,
                db_instance_identifier: self.db_instance_identifier,
                db_instance_class: self.db_instance_class,
                ca_certificate_identifier: self.ca_certificate_identifier,
                preferred_maintenance_window: self.preferred_maintenance_window,
                availability_zone: self.availability_zone,
                enable_performance_insights: self.enable_performance_insights,
                auto_minor_version_upgrade: None, // not applicable for DocumentDB
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        DBInstanceRef::internal_new(resource_id)
    }
}

pub struct DBSubnetGroupBuilder {
    id: Id,
    db_subnet_group_name: Option<String>,
    subnet_ids: Vec<String>,
    db_subnet_group_description: String,
}

impl DBSubnetGroupBuilder {
    pub fn new(id: &str, subnet_ids: Vec<String>, db_subnet_group_description: String) -> Self {
        Self {
            id: Id(id.to_string()),
            db_subnet_group_name: None,
            subnet_ids,
            db_subnet_group_description,
        }
    }

    pub fn db_subnet_group_name(self, db_subnet_group_name: DocDBSubnetGroupName) -> Self {
        Self {
            db_subnet_group_name: Some(db_subnet_group_name.0),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> DBSubnetGroupRef {
        let resource_id = Resource::generate_id("DBSubnetGroup");

        let resource = DBSubnetGroup {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: DBSubnetGroupType::DBSubnetGroupType,
            properties: DBSubnetGroupProperties {
                db_subnet_group_name: self.db_subnet_group_name,
                subnet_ids: self.subnet_ids,
                db_subnet_group_description: self.db_subnet_group_description,
            },
        };
        stack_builder.add_resource(resource);

        DBSubnetGroupRef::internal_new(resource_id)
    }
}

pub enum InstanceCategory {
    Availability,
    ConfigChange,
    Creation,
    Deletion,
    Failure,
    Notification,
    Recovery,
    SecurityPatching,
}

impl From<InstanceCategory> for String {
    fn from(value: InstanceCategory) -> Self {
        match value {
            InstanceCategory::Availability => "availability".to_string(),
            InstanceCategory::ConfigChange => "configuration change".to_string(),
            InstanceCategory::Creation => "creation".to_string(),
            InstanceCategory::Deletion => "deletion".to_string(),
            InstanceCategory::Failure => "failure".to_string(),
            InstanceCategory::Notification => "notification".to_string(),
            InstanceCategory::Recovery => "recovery".to_string(),
            InstanceCategory::SecurityPatching => "security patching".to_string(),
        }
    }
}

pub enum ClusterCategory {
    Creation,
    Deletion,
    Failover,
    Maintenance,
    Notification
}

impl From<ClusterCategory> for String {
    fn from(value: ClusterCategory) -> Self {
        match value {
            ClusterCategory::Creation => "creation".to_string(),
            ClusterCategory::Deletion => "deletion".to_string(),
            ClusterCategory::Failover => "failover".to_string(),
            ClusterCategory::Maintenance => "maintenance".to_string(),
            ClusterCategory::Notification => "notification".to_string(),
        }
    }
}

pub enum SnapshotCategory {
    Backup
}

impl From<SnapshotCategory> for String {
    fn from(value: SnapshotCategory) -> Self {
        match value {
            SnapshotCategory::Backup => "backup".to_string(),
        }
    }
}

pub enum ParameterGroupCategory {
    ConfigChange
}

impl From<ParameterGroupCategory> for String {
    fn from(value: ParameterGroupCategory) -> Self {
        match value {
            ParameterGroupCategory::ConfigChange => "configuration change".to_string(),
        }
    }
}

// TODO add other options
pub enum SourceIdsAndType<'a> {
    Cluster(Option<Vec<&'a DBClusterRef>>, Option<Vec<ClusterCategory>>),
    Instance(Option<Vec<&'a DBInstanceRef>>, Option<Vec<InstanceCategory>>),
    // db-parameter-group, db-security-group, db-cluster-snapshot
    // SecurityGroup(String), // DBSecurityGroupName
    // ParameterGroup(String), // DBParameterGroupName
    // Snapshot(String), // DBSnapshotIdentifier
}

pub struct EventSubscriptionBuilder {
    id: Id,
    enabled: Option<bool>,
    sns_topic_arn: Value,
    event_categories: Option<Vec<String>>,
    source_ids: Option<Vec<Value>>,
    source_type: Option<String>,
    subscription_name: Option<String>,
}

impl EventSubscriptionBuilder {
    pub fn new(id: &str, sns_topic_arn: &TopicRef) -> Self {
        Self {
            id: Id(id.to_string()),
            enabled: None,
            sns_topic_arn: sns_topic_arn.get_arn(),
            event_categories: None,
            source_ids: None,
            source_type: None,
            subscription_name: None,
        }
    }

    pub fn enabled(self, enabled: bool) -> Self {
        Self {
            enabled: Some(enabled),
            ..self
        }
    }

    pub fn sources(self, sources: SourceIdsAndType) -> Self {
        let (source_type, source_ids, categories) = match sources {
            SourceIdsAndType::Cluster(refs, categories) => {
                ("db-cluster".to_string(), refs.map(|rfs| rfs.into_iter().map(|v| v.get_ref()).collect()), categories.map(|cats| cats.into_iter().map(Into::into).collect()))
            },
            SourceIdsAndType::Instance(refs, categories) => {
                ("db-instance".to_string(), refs.map(|rfs| rfs.into_iter().map(|v| v.get_ref()).collect()), categories.map(|cats| cats.into_iter().map(Into::into).collect()))
            },
        };
        
        Self {
            source_type: Some(source_type),
            source_ids: source_ids,
            event_categories: categories,
            ..self
        }
    }

    pub fn subscription_name(self, subscription_name: DocDBSubscriptionName) -> Self {
        Self {
            subscription_name: Some(subscription_name.0),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> EventSubscriptionRef {
        let resource_id = Resource::generate_id("EventSubscription");

        let resource = EventSubscription {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: EventSubscriptionType::EventSubscriptionType,
            properties: EventSubscriptionProperties {
                enabled: self.enabled,
                sns_topic_arn: self.sns_topic_arn,
                event_categories: self.event_categories,
                source_ids: self.source_ids,
                source_type: self.source_type,
                subscription_name: self.subscription_name,
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        EventSubscriptionRef::internal_new(resource_id)
    }
}

 pub enum GlobalEngine {
    Docdb,
}

impl From<GlobalEngine> for String {
    fn from(engine: GlobalEngine) -> Self {
        match engine {
            GlobalEngine::Docdb => "docdb".to_string(),
        }
    }
}

pub struct GlobalClusterBuilder {
    id: Id,
    engine: Option<String>,
    // Required: Yes, Pattern: <code class="code">^[a-zA-Z]<span>{</span>1}(?:-?[a-zA-Z0-9])<span>{</span>0,62}$</code>
    // Minimum: <code class="code">1</code>, Maximum: <code class="code">63</code>
    global_cluster_identifier: String,
    source_db_cluster_identifier: Option<Value>,
    storage_encrypted: Option<bool>,
    deletion_protection: Option<bool>,
    engine_version: Option<String>,
}

impl GlobalClusterBuilder {
    pub fn new(id: &str, global_cluster_identifier: String) -> Self {
        Self {
            id: Id(id.to_string()),
            engine: None,
            global_cluster_identifier,
            source_db_cluster_identifier: None,
            storage_encrypted: None,
            deletion_protection: None,
            engine_version: None,
        }
    }

    pub fn engine(self, engine: GlobalEngine) -> Self {
        Self {
            engine: Some(engine.into()),
            ..self
        }
    }
    
    pub fn engine_version(self, engine_version: EngineVersion) -> Self {
        Self {
            engine_version: Some(engine_version.into()),
            ..self
        }
    }

    pub fn source_db_cluster_identifier(self, source_db_cluster_identifier: &DBClusterRef) -> Self {
        Self {
            source_db_cluster_identifier: Some(source_db_cluster_identifier.get_arn()),
            ..self
        }
    }

    pub fn storage_encrypted(self, storage_encrypted: bool) -> Self {
        Self {
            storage_encrypted: Some(storage_encrypted),
            ..self
        }
    }

    pub fn deletion_protection(self, deletion_protection: bool) -> Self {
        Self {
            deletion_protection: Some(deletion_protection),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> GlobalClusterRef {
        let resource_id = Resource::generate_id("GlobalCluster");

        let resource = GlobalCluster {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: GlobalClusterType::GlobalClusterType,
            properties: GlobalClusterProperties {
                engine: self.engine,
                global_cluster_identifier: self.global_cluster_identifier,
                source_db_cluster_identifier: self.source_db_cluster_identifier,
                storage_encrypted: self.storage_encrypted,
                deletion_protection: self.deletion_protection,
                engine_version: self.engine_version,
            },
        };
        stack_builder.add_resource(resource);

        GlobalClusterRef::internal_new(resource_id)
    }
}

pub struct ServerlessV2ScalingConfigurationBuilder {
    max_capacity: f32,
    min_capacity: f32,
}

impl ServerlessV2ScalingConfigurationBuilder {
    pub fn new(max_capacity: DocDbCapacityUnits, min_capacity: DocDbCapacityUnits) -> Self {
        Self {
            min_capacity: min_capacity.0,
            max_capacity: max_capacity.0,
        }
    }

    pub fn build(self) -> ServerlessV2ScalingConfiguration {
        ServerlessV2ScalingConfiguration {
            max_capacity: self.max_capacity.floor(),
            min_capacity: self.min_capacity,
        }
    }
}

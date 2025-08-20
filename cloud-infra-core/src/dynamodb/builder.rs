use std::marker::PhantomData;
use crate::dynamodb::dto::{AttributeDefinition, DynamoDBTable, DynamoDBTableProperties, KeySchema};
use crate::dynamodb::{OnDemandThroughput, ProvisionedThroughput};
use crate::stack::Resource;
use crate::wrappers::{NonZeroNumber, StringWithOnlyAlphaNumericsAndUnderscores};

#[derive(PartialEq)]
pub enum BillingMode {
    PayPerRequest,
    Provisioned,
}

impl From<BillingMode> for String {
    fn from(value: BillingMode) -> Self {
        match value {
            BillingMode::PayPerRequest => "PAY_PER_REQUEST".to_string(),
            BillingMode::Provisioned => "PROVISIONED".to_string()
        }
    }
}

pub enum AttributeType {
    String,
    Number,
    Binary,
}

impl From<AttributeType> for String {
    fn from(value: AttributeType) -> Self {
        match value {
            AttributeType::String => "S".to_string(),
            AttributeType::Number => "N".to_string(),
            AttributeType::Binary => "B".to_string()
        }
    }
}

pub struct DynamoDBKey {
    key: String,
    key_type: AttributeType,
}

impl DynamoDBKey {
    pub fn new(key: StringWithOnlyAlphaNumericsAndUnderscores, key_type: AttributeType) -> Self {
        Self {
            key: key.0,
            key_type,
        }
    }
}

pub trait DynamoDBTableBuilderState {}

pub struct StartState {}
impl DynamoDBTableBuilderState for StartState {}

pub struct ProvisionedStateStart {}
impl DynamoDBTableBuilderState for ProvisionedStateStart {}
pub struct ProvisionedStateReadSet {}
impl DynamoDBTableBuilderState for ProvisionedStateReadSet {}
pub struct ProvisionedStateWriteSet {}
impl DynamoDBTableBuilderState for ProvisionedStateWriteSet {}

pub struct PayPerRequestState {}
impl DynamoDBTableBuilderState for PayPerRequestState {}

pub struct DynamoDBTableBuilder<T: DynamoDBTableBuilderState> {
    state: PhantomData<T>,
    table_name: Option<String>,
    partition_key: Option<DynamoDBKey>,
    sort_key: Option<DynamoDBKey>,
    billing_mode: Option<BillingMode>,
    read_capacity: Option<u32>,
    write_capacity: Option<u32>,
    max_read_capacity: Option<u32>,
    max_write_capacity: Option<u32>,
}

impl DynamoDBTableBuilder<StartState> {
    pub fn new(key: DynamoDBKey) -> Self {
        DynamoDBTableBuilder {
            state: Default::default(),
            table_name: None,
            partition_key: Some(key),
            sort_key: None,
            billing_mode: None,
            read_capacity: None,
            write_capacity: None,
            max_read_capacity: None,
            max_write_capacity: None,
        }
    }
}

impl<T: DynamoDBTableBuilderState> DynamoDBTableBuilder<T> {
    pub fn sort_key(self, key: DynamoDBKey) -> Self {
        Self {
            sort_key: Some(key),
            ..self
        }
    }

    pub fn table_name(self, name: StringWithOnlyAlphaNumericsAndUnderscores) -> Self {
        Self {
            table_name: Some(name.0),
            ..self
        }
    }

    pub fn pay_per_request_billing(self) -> DynamoDBTableBuilder<PayPerRequestState> {
        DynamoDBTableBuilder {
            billing_mode: Some(BillingMode::PayPerRequest),
            state: Default::default(),
            table_name: self.table_name,
            partition_key: self.partition_key,
            sort_key: self.sort_key,
            read_capacity: self.read_capacity,
            write_capacity: self.write_capacity,
            max_read_capacity: self.max_read_capacity,
            max_write_capacity: self.max_write_capacity,
        }
    }

    pub fn provisioned_billing(self) -> DynamoDBTableBuilder<ProvisionedStateStart> {
        DynamoDBTableBuilder {
            billing_mode: Some(BillingMode::Provisioned),
            state: Default::default(),
            table_name: self.table_name,
            partition_key: self.partition_key,
            sort_key: self.sort_key,
            read_capacity: self.read_capacity,
            write_capacity: self.write_capacity,
            max_read_capacity: self.max_read_capacity,
            max_write_capacity: self.max_write_capacity,
        }
    }

    fn build_internal(self) -> DynamoDBTable {
        let DynamoDBKey { key, key_type } = self.partition_key.unwrap();
        let mut key_schema = vec![KeySchema { attribute_name: key.clone(), key_type: "HASH".to_string() }];
        let mut key_attributes = vec![AttributeDefinition {
            attribute_name: key,
            attribute_type: key_type.into(),
        }];
        
        if let Some(DynamoDBKey { key, key_type }) = self.sort_key {
            let sort_key = KeySchema { attribute_name: key.clone(), key_type: "RANGE".to_string() };
            let sort_key_attributes = AttributeDefinition {
                attribute_name: key,
                attribute_type: key_type.into(),
            };
            key_schema.push(sort_key);
            key_attributes.push(sort_key_attributes);
        }

        let billing_mode = self.billing_mode.expect("billing mode should be set, as this is enforced by the builder");
        
        let provisioned_throughput = if billing_mode == BillingMode::Provisioned {
            Some(ProvisionedThroughput {
                read_capacity: self.read_capacity.expect("for provisioned billing mode, read capacity should be set"),
                write_capacity: self.write_capacity.expect("for provisioned billing mode, write capacity should be set"),
            })
        } else {
            None
        };
        
        let on_demand_throughput = if billing_mode == BillingMode::PayPerRequest {
            Some(OnDemandThroughput {
                max_read_capacity: self.max_read_capacity,
                max_write_capacity: self.max_write_capacity,
            })
        } else {
            None
        };

        let properties = DynamoDBTableProperties {
            key_schema,
            attribute_definitions: key_attributes,
            billing_mode: billing_mode.into(),
            provisioned_throughput,
            on_demand_throughput,
        };

        DynamoDBTable::new(Resource::generate_id("DynamoDBTable"), properties)
    }
}

impl DynamoDBTableBuilder<PayPerRequestState> {
    pub fn max_read_capacity(self, capacity: NonZeroNumber) -> Self {
        Self {
            max_read_capacity: Some(capacity.0),
            ..self
        }
    }

    pub fn max_write_capacity(self, capacity: NonZeroNumber) -> Self {
        Self {
            max_write_capacity: Some(capacity.0),
            ..self
        }
    }

    pub fn build(self) -> DynamoDBTable {
        self.build_internal()
    }
}

impl DynamoDBTableBuilder<ProvisionedStateStart> {
    pub fn read_capacity(self, capacity: NonZeroNumber) -> DynamoDBTableBuilder<ProvisionedStateReadSet> {
        DynamoDBTableBuilder {
            read_capacity: Some(capacity.0),
            state: Default::default(),
            table_name: self.table_name,
            partition_key: self.partition_key,
            sort_key: self.sort_key,
            billing_mode: self.billing_mode,
            write_capacity: self.write_capacity,
            max_read_capacity: self.read_capacity,
            max_write_capacity: self.max_write_capacity,
        }
    }
}

impl DynamoDBTableBuilder<ProvisionedStateReadSet> {
    pub fn write_capacity(self, capacity: NonZeroNumber) -> DynamoDBTableBuilder<ProvisionedStateWriteSet> {
        DynamoDBTableBuilder {
            write_capacity: Some(capacity.0),
            state: Default::default(),
            table_name: self.table_name,
            partition_key: self.partition_key,
            sort_key: self.sort_key,
            billing_mode: self.billing_mode,
            read_capacity: self.read_capacity,
            max_read_capacity: self.max_read_capacity,
            max_write_capacity: self.max_write_capacity,
        }
    }
}

impl DynamoDBTableBuilder<ProvisionedStateWriteSet> {
    pub fn build(self) -> DynamoDBTable {
        self.build_internal()
    }
}
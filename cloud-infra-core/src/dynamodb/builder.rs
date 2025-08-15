use std::marker::PhantomData;
use crate::dynamodb::dto::{AttributeDefinition, DynamoDBTable, DynamoDBTableProperties, KeySchema};
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
    STRING,
    NUMBER,
    BINARY,
}

impl From<AttributeType> for String {
    fn from(value: AttributeType) -> Self {
        match value {
            AttributeType::STRING => "S".to_string(),
            AttributeType::NUMBER => "N".to_string(),
            AttributeType::BINARY => "B".to_string()
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

pub struct ProvisionedState {}

impl DynamoDBTableBuilderState for ProvisionedState {}

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
            max_write_capacity: None,
            read_capacity: None,
            write_capacity: None,
            max_read_capacity: None,
        }
    }

    pub fn provisioned_billing(self) -> DynamoDBTableBuilder<ProvisionedState> {
        DynamoDBTableBuilder {
            billing_mode: Some(BillingMode::Provisioned),
            state: Default::default(),
            table_name: self.table_name,
            partition_key: self.partition_key,
            sort_key: self.sort_key,
            max_write_capacity: None,
            read_capacity: None,
            write_capacity: None,
            max_read_capacity: None,
        }
    }

    fn build_internal(self) -> Resource {
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
        
        let properties = DynamoDBTableProperties {
            key_schema,
            attribute_definitions: key_attributes,
            billing_mode: self.billing_mode.unwrap().into(),
            read_capacity: self.read_capacity,
            write_capacity: self.write_capacity,
            max_read_capacity: self.max_read_capacity,
            max_write_capacity: self.max_write_capacity,
        };

        Resource::DynamoDBTable(DynamoDBTable::new(Resource::generate_id("DynamoDBTable"), properties))
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

    pub fn build(self) -> Resource {
        self.build_internal()
    }
}

impl DynamoDBTableBuilder<ProvisionedState> {
    pub fn read_capacity(self, capacity: NonZeroNumber) -> Self {
        Self {
            read_capacity: Some(capacity.0),
            ..self
        }
    }

    pub fn write_capacity(self, capacity: NonZeroNumber) -> Self {
        Self {
            write_capacity: Some(capacity.0),
            ..self
        }
    }
    
    pub fn build(self) -> Resource {
        self.build_internal()
    }
}
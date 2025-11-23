use crate::dynamodb::dto::{AttributeDefinition, KeySchema, Table, TableProperties};
use crate::dynamodb::{OnDemandThroughput, ProvisionedThroughput, TableRef};
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use crate::wrappers::{NonZeroNumber, StringWithOnlyAlphaNumericsAndUnderscores};
use std::marker::PhantomData;
use crate::type_state;

#[derive(PartialEq)]
pub enum BillingMode {
    PayPerRequest,
    Provisioned,
}

impl From<BillingMode> for String {
    fn from(value: BillingMode) -> Self {
        match value {
            BillingMode::PayPerRequest => "PAY_PER_REQUEST".to_string(),
            BillingMode::Provisioned => "PROVISIONED".to_string(),
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
            AttributeType::Binary => "B".to_string(),
        }
    }
}

pub struct Key {
    key: String,
    key_type: AttributeType,
}

impl Key {
    pub fn new(key: StringWithOnlyAlphaNumericsAndUnderscores, key_type: AttributeType) -> Self {
        Self { key: key.0, key_type }
    }
}

type_state!(
    TableBuilderState,
    StartState,
    ProvisionedStateStart,
    ProvisionedStateReadSet,
    ProvisionedStateWriteSet,
    PayPerRequestState,
);

/// Builder for DynamoDB tables.
///
/// Supports both pay-per-request and provisioned billing modes. The builder enforces
/// correct configuration based on the chosen billing mode.
pub struct TableBuilder<T: TableBuilderState> {
    state: PhantomData<T>,
    id: Id,
    table_name: Option<String>,
    partition_key: Option<Key>,
    sort_key: Option<Key>,
    billing_mode: Option<BillingMode>,
    read_capacity: Option<u32>,
    write_capacity: Option<u32>,
    max_read_capacity: Option<u32>,
    max_write_capacity: Option<u32>,
}

impl TableBuilder<StartState> {
    pub fn new(id: &str, key: Key) -> Self {
        TableBuilder {
            state: Default::default(),
            id: Id(id.to_string()),
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

impl<T: TableBuilderState> TableBuilder<T> {
    pub fn sort_key(self, key: Key) -> Self {
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

    /// Configures the table to use pay-per-request billing.
    ///
    /// With this mode, you pay per request and can optionally set max throughput limits.
    pub fn pay_per_request_billing(self) -> TableBuilder<PayPerRequestState> {
        TableBuilder {
            billing_mode: Some(BillingMode::PayPerRequest),
            state: Default::default(),
            id: self.id,
            table_name: self.table_name,
            partition_key: self.partition_key,
            sort_key: self.sort_key,
            max_read_capacity: self.max_read_capacity,
            max_write_capacity: self.max_write_capacity,
            read_capacity: None,
            write_capacity: None,
        }
    }

    /// Configures the table to use provisioned billing.
    ///
    /// With this mode, you must specify read and write capacity units.
    pub fn provisioned_billing(self) -> TableBuilder<ProvisionedStateStart> {
        TableBuilder {
            billing_mode: Some(BillingMode::Provisioned),
            state: Default::default(),
            id: self.id,
            table_name: self.table_name,
            partition_key: self.partition_key,
            sort_key: self.sort_key,
            read_capacity: self.read_capacity,
            write_capacity: self.write_capacity,
            max_read_capacity: None,
            max_write_capacity: None,
        }
    }

    fn build_internal(self, stack_builder: &mut StackBuilder) -> TableRef {
        let Key { key, key_type } = self.partition_key.unwrap();
        let mut key_schema = vec![KeySchema {
            attribute_name: key.clone(),
            key_type: "HASH".to_string(),
        }];
        let mut key_attributes = vec![AttributeDefinition {
            attribute_name: key,
            attribute_type: key_type.into(),
        }];

        if let Some(Key { key, key_type }) = self.sort_key {
            let sort_key = KeySchema {
                attribute_name: key.clone(),
                key_type: "RANGE".to_string(),
            };
            let sort_key_attributes = AttributeDefinition {
                attribute_name: key,
                attribute_type: key_type.into(),
            };
            key_schema.push(sort_key);
            key_attributes.push(sort_key_attributes);
        }

        let billing_mode = self
            .billing_mode
            .expect("billing mode should be set, as this is enforced by the builder");

        let provisioned_throughput = if billing_mode == BillingMode::Provisioned {
            Some(ProvisionedThroughput {
                read_capacity: self
                    .read_capacity
                    .expect("for provisioned billing mode, read capacity should be set"),
                write_capacity: self
                    .write_capacity
                    .expect("for provisioned billing mode, write capacity should be set"),
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

        let properties = TableProperties {
            key_schema,
            attribute_definitions: key_attributes,
            billing_mode: billing_mode.into(),
            provisioned_throughput,
            on_demand_throughput,
        };

        let resource_id = Resource::generate_id("DynamoDBTable");
        stack_builder.add_resource(Table {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::DynamoDB::Table".to_string(),
            properties,
        });

        TableRef::new(resource_id)
    }
}

impl TableBuilder<PayPerRequestState> {
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

    pub fn build(self, stack_builder: &mut StackBuilder) -> TableRef {
        self.build_internal(stack_builder)
    }
}

impl TableBuilder<ProvisionedStateStart> {
    pub fn read_capacity(self, capacity: NonZeroNumber) -> TableBuilder<ProvisionedStateReadSet> {
        TableBuilder {
            read_capacity: Some(capacity.0),
            state: Default::default(),
            id: self.id,
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

impl TableBuilder<ProvisionedStateReadSet> {
    pub fn write_capacity(self, capacity: NonZeroNumber) -> TableBuilder<ProvisionedStateWriteSet> {
        TableBuilder {
            write_capacity: Some(capacity.0),
            state: Default::default(),
            id: self.id,
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

impl TableBuilder<ProvisionedStateWriteSet> {
    pub fn build(self, stack_builder: &mut StackBuilder) -> TableRef {
        self.build_internal(stack_builder)
    }
}

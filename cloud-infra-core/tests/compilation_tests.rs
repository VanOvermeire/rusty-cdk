use cloud_infra_core::dynamodb::DynamoDBKey;
use cloud_infra_core::dynamodb::DynamoDBTableBuilder;
use cloud_infra_core::dynamodb::AttributeType;
use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_macros::create_alphanumeric_underscore_string;

#[test]
fn dynamodb_builder_should_compile() {
    let key = create_alphanumeric_underscore_string!("test");
    DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String))
        .pay_per_request_billing()
        .build();
}

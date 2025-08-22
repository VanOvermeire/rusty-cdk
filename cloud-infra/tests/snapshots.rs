use serde_json::Value;
use cloud_infra_core::wrappers::NonZeroNumber;
use cloud_infra_core::dynamodb::DynamoDBKey;
use cloud_infra_core::dynamodb::DynamoDBTableBuilder;
use cloud_infra_core::dynamodb::AttributeType;
use cloud_infra_core::stack::{Resource, StackBuilder};
use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_macros::{non_zero_number, string_with_only_alpha_numerics_and_underscores};

#[test]
fn test_dynamodb() {
    let pk = string_with_only_alpha_numerics_and_underscores!("pk");
    let sk = string_with_only_alpha_numerics_and_underscores!("sk");
    let table = DynamoDBTableBuilder::new(DynamoDBKey::new(pk, AttributeType::String))
        .sort_key(DynamoDBKey::new(sk, AttributeType::Number))
        .provisioned_billing()
        .read_capacity(non_zero_number!(4))
        .write_capacity(non_zero_number!(5))
        .table_name(string_with_only_alpha_numerics_and_underscores!("table_name"))
        .build();
    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(table);
    let stack = stack_builder.build();

    let synthesized = cloud_infra::synth_stack(stack).unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();
    
    insta::assert_json_snapshot!(synthesized);
}
use cloud_infra_core::dynamodb::{AttributeType, DynamoDBKey, DynamoDBTableBuilder};
use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_macros::string_with_only_alpha_numerics_and_underscores;

fn example() {
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    DynamoDBTableBuilder::new("myTable", DynamoDBKey::new(key, AttributeType::String))
        .pay_per_request_billing()
        .read_capacity(5)
        .build();
}

fn main() {}

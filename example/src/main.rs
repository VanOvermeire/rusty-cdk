use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_core::dynamodb::{AttributeType, DynamoDBKey, DynamoDBTableBuilder};
use cloud_infra_macros::create_alphanumeric_underscore_string;

fn main() {
    let key = create_alphanumeric_underscore_string!("test");
    let table = DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::STRING)).provisioned_billing()
        .read_capacity(5) //
        // .write_capacity(1)
        .build();

    let result = cloud_infra::synth(vec![table]).unwrap();
    println!("{}", result);
}

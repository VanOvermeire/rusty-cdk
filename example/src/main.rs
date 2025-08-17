use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_core::wrappers::NonZeroNumber;
use cloud_infra_core::dynamodb::{AttributeType, DynamoDBKey, DynamoDBTableBuilder};
use cloud_infra_core::lambda::{Architecture, LambdaFunctionBuilder, Runtime, Zip};
use cloud_infra_macros::{create_alphanumeric_underscore_string, create_non_zero_number};

#[tokio::main]
async fn main() {
    // let read_capacity = create_non_zero_number!(1);
    // let write_capacity = create_non_zero_number!(1);
    // let key = create_alphanumeric_underscore_string!("test");
    // let table = DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String)).provisioned_billing()
    //     .read_capacity(read_capacity) 
    //     .write_capacity(write_capacity)
    //     .build();
    let mut all_resources = LambdaFunctionBuilder::new(Architecture::ARM64, 512, 30)
        .zip(Zip::new("test", "dir"))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .build();
    // all_resources.push(table);

    let result = cloud_infra::synth(all_resources).unwrap();
    println!("{}", result);
    
    // cloud_infra::deploy("ExampleRemove", result).await;
}

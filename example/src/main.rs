use std::path::{absolute, Path};
use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_core::wrappers::NonZeroNumber;
use cloud_infra_core::wrappers::ZipFile;
use cloud_infra_core::dynamodb::{AttributeType, DynamoDBKey, DynamoDBTableBuilder};
use cloud_infra_core::lambda::{Architecture, LambdaFunctionBuilder, Runtime, Zip};
use cloud_infra_macros::{string_with_only_alpha_numerics_and_underscores, non_zero_number, zipfile};

#[tokio::main]
async fn main() {
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table = DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String)).provisioned_billing()
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build();
    let zipper = zipfile!("./example/output/todo-backend.zip");
    let mut all_resources = LambdaFunctionBuilder::new(Architecture::ARM64, 512, 30)
        .zip(Zip::new("configuration-of-sam-van-overmeire", zipper))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .build();
    // all_resources.push(table);
    
    let result = cloud_infra::synth(all_resources).unwrap();
    println!("{}", result);
    
    cloud_infra::deploy("ExampleRemove", result).await;
}

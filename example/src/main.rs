use cloud_infra::wrappers::EnvVarKey;
use cloud_infra::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra::wrappers::NonZeroNumber;
use cloud_infra::wrappers::ZipFile;
use cloud_infra::wrappers::Memory;
use cloud_infra::wrappers::Timeout;
use cloud_infra::dynamodb::{AttributeType, DynamoDBKey, DynamoDBTableBuilder};
use cloud_infra::iam::{Permission};
use cloud_infra::lambda::{Architecture, LambdaFunctionBuilder, Runtime, Zip};
use cloud_infra::sqs::SqsQueueBuilder;
use cloud_infra::stack::StackBuilder;
use cloud_infra::{non_zero_number, string_with_only_alpha_numerics_and_underscores, zipfile, memory, timeout, env_var_key};

#[tokio::main]
async fn main() {
    let mut stack_builder = StackBuilder::new();
    
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String)).provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build();
    let zipper = zipfile!("./example/output/todo-backend.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);

    let queue = SqsQueueBuilder::new()
        .standard_queue()
        .build();
    
    let (fun, role, map) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new("configuration-of-sam-van-overmeire", zipper))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .sqs_event_source_mapping(&queue, None)
        .build();
    
    stack_builder.add_resource(fun);
    stack_builder.add_resource(role);
    stack_builder.add_resource(table);
    stack_builder.add_resource(map);
    stack_builder.add_resource(queue);
    let stack = stack_builder.build();

    if let Err(s) = stack {
        println!("{s}");
    } else {
        let result = cloud_infra::synth_stack(stack.unwrap()).unwrap(); // TODO
        println!("{}", result);
    }

    // cloud_infra::deploy("ExampleRemove", result).await;
}

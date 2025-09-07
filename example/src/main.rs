use cloud_infra::apigateway::builder::HttpApiGatewayBuilder;
use cloud_infra::dynamodb::{AttributeType, DynamoDBKey, DynamoDBTableBuilder};
use cloud_infra::iam::Permission;
use cloud_infra::lambda::{Architecture, LambdaFunctionBuilder, Runtime, Zip};
use cloud_infra::shared::http::HttpMethod;
use cloud_infra::sqs::SqsQueueBuilder;
use cloud_infra::stack::StackBuilder;
use cloud_infra::wrappers::Bucket;
use cloud_infra::wrappers::{EnvVarKey, Memory, NonZeroNumber, StringWithOnlyAlphaNumericsAndUnderscores, Timeout, ZipFile};
use cloud_infra::{bucket, env_var_key, memory, non_zero_number, string_with_only_alpha_numerics_and_underscores, timeout, zip_file, Synth};

#[tokio::main]
async fn main() {
    let stack_builder = StackBuilder::new();

    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String))
        .provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build();

    let queue = SqsQueueBuilder::new().standard_queue().build();
    let bucket = bucket!("configuration-of-sam-van-overmeire");

    let zipper = zip_file!("./example/output/todo-backend.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let (fun, role, log_group, map) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zipper))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .sqs_event_source_mapping(&queue, None)
        .build();

    let (api, stage, routes) = HttpApiGatewayBuilder::new()
        .disable_execute_api_endpoint(true)
        .add_route_lambda("/books".to_string(), HttpMethod::Get, &fun)
        .build();

    let stack = stack_builder.add_resource(fun)
        .add_resource(role)
        .add_resource(log_group)
        .add_resource(table)
        .add_resource(map)
        .add_resource(queue)
        .add_resource(api)
        .add_resource(stage)
        .add_resource_triples(routes)
        .build();

    if let Err(s) = stack {
        println!("{s}");
    } else {
        let synthesized: Synth = stack.unwrap().try_into().unwrap();
        println!("{}", synthesized);
        // cloud_infra::deploy("ExampleRemove", synthesized).await;
    }
}

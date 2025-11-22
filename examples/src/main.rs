use rusty_cdk::apigateway::builder::ApiGatewayV2Builder;
use rusty_cdk::dynamodb::{AttributeType, Key, TableBuilder};
use rusty_cdk::iam::Permission;
use rusty_cdk::lambda::{Architecture, FunctionBuilder, Runtime, Zip};
use rusty_cdk::shared::http::HttpMethod;
use rusty_cdk::sqs::QueueBuilder;
use rusty_cdk::stack::StackBuilder;
use rusty_cdk::wrappers::*;
use rusty_cdk::{bucket, env_var_key, memory, non_zero_number, string_with_only_alpha_numerics_and_underscores, timeout, zip_file};

#[tokio::main]
async fn main() {
    let mut stack_builder = StackBuilder::new();

    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = TableBuilder::new("myTable", Key::new(key, AttributeType::String))
        .provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build(&mut stack_builder);

    let queue = QueueBuilder::new("myQueue").standard_queue().build(&mut stack_builder);
    
    let bucket = bucket!("configuration-of-sam-van-overmeire");
    let zipper = zip_file!("./examples/output/todo-backend.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let (fun, _role, _log_group) = FunctionBuilder::new("myFun", Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zipper))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .sqs_event_source_mapping(&queue, None)
        // TODO remove/replace with valid TOML
        // .check_permissions_against_dependencies(TomlFile("...Cargo.toml".to_string()))
        .build(&mut stack_builder);
 
    ApiGatewayV2Builder::new("myAGW")
        .disable_execute_api_endpoint(true)
        .add_route_lambda("/books", HttpMethod::Get, &fun)
        .build(&mut stack_builder);

    let stack = stack_builder
        .add_tag("OWNER", "me")
        .build();

    if let Err(s) = stack {
        println!("{s}");
    } else {
        let synthesized = stack.unwrap().synth().unwrap();
        println!("{}", synthesized);
        // rusty_cdk::deploy("ExampleRemove", stack.unwrap()).await;
    }
}

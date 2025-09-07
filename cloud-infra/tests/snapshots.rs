use cloud_infra::wrappers::StringWithOnlyAlphaNumericsUnderscoresAndHyphens;
use cloud_infra::wrappers::Bucket;
use cloud_infra_core::dynamodb::AttributeType;
use cloud_infra_core::dynamodb::DynamoDBKey;
use cloud_infra_core::dynamodb::DynamoDBTableBuilder;
use cloud_infra_core::lambda::{Architecture, LambdaFunctionBuilder, Runtime, Zip};
use cloud_infra_core::stack::{StackBuilder};
use cloud_infra_core::wrappers::EnvVarKey;
use cloud_infra_core::wrappers::Memory;
use cloud_infra_core::wrappers::NonZeroNumber;
use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_core::wrappers::{Timeout, ZipFile};
use cloud_infra_macros::{env_var_key, memory, non_zero_number, string_with_only_alpha_numerics_and_underscores, string_with_only_alpha_numerics_underscores_and_hyphens, timeout, zip};
use serde_json::Value;
use cloud_infra::Synth;
use cloud_infra_core::apigateway::builder::{HttpApiGatewayBuilder};
use cloud_infra_core::iam::Permission;
use cloud_infra_core::shared::http::HttpMethod;
use cloud_infra_core::sns::builder::{FifoThroughputScope, SnsTopicBuilder, Subscription};
use cloud_infra_core::sqs::SqsQueueBuilder;

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
    let stack = stack_builder.build().unwrap();

    let synthesized: Synth = stack.try_into().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"DynamoDBTable[0-9]+", "[DynamoDBTable]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_lambda() {
    let mem = memory!(256);
    let timeout = timeout!(30);
    let zip_file = zip!("./cloud-infra/tests/example.zip");
    // not interested in testing the bucket macro here, so use the wrapper directly
    let bucket = Bucket("some-bucket".to_ascii_lowercase());
    
    let (fun, role, log) = LambdaFunctionBuilder::new(Architecture::ARM64, mem, timeout)
        .env_var_string(env_var_key!("STAGE"), "prod".to_string())
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .build();

    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(fun);
    stack_builder.add_resource(role);
    stack_builder.add_resource(log);
    let stack = stack_builder.build().unwrap();

    let synthesized: Synth = stack.try_into().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_sns() {
    let sns = SnsTopicBuilder::new()
        .topic_name(string_with_only_alpha_numerics_underscores_and_hyphens!("some-name"))
        .fifo()
        .fifo_throughput_scope(FifoThroughputScope::Topic)
        .content_based_deduplication(true)
        .build();

    let synthesized: Synth = vec![sns.into()].try_into().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();
    
    insta::with_settings!({filters => vec![
            (r"SnsTopic[0-9]+", "[SnsTopic]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_lambda_with_sns_subscription() {
    let zip_file = zip!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();
    
    let (fun, role, log) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .build();

    let (sns, subscriptions) = SnsTopicBuilder::new()
        .add_subscription(Subscription::Lambda(&fun))
        .build();

    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(fun);
    stack_builder.add_resource(role);
    stack_builder.add_resource(log);
    stack_builder.add_resource(sns);
    stack_builder.add_resource_tuples(subscriptions);
    let stack = stack_builder.build().unwrap();

    let synthesized: Synth = stack.try_into().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
            (r"SnsTopic[0-9]+", "[SnsTopic]"),
            (r"SnsSubscription[0-9]+", "[SnsSubscription]"),
            (r"LambdaPermission[0-9]+", "[LambdaPermission]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_lambda_with_api_gateway() {
    let zip_file = zip!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();
    
    let (fun, role, log) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .build();

    let (api, stage, routes) = HttpApiGatewayBuilder::new()
        .disable_execute_api_endpoint(true)
        .add_route_lambda("/books".to_string(), HttpMethod::Get, &fun)
        .build();

    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(fun);
    stack_builder.add_resource(role);
    stack_builder.add_resource(log);
    stack_builder.add_resource(api);
    stack_builder.add_resource(stage);
    stack_builder.add_resource_triples(routes);
    let stack = stack_builder.build().unwrap();

    let synthesized: Synth = stack.try_into().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
            (r"LambdaPermission[0-9]+", "[LambdaPermission]"),
            (r"HttpApiStage[0-9]+", "[HttpApiStage]"),
            (r"HttpApiRoute[0-9]+", "[HttpApiRoute]"),
            (r"HttpApiIntegration[0-9]+", "[HttpApiIntegration]"),
            (r"HttpApiGateway[0-9]+", "[HttpApiGateway]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_lambda_with_dynamodb() {
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String)).provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build();
    
    let zip_file = zip!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();
    
    let (fun, role, log) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .build();

    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(table);
    stack_builder.add_resource(fun);
    stack_builder.add_resource(role);
    stack_builder.add_resource(log);
    let stack = stack_builder.build().unwrap();

    let synthesized: Synth = stack.try_into().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"DynamoDBTable[0-9]+", "[DynamoDBTable]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_lambda_with_dynamodb_and_sqs() {
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String)).provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build();

    let queue = SqsQueueBuilder::new()
        .standard_queue()
        .build();

    let zip_file = zip!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, role, log, map) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .sqs_event_source_mapping(&queue, None)
        .build();

    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(fun);
    stack_builder.add_resource(role);
    stack_builder.add_resource(log);
    stack_builder.add_resource(table);
    stack_builder.add_resource(map);
    stack_builder.add_resource(queue);
    let stack = stack_builder.build().unwrap();

    let synthesized: Synth = stack.try_into().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"DynamoDBTable[0-9]+", "[DynamoDBTable]"),
            (r"SqsQueue[0-9]+", "[SqsQueue]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

fn get_bucket() -> Bucket {
    // not interested in testing the bucket macro here, so use the wrapper directly
    Bucket("some-bucket".to_ascii_lowercase())
}
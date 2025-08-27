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
use cloud_infra_macros::{env_var_key, memory, non_zero_number, string_with_only_alpha_numerics_and_underscores, timeout, zipfile};
use serde_json::Value;
use cloud_infra_core::iam::Permission;
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

    let synthesized = cloud_infra::synth_stack(stack).unwrap();
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
    let zip_file = zipfile!("./cloud-infra/tests/example.zip");
    // not interested in testing the bucket macro here, so just use the wrapper directly
    let bucket = Bucket("some-bucket".to_ascii_lowercase());
    
    let lambda = LambdaFunctionBuilder::new(Architecture::ARM64, mem, timeout)
        .env_var_string(env_var_key!("STAGE"), "prod".to_string())
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .build();

    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(lambda.0);
    stack_builder.add_resource(lambda.1);
    let stack = stack_builder.build().unwrap();

    let synthesized = cloud_infra::synth_stack(stack).unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
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
    
    let zip_file = zipfile!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    // not interested in testing the bucket macro here, so just use the wrapper directly
    let bucket = Bucket("some-bucket".to_ascii_lowercase());
    
    let (fun, role) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
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
    let stack = stack_builder.build().unwrap();

    let synthesized = cloud_infra::synth_stack(stack).unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
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

    let zip_file = zipfile!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    // not interested in testing the bucket macro here, so just use the wrapper directly
    let bucket = Bucket("some-bucket".to_ascii_lowercase());

    let (fun, role, map) = LambdaFunctionBuilder::new(Architecture::ARM64, memory, timeout)
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
    stack_builder.add_resource(table);
    stack_builder.add_resource(map);
    stack_builder.add_resource(queue);
    let stack = stack_builder.build().unwrap();

    let synthesized = cloud_infra::synth_stack(stack).unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"DynamoDBTable[0-9]+", "[DynamoDBTable]"),
            (r"SqsQueue[0-9]+", "[SqsQueue]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

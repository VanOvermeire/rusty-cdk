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

    let lambda = LambdaFunctionBuilder::new(Architecture::ARM64, mem, timeout)
        .add_env_var(env_var_key!("STAGE"), "prod".to_string())
        .zip(Zip::new("some-bucket", zip_file))
        .handler("bootstrap".to_string())
        .runtime(Runtime::ProvidedAl2023)
        .build();

    let mut stack_builder = StackBuilder::new();
    stack_builder.add_resource(lambda.0);
    stack_builder.add_resource(lambda.1);
    let stack = stack_builder.build();

    let synthesized = cloud_infra::synth_stack(stack).unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized.0).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

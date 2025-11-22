use rusty_cdk_core::apigateway::builder::ApiGatewayV2Builder;
use rusty_cdk_core::dynamodb::AttributeType;
use rusty_cdk_core::dynamodb::Key;
use rusty_cdk_core::dynamodb::TableBuilder;
use rusty_cdk_core::iam::{CustomPermission, Effect, Permission, StatementBuilder};
use rusty_cdk_core::lambda::{Architecture, FunctionBuilder, Runtime, Zip};
use rusty_cdk_core::secretsmanager::builder::{GenerateSecretStringBuilder, SecretBuilder};
use rusty_cdk_core::shared::http::HttpMethod;
use rusty_cdk_core::sns::builder::{FifoThroughputScope, TopicBuilder, SubscriptionType};
use rusty_cdk_core::sqs::QueueBuilder;
use rusty_cdk_core::stack::{StackBuilder};
use rusty_cdk_core::wrappers::*;
use rusty_cdk_macros::*;
use serde_json::{Map, Value};
use rusty_cdk_core::appconfig::builder::{ApplicationBuilder, ConfigurationProfileBuilder, DeploymentStrategyBuilder, EnvironmentBuilder, LocationUri, ReplicateTo};
use rusty_cdk_core::cloudfront::{CachePolicyBuilder, DistributionBuilder, OriginAccessControlBuilder, Cookies, DefaultCacheBehaviorBuilder, Headers, OriginAccessControlType, OriginBuilder, ParametersInCacheKeyAndForwardedToOriginBuilder, QueryString, SigningBehavior, SigningProtocol, ViewerProtocolPolicy};
use rusty_cdk_core::s3::builder::{CorsConfigurationBuilder, CorsRuleBuilder, LifecycleConfigurationBuilder, LifecycleRuleBuilder, LifecycleRuleStatus, LifecycleRuleTransitionBuilder, LifecycleStorageClass, PublicAccessBlockConfigurationBuilder, BucketBuilder, Encryption};

#[test]
fn dynamodb() {
    let mut stack_builder = StackBuilder::new();
    let pk = string_with_only_alpha_numerics_and_underscores!("pk");
    let sk = string_with_only_alpha_numerics_and_underscores!("sk");
    TableBuilder::new("table", Key::new(pk, AttributeType::String))
        .sort_key(Key::new(sk, AttributeType::Number))
        .provisioned_billing()
        .read_capacity(non_zero_number!(4))
        .write_capacity(non_zero_number!(5))
        .table_name(string_with_only_alpha_numerics_and_underscores!("table_name"))
        .build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"DynamoDBTable[0-9]+", "[DynamoDBTable]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn bucket() {
    let mut stack_builder = StackBuilder::new();
    BucketBuilder::new("bucket")
        .encryption(Encryption::S3Managed)
        .lifecycle_configuration(LifecycleConfigurationBuilder::new()
            .add_rule(LifecycleRuleBuilder::new(LifecycleRuleStatus::Enabled)
                .prefix("/prefix")
                .add_transition(LifecycleRuleTransitionBuilder::new(LifecycleStorageClass::Glacier)
                    .transition_in_days(lifecycle_transition_in_days!(30,Glacier))
                    .build()
                )
                .build())
            .build()
        )
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"S3Bucket[0-9]+", "[S3Bucket]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn website_bucket() {
    let mut stack_builder = StackBuilder::new();
    BucketBuilder::new("buck")
        .name(bucket_name!("sams-great-website"))
        .website("index.html")
        .cors_config(CorsConfigurationBuilder::new(vec![CorsRuleBuilder::new(vec!["*"], vec![HttpMethod::Get]).build()]))
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"S3Bucket[0-9]+", "[S3Bucket]"),
            (r"S3BucketPolicy[0-9]+", "[S3BucketPolicy]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn lambda() {
    let mut stack_builder = StackBuilder::new();
    let mem = memory!(256);
    let timeout = timeout!(30);
    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let bucket = get_bucket();
    FunctionBuilder::new("fun", Architecture::ARM64, mem, timeout)
        .env_var_string(env_var_key!("STAGE"), "prod")
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

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
fn sns() {
    let mut stack_builder = StackBuilder::new();
    TopicBuilder::new("topic")
        .topic_name(string_with_only_alpha_numerics_underscores_and_hyphens!("some-name"))
        .fifo()
        .fifo_throughput_scope(FifoThroughputScope::Topic)
        .content_based_deduplication(true)
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"SnsTopic[0-9]+", "[SnsTopic]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn sqs() {
    let mut stack_builder = StackBuilder::new();
    QueueBuilder::new("queue")
        .fifo_queue()
        .content_based_deduplication(true)
        .delay_seconds(delay_seconds!(30))
        .message_retention_period(message_retention_period!(600))
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"SqsQueue[0-9]+", "[SqsQueue]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn lambda_with_sns_subscription() {
    let mut stack_builder = StackBuilder::new();
    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, _role, _log) = FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build(&mut stack_builder);
    TopicBuilder::new("topic").add_subscription(SubscriptionType::Lambda(&fun)).build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

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
fn lambda_with_secret_and_custom_permissions() {
    let mut stack_builder = StackBuilder::new();
    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let action = iam_action!("secretsmanager:Get*");
    let statement = StatementBuilder::new(vec![action], Effect::Allow)
        .resources(vec!["*".into()])
        .build();
    let mut template_for_string = Map::new();
    template_for_string.insert("user".to_string(), Value::String("me".to_string()));
    let secret = SecretBuilder::new("my-secret")
        .generate_secret_string(GenerateSecretStringBuilder::new()
            .exclude_punctuation(true)
            .generate_string_key("password")
            .secret_string_template(Value::Object(template_for_string))
            .build()
        )
        .build(&mut stack_builder);
    FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("SECRET"), secret.get_ref())
        .permissions(Permission::Custom(CustomPermission::new("my-perm", statement)))
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
            (r"LambdaPermission[0-9]+", "[LambdaPermission]"),
            (r"SecretsManagerSecret[0-9]+", "[SecretsManagerSecret]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn lambda_with_api_gateway() {
    let mut stack_builder = StackBuilder::new();
    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, _role, _log) = FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build(&mut stack_builder);
    ApiGatewayV2Builder::new("AGW")
        .disable_execute_api_endpoint(true)
        .add_route_lambda("/books", HttpMethod::Get, &fun)
        .build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

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
fn lambda_with_dynamodb() {
    let mut stack_builder = StackBuilder::new();
    
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = TableBuilder::new("Dynamo", Key::new(key, AttributeType::String))
        .provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build(&mut stack_builder);

    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();
    FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

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
fn lambda_with_dynamodb_and_sqs() {
    let mut stack_builder = StackBuilder::new();
    
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = TableBuilder::new("table", Key::new(key, AttributeType::String))
        .provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build(&mut stack_builder);

    let queue = QueueBuilder::new("queue").standard_queue().build(&mut stack_builder);

    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();
    FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .sqs_event_source_mapping(&queue, None)
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

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

#[test]
fn cloudfront_with_s3_origin() {
    let mut stack_builder = StackBuilder::new();
    let oac = OriginAccessControlBuilder::new("oac", "myoac", OriginAccessControlType::S3, SigningBehavior::Always, SigningProtocol::SigV4)
        .build(&mut stack_builder);
    let bucket = BucketBuilder::new("bucket")
        .name(bucket_name!("sam-cloudfront-test"))
        .public_access_block_configuration(PublicAccessBlockConfigurationBuilder::new().block_public_acls(false).block_public_acls(false).ignore_public_acls(false).restrict_public_buckets(false).build())
        .build(&mut stack_builder);
    let params = ParametersInCacheKeyAndForwardedToOriginBuilder::new(false, Cookies::All, QueryString::All, Headers::Whitelist(vec!["authorization".to_string()]))
        .build();
    let pol = CachePolicyBuilder::new("policy", "unique-pol-name", 5, 0, 30, params)
        .build(&mut stack_builder);
    let origin = OriginBuilder::new("originId")
        .s3_origin(&bucket, &oac, None)
        .build();
    let default_cache = DefaultCacheBehaviorBuilder::new(&origin, &pol, ViewerProtocolPolicy::RedirectToHttps)
        .build();
    DistributionBuilder::new("distro", default_cache)
        .origins(vec![origin])
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"S3Bucket[0-9]+", "[S3Bucket]"),
            (r"S3BucketPolicy[0-9]+", "[S3BucketPolicy]"),
            (r"OAC[0-9]+", "[OAC]"),
            (r"CloudFrontDistribution[0-9]+", "[CloudFrontDistribution]"),
            (r"CachePolicy[0-9]+", "[CachePolicy]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn appconfig() {
    let mut stack_builder = StackBuilder::new();
    let app_config = ApplicationBuilder::new("app", app_config_name!("my-application")).build(&mut stack_builder);
    EnvironmentBuilder::new("env", app_config_name!("prod"), &app_config).build(&mut stack_builder);
    ConfigurationProfileBuilder::new("cp", app_config_name!("config-profile"), &app_config, LocationUri::Hosted).build(&mut stack_builder);
    DeploymentStrategyBuilder::new("ds", app_config_name!("instant"), deployment_duration_in_minutes!(0), growth_factor!(100), ReplicateTo::None).build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"AppConfigApp[0-9]+", "[AppConfigApp]"),
            (r"ConfigurationProfile[0-9]+", "[ConfigurationProfile]"),
            (r"DeploymentStrategy[0-9]+", "[DeploymentStrategy]"),
            (r"Environment[0-9]+", "[Environment]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

fn get_bucket() -> Bucket {
    // not interested in testing the bucket macro here, so use the wrapper directly
    // if you want safety, you should use the bucket macro instead
    Bucket("some-bucket".to_ascii_lowercase())
}

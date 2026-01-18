use rusty_cdk_core::apigateway::ApiGatewayV2Builder;
use rusty_cdk_core::appconfig::{
    ApplicationBuilder, ConfigurationProfileBuilder, DeploymentStrategyBuilder, EnvironmentBuilder, ReplicateTo,
};
use rusty_cdk_core::appsync::{AppSyncApiBuilder, AuthMode, AuthProviderBuilder, AuthType, ChannelNamespaceBuilder, EventConfigBuilder};
use rusty_cdk_core::cloudfront::{
    CachePolicyBuilder, Cookies, DefaultCacheBehaviorBuilder, DistributionBuilder, Headers, OriginAccessControlBuilder,
    OriginAccessControlType, OriginBuilder, ParametersInCacheKeyAndForwardedToOriginBuilder, QueryString, SigningBehavior, SigningProtocol,
    ViewerProtocolPolicy,
};
use rusty_cdk_core::cloudwatch::LogGroupBuilder;
use rusty_cdk_core::dynamodb::AttributeType;
use rusty_cdk_core::dynamodb::Key;
use rusty_cdk_core::dynamodb::TableBuilder;
use rusty_cdk_core::events::{FlexibleTimeWindowBuilder, JsonTarget, Mode, ScheduleBuilder, State, TargetBuilder};
use rusty_cdk_core::iam::{CustomPermission, Effect, Permission, PolicyDocumentBuilder, PrincipalBuilder, StatementBuilder};
use rusty_cdk_core::lambda::{Architecture, Code, FunctionBuilder, Runtime, Zip};
use rusty_cdk_core::s3::{
    BucketBuilder, ConfigurationState, CorsConfigurationBuilder, CorsRuleBuilder, Encryption, Expiration,
    IntelligentTieringConfigurationBuilder, IntelligentTieringStatus, InventoryTableConfigurationBuilder, JournalTableConfigurationBuilder,
    LifecycleConfigurationBuilder, LifecycleRuleBuilder, LifecycleRuleStatus, LifecycleRuleTransitionBuilder, LifecycleStorageClass,
    MetadataConfigurationBuilder, MetadataDestinationBuilder, NotificationDestination, NotificationEventType,
    PublicAccessBlockConfigurationBuilder, RecordExpirationBuilder, TableBucketType,
};
use rusty_cdk_core::secretsmanager::{GenerateSecretStringBuilder, SecretBuilder};
use rusty_cdk_core::shared::HttpMethod;
use rusty_cdk_core::shared::{DeletionPolicy, UpdateReplacePolicy};
use rusty_cdk_core::sns::{FifoThroughputScope, SubscriptionType, TopicBuilder};
use rusty_cdk_core::sqs::QueueBuilder;
use rusty_cdk_core::stack::StackBuilder;
use rusty_cdk_core::wrappers::*;
use rusty_cdk_macros::*;
use serde_json::{json, Map, Value};
use std::fs::read_to_string;

#[test]
fn dynamodb() {
    let mut stack_builder = StackBuilder::new();
    let pk = string_with_only_alphanumerics_and_underscores!("pk");
    let sk = string_with_only_alphanumerics_and_underscores!("sk");
    TableBuilder::new("table", Key::new(pk, AttributeType::String))
        .sort_key(Key::new(sk, AttributeType::Number))
        .provisioned_billing()
        .read_capacity(non_zero_number!(4))
        .write_capacity(non_zero_number!(5))
        .table_name(string_with_only_alphanumerics_and_underscores!("table_name"))
        .update_replace_and_deletion_policy(UpdateReplacePolicy::Retain, DeletionPolicy::Retain)
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
    let lifecycle_configuration = LifecycleConfigurationBuilder::new()
        .add_rule(
            LifecycleRuleBuilder::new(LifecycleRuleStatus::Enabled)
                .prefix("/prefix")
                .add_transition(
                    LifecycleRuleTransitionBuilder::new(LifecycleStorageClass::Glacier)
                        .transition_in_days(lifecycle_transition_in_days!(30, "Glacier"))
                        .build(),
                )
                .build(),
        )
        .build();
    BucketBuilder::new("bucket")
        .update_replace_and_deletion_policy(UpdateReplacePolicy::Retain, DeletionPolicy::Retain)
        .encryption(Encryption::S3Managed)
        .lifecycle_configuration(lifecycle_configuration)
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
    let cors_configuration = CorsConfigurationBuilder::new(vec![CorsRuleBuilder::new(vec!["*"], vec![HttpMethod::Get]).build()]).build();
    BucketBuilder::new("buck")
        .name(bucket_name!("sams-great-website"))
        .website("index.html")
        .cors_config(cors_configuration)
        .custom_bucket_policy_statements(vec![
            StatementBuilder::new(vec![iam_action!("s3:Put*")], Effect::Allow)
                .resources(vec!["*".into()])
                .build(),
        ])
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
        .code(Code::Zip(Zip::new(bucket, zip_file)))
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
fn lambda_with_custom_log_group() {
    let mut stack_builder = StackBuilder::new();

    let log_group = LogGroupBuilder::new("funLogGroup")
        .log_group_name_string(log_group_name!("custom-name"))
        .log_group_retention(log_retention!(90))
        .build(&mut stack_builder);

    let mem = memory!(256);
    let timeout = timeout!(30);
    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let bucket = get_bucket();
    FunctionBuilder::new("fun", Architecture::ARM64, mem, timeout)
        .env_var_string(env_var_key!("STAGE"), "prod")
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .log_group(&log_group)
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
        .topic_name(string_with_only_alphanumerics_underscores_and_hyphens!("some-name"))
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
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build(&mut stack_builder);
    TopicBuilder::new("topic")
        .add_subscription(SubscriptionType::Lambda(&fun))
        .build(&mut stack_builder);
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
        .generate_secret_string(
            GenerateSecretStringBuilder::new()
                .exclude_punctuation(true)
                .generate_string_key("password")
                .secret_string_template(Value::Object(template_for_string))
                .build(),
        )
        .build(&mut stack_builder);
    FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("SECRET"), secret.get_ref())
        .add_permission(Permission::Custom(CustomPermission::new(policy_name!("my-perm"), statement)))
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
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build(&mut stack_builder);
    ApiGatewayV2Builder::new("AGW", "exampleGW")
        .http()
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
fn lambda_with_websocket_api_gateway() {
    let mut stack_builder = StackBuilder::new();

    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, _role, _log_group) = FunctionBuilder::new("myFun", Architecture::ARM64, memory, timeout)
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build(&mut stack_builder);

    ApiGatewayV2Builder::new("WebSocketGW", "my-websocket-api")
        .websocket("$request.body.action")
        .add_route_lambda("handle", &fun)
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
    let key = string_with_only_alphanumerics_and_underscores!("test");
    let table_name = string_with_only_alphanumerics_and_underscores!("example_remove");
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
        .add_permission(Permission::DynamoDBRead(&table))
        .code(Code::Zip(Zip::new(bucket, zip_file)))
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
fn lambda_with_custom_log_group_and_schedule() {
    let mut stack_builder = StackBuilder::new();

    let log_group = LogGroupBuilder::new("myFunLogGroup")
        .log_group_name_string(log_group_name!("my-fun-logs"))
        .log_group_retention(log_retention!(7))
        .build(&mut stack_builder);

    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();
    let (fun, _role, _log_group) = FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .log_group(&log_group)
        .build(&mut stack_builder);

    let scheduler_role_from_account = Value::String("arn:aws:iam::1234:role/ASchedulerToLambdaRole".to_string());
    let target = TargetBuilder::new_json_target(JsonTarget::Lambda(&fun), scheduler_role_from_account)
        .input(json!({ "value": "hello" }))
        .build();
    let flexible = FlexibleTimeWindowBuilder::new(Mode::Flexible(max_flexible_time_window!(2))).build();
    ScheduleBuilder::new("Schedule", target, flexible)
        .rate_schedule(schedule_rate_expression!(5, "minutes"))
        .state(State::Enabled)
        .build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
            (r"Schedule[0-9]+", "[Schedule]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn lambda_with_dynamodb_and_sqs() {
    let mut stack_builder = StackBuilder::new();

    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alphanumerics_and_underscores!("test");
    let table_name = string_with_only_alphanumerics_and_underscores!("example_remove");
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
        .add_permission(Permission::DynamoDBRead(&table))
        .code(Code::Zip(Zip::new(bucket, zip_file)))
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
    let oac = OriginAccessControlBuilder::new(
        "oac",
        "myoac",
        OriginAccessControlType::S3,
        SigningBehavior::Always,
        SigningProtocol::SigV4,
    )
    .build(&mut stack_builder);
    let bucket = BucketBuilder::new("bucket")
        .name(bucket_name!("sam-cloudfront-test"))
        .public_access_block_configuration(
            PublicAccessBlockConfigurationBuilder::new()
                .block_public_acls(false)
                .block_public_acls(false)
                .ignore_public_acls(false)
                .restrict_public_buckets(false)
                .build(),
        )
        .build(&mut stack_builder);
    let params = ParametersInCacheKeyAndForwardedToOriginBuilder::new(
        false,
        Cookies::All,
        QueryString::All,
        Headers::Whitelist(vec!["authorization".to_string()]),
    )
    .build();
    let pol = CachePolicyBuilder::new("policy", "unique-pol-name", 5, 0, 30, params).build(&mut stack_builder);
    let origin = OriginBuilder::new("originId").s3_origin(&bucket, &oac, None).build();
    let default_cache = DefaultCacheBehaviorBuilder::new(&origin, &pol, ViewerProtocolPolicy::RedirectToHttps).build();
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
    ConfigurationProfileBuilder::new("cp", app_config_name!("config-profile"), &app_config, location_uri!("hosted"))
        .build(&mut stack_builder);
    DeploymentStrategyBuilder::new(
        "ds",
        app_config_name!("instant"),
        deployment_duration_in_minutes!(0),
        growth_factor!(100),
        ReplicateTo::None,
    )
    .build(&mut stack_builder);
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

#[test]
fn appconfig_with_lambda() {
    let mut stack_builder = StackBuilder::new();
    let app_config = ApplicationBuilder::new("app", app_config_name!("my-application")).build(&mut stack_builder);
    let env = EnvironmentBuilder::new("env", app_config_name!("prod"), &app_config).build(&mut stack_builder);
    let profile = ConfigurationProfileBuilder::new("cp", app_config_name!("config-profile"), &app_config, location_uri!("hosted"))
        .build(&mut stack_builder);
    DeploymentStrategyBuilder::new(
        "ds",
        app_config_name!("instant"),
        deployment_duration_in_minutes!(0),
        growth_factor!(100),
        ReplicateTo::None,
    )
    .build(&mut stack_builder);
    let bucket = get_bucket();
    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .add_permission(Permission::AppConfigRead(&app_config, &env, &profile))
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("APPCONFIG_APPLICATION_ID"), app_config.get_ref())
        .build(&mut stack_builder);
    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"Asset[0-9]+\.zip", "[Asset]"),
            (r"AppConfigApp[0-9]+", "[AppConfigApp]"),
            (r"ConfigurationProfile[0-9]+", "[ConfigurationProfile]"),
            (r"DeploymentStrategy[0-9]+", "[DeploymentStrategy]"),
            (r"Environment[0-9]+", "[Environment]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn api_gateway_and_existing_lambdas_keeps_ids() {
    let mut stack_builder = StackBuilder::new();
    let bucket = get_bucket();
    let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let (fun, _role, _log_group) = FunctionBuilder::new("myFun", Architecture::ARM64, memory, timeout)
        .code(Code::Zip(Zip::new(bucket, zip_file)))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build(&mut stack_builder);
    ApiGatewayV2Builder::new("myAGW", "exampleGW")
        .http()
        .disable_execute_api_endpoint(true)
        .add_route_lambda("/books", HttpMethod::Get, &fun)
        .build(&mut stack_builder);
    let mut stack = stack_builder.build().unwrap();

    let existing = read_to_string("./tests/existing_stack.json").expect("example JSON to be present");
    let synthesized = stack.synth_for_existing(&existing).unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
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
fn lambda_with_inline_code() {
    let mut stack_builder = StackBuilder::new();

    let memory = memory!(512);
    let timeout = timeout!(30);
    FunctionBuilder::new("myFun", Architecture::ARM64, memory, timeout)
        .code(Code::Inline(
            "module.exports.handler = async (e) => { console.log(e) };".to_string(),
        ))
        .handler("index.handler")
        .runtime(Runtime::NodeJs22)
        .build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LambdaPermission[0-9]+", "[LambdaPermission]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn bucket_with_notifications_to_sns() {
    let mut stack_builder = StackBuilder::new();

    let topic = TopicBuilder::new("top").build(&mut stack_builder);
    BucketBuilder::new("buc")
        .add_notification(NotificationDestination::Sns(&topic, NotificationEventType::ObjectCreated))
        .build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LambdaPermission[0-9]+", "[LambdaPermission]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"SnsTopic[0-9]+", "[SnsTopic]"),
            (r"TopicPolicy[0-9]+", "[TopicPolicy]"),
            (r"S3Bucket[0-9]+", "[S3Bucket]"),
            (r"BucketNotification[0-9]+", "[BucketNotification]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn bucket_with_notifications_to_sqs() {
    let mut stack_builder = StackBuilder::new();

    let queue = QueueBuilder::new("queue").standard_queue().build(&mut stack_builder);
    BucketBuilder::new("buck")
        .add_notification(NotificationDestination::Sqs(&queue, NotificationEventType::ObjectCreatedPost))
        .build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"LambdaFunction[0-9]+", "[LambdaFunction]"),
            (r"LambdaFunctionRole[0-9]+", "[LambdaFunctionRole]"),
            (r"LambdaPermission[0-9]+", "[LambdaPermission]"),
            (r"LogGroup[0-9]+", "[LogGroup]"),
            (r"QueuePolicy[0-9]+", "[QueuePolicy]"),
            (r"SqsQueue[0-9]+", "[SqsQueue]"),
            (r"S3Bucket[0-9]+", "[S3Bucket]"),
            (r"BucketNotification[0-9]+", "[BucketNotification]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn sns_with_policy() {
    let mut stack_builder = StackBuilder::new();

    let condition = json!({
        "ArnLike": {
            "aws:SourceArn": "some-source-for-publishes"
        }
    });
    let principal = PrincipalBuilder::new().service("s3.amazonaws.com".to_string()).build();
    let statement = StatementBuilder::new(vec![iam_action!("sns:Publish")], Effect::Allow)
        .principal(principal)
        .condition(condition)
        .build();
    let doc = PolicyDocumentBuilder::new(vec![statement]).build();
    TopicBuilder::new("top").topic_policy(doc).build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"SnsTopic[0-9]+", "[SnsTopic]"),
            (r"TopicPolicy[0-9]+", "[TopicPolicy]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn sqs_with_policy() {
    let mut stack_builder = StackBuilder::new();

    let condition = json!({
        "ArnLike": {
            "aws:SourceArn": "some-source-for-publishes"
        }
    });
    let principal = PrincipalBuilder::new().service("s3.amazonaws.com".to_string()).build();
    let statement = StatementBuilder::new(
        vec![
            iam_action!("sqs:GetQueueAttributes"),
            iam_action!("sqs:GetQueueUrl"),
            iam_action!("sqs:SendMessage"),
        ],
        Effect::Allow,
    )
    .principal(principal)
    .condition(condition)
    .build();
    let doc = PolicyDocumentBuilder::new(vec![statement]).build();
    QueueBuilder::new("queue")
        .standard_queue()
        .queue_policy(doc)
        .build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"QueuePolicy[0-9]+", "[QueuePolicy]"),
            (r"SqsQueue[0-9]+", "[SqsQueue]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn app_sync_api() {
    let mut stack_builder = StackBuilder::new();

    let auth_provider = AuthProviderBuilder::new(AuthType::ApiKey).build();
    let auth_mode = AuthMode::ApiKey;
    let config = EventConfigBuilder::new(
        vec![auth_provider],
        vec![auth_mode.clone()],
        vec![auth_mode.clone()],
        vec![auth_mode.clone()],
    )
    .build();

    let api_ref = AppSyncApiBuilder::new("API", app_sync_api_name!("planning-poker-api"))
        .event_config(config)
        .build(&mut stack_builder);

    ChannelNamespaceBuilder::new("Namespace", &api_ref, channel_namespace_name!("default")).build(&mut stack_builder);

    let stack = stack_builder.build().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"AppSyncApi[0-9]+", "[AppSyncApi]"),
            (r"ChannelNamespace[0-9]+", "[ChannelNamespace]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn bucket_with_intelligent_tiering_and_metadata_table() {
    let mut stack_builder = StackBuilder::new();

    BucketBuilder::new("test-buck")
        .add_intelligent_tiering(
            IntelligentTieringConfigurationBuilder::new(
                "intelligent",
                IntelligentTieringStatus::Enabled,
                vec![bucket_tiering!("DEEP_ARCHIVE_ACCESS", 180)],
            )
            .prefix("/test")
            .build(),
        )
        .metadata_configuration(
            MetadataConfigurationBuilder::new(
                JournalTableConfigurationBuilder::new(
                    RecordExpirationBuilder::new(Expiration::Enabled)
                        .days(record_expiration_days!(30))
                        .build(),
                )
                .build(),
            )
            .destination(MetadataDestinationBuilder::new(TableBucketType::Aws).build())
            .inventory_table_configuration(InventoryTableConfigurationBuilder::new(ConfigurationState::Enabled).build())
            .build(),
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

fn get_bucket() -> Bucket {
    // not interested in testing the bucket macro here, so use the wrapper directly
    // if you want safety, you should use the bucket macro instead
    Bucket("some-bucket".to_ascii_lowercase())
}

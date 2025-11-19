use cloud_infra_core::apigateway::builder::HttpApiGatewayBuilder;
use cloud_infra_core::dynamodb::AttributeType;
use cloud_infra_core::dynamodb::DynamoDBKey;
use cloud_infra_core::dynamodb::DynamoDBTableBuilder;
use cloud_infra_core::iam::{CustomPermission, Effect, Permission, StatementBuilder};
use cloud_infra_core::lambda::{Architecture, LambdaFunctionBuilder, Runtime, Zip};
use cloud_infra_core::secretsmanager::builder::{SecretsManagerGenerateSecretStringBuilder, SecretsManagerSecretBuilder};
use cloud_infra_core::shared::http::HttpMethod;
use cloud_infra_core::sns::builder::{FifoThroughputScope, SnsTopicBuilder, Subscription};
use cloud_infra_core::sqs::SqsQueueBuilder;
use cloud_infra_core::stack::{Stack, StackBuilder};
use cloud_infra_core::wrappers::*;
use cloud_infra_macros::*;
use serde_json::{Map, Value};
use cloud_infra_core::cloudfront::{CachePolicyBuilder, CloudFrontDistributionBuilder, CloudFrontOriginAccessControlBuilder, Cookies, DefaultCacheBehaviorBuilder, Headers, OriginAccessControlType, OriginBuilder, ParametersInCacheKeyAndForwardedToOriginBuilder, QueryString, SigningBehavior, SigningProtocol, ViewerProtocolPolicy};
use cloud_infra_core::s3::builder::{CorsConfigurationBuilder, CorsRuleBuilder, LifecycleConfigurationBuilder, LifecycleRuleBuilder, LifecycleRuleStatus, LifecycleRuleTransitionBuilder, LifecycleStorageClass, PublicAccessBlockConfigurationBuilder, S3BucketBuilder, S3Encryption};

#[test]
fn test_dynamodb() {
    let pk = string_with_only_alpha_numerics_and_underscores!("pk");
    let sk = string_with_only_alpha_numerics_and_underscores!("sk");
    let table = DynamoDBTableBuilder::new("table", DynamoDBKey::new(pk, AttributeType::String))
        .sort_key(DynamoDBKey::new(sk, AttributeType::Number))
        .provisioned_billing()
        .read_capacity(non_zero_number!(4))
        .write_capacity(non_zero_number!(5))
        .table_name(string_with_only_alpha_numerics_and_underscores!("table_name"))
        .build();
    let stack_builder = StackBuilder::new().add_resource(table);
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
fn test_bucket() {
    let bucket = S3BucketBuilder::new("bucket")
        .encryption(S3Encryption::S3Managed)
        .lifecycle_configuration(LifecycleConfigurationBuilder::new()
            .add_rule(LifecycleRuleBuilder::new(LifecycleRuleStatus::Enabled)
                .prefix("/prefix")
                .add_transition(LifecycleRuleTransitionBuilder::new(LifecycleStorageClass::Glacier)
                    .transition_in_days(30)
                    .build()
                )
                .build())
            .build()
        )
        .build();
    let stack_builder = StackBuilder::new().add_resource(bucket);
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
fn test_website_bucket() {
    let bucket = S3BucketBuilder::new("buck")
        .name(bucket_name!("sams-great-website"))
        .website("index.html")
        .cors_config(CorsConfigurationBuilder::new(vec![CorsRuleBuilder::new(vec!["*"], vec![HttpMethod::Get]).build()]))
        .build();
    let stack_builder = StackBuilder::new().add_resource_tuple(bucket);
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
fn test_lambda() {
    let mem = memory!(256);
    let timeout = timeout!(30);
    let zip_file = zip_file!("./cloud-infra/tests/example.zip");
    let bucket = get_bucket();
    let (fun, role, log) = LambdaFunctionBuilder::new("fun", Architecture::ARM64, mem, timeout)
        .env_var_string(env_var_key!("STAGE"), "prod")
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build();
    let stack = StackBuilder::new()
        .add_resource(fun)
        .add_resource(role)
        .add_resource(log)
        .build()
        .unwrap();

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
fn test_sns() {
    let sns = SnsTopicBuilder::new("topic")
        .topic_name(string_with_only_alpha_numerics_underscores_and_hyphens!("some-name"))
        .fifo()
        .fifo_throughput_scope(FifoThroughputScope::Topic)
        .content_based_deduplication(true)
        .build();
    let stack: Stack = vec![sns.into()].try_into().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"SnsTopic[0-9]+", "[SnsTopic]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_sqs() {
    let sqs = SqsQueueBuilder::new("queue")
        .fifo_queue()
        .content_based_deduplication(true)
        .delay_seconds(delay_seconds!(30))
        .message_retention_period(message_retention_period!(600))
        .build();
    let stack: Stack = vec![sqs.into()].try_into().unwrap();

    let synthesized = stack.synth().unwrap();
    let synthesized: Value = serde_json::from_str(&synthesized).unwrap();

    insta::with_settings!({filters => vec![
            (r"SqsQueue[0-9]+", "[SqsQueue]"),
        ]},{
            insta::assert_json_snapshot!(synthesized);
    });
}

#[test]
fn test_lambda_with_sns_subscription() {
    let zip_file = zip_file!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, role, log) = LambdaFunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build();
    let (sns, subscriptions) = SnsTopicBuilder::new("topic").add_subscription(Subscription::Lambda(&fun)).build();
    let stack_builder = StackBuilder::new()
        .add_resource(fun)
        .add_resource(role)
        .add_resource(log)
        .add_resource(sns)
        .add_resource_tuples(subscriptions);
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
fn test_lambda_with_secret_and_custom_permissions() {
    let zip_file = zip_file!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let action = iam_action!("secretsmanager:Get*");
    let statement = StatementBuilder::new(vec![action], Effect::Allow)
        .resources(vec!["*".into()])
        .build();
    let mut template_for_string = Map::new();
    template_for_string.insert("user".to_string(), Value::String("me".to_string()));
    let secret = SecretsManagerSecretBuilder::new("my-secret")
        .generate_secret_string(SecretsManagerGenerateSecretStringBuilder::new()
            .exclude_punctuation(true)
            .generate_string_key("password")
            .secret_string_template(Value::Object(template_for_string))
            .build()
        )
        .build();
    let (fun, role, log) = LambdaFunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("SECRET"), secret.get_ref())
        .permissions(Permission::Custom(CustomPermission::new("my-perm", statement)))
        .build();
    let stack_builder = StackBuilder::new().add_resource(fun).add_resource(role).add_resource(log).add_resource(secret);
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
fn test_lambda_with_api_gateway() {
    let zip_file = zip_file!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, role, log) = LambdaFunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .build();
    let (api, stage, routes) = HttpApiGatewayBuilder::new("AGW")
        .disable_execute_api_endpoint(true)
        .add_route_lambda("/books", HttpMethod::Get, &fun)
        .build();
    let stack_builder = StackBuilder::new()
        .add_resource(fun)
        .add_resource(role)
        .add_resource(log)
        .add_resource(api)
        .add_resource(stage)
        .add_resource_triples(routes);
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
fn test_lambda_with_dynamodb() {
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = DynamoDBTableBuilder::new("Dynamo", DynamoDBKey::new(key, AttributeType::String))
        .provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build();

    let zip_file = zip_file!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, role, log) = LambdaFunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .build();

    let stack_builder = StackBuilder::new()
        .add_resource(table)
        .add_resource(fun)
        .add_resource(role)
        .add_resource(log);
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
fn test_lambda_with_dynamodb_and_sqs() {
    let read_capacity = non_zero_number!(1);
    let write_capacity = non_zero_number!(1);
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let table_name = string_with_only_alpha_numerics_and_underscores!("example_remove");
    let table = DynamoDBTableBuilder::new("table", DynamoDBKey::new(key, AttributeType::String))
        .provisioned_billing()
        .table_name(table_name)
        .read_capacity(read_capacity)
        .write_capacity(write_capacity)
        .build();

    let queue = SqsQueueBuilder::new("queue").standard_queue().build();

    let zip_file = zip_file!("./cloud-infra/tests/example.zip");
    let memory = memory!(512);
    let timeout = timeout!(30);
    let bucket = get_bucket();

    let (fun, role, log, map) = LambdaFunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
        .permissions(Permission::DynamoDBRead(&table))
        .zip(Zip::new(bucket, zip_file))
        .handler("bootstrap")
        .runtime(Runtime::ProvidedAl2023)
        .env_var(env_var_key!("TABLE_NAME"), table.get_ref())
        .sqs_event_source_mapping(&queue, None)
        .build();

    let stack = StackBuilder::new()
        .add_resource(fun)
        .add_resource(role)
        .add_resource(log)
        .add_resource(table)
        .add_resource(map)
        .add_resource(queue)
        .build()
        .unwrap();

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
    let oac = CloudFrontOriginAccessControlBuilder::new("oac", "myoac", OriginAccessControlType::S3, SigningBehavior::Always, SigningProtocol::SigV4)
        .build();
    let bucket = S3BucketBuilder::new("bucket")
        .name(bucket_name!("sam-cloudfront-test"))
        .public_access_block_configuration(PublicAccessBlockConfigurationBuilder::new().block_public_acls(false).block_public_acls(false).ignore_public_acls(false).restrict_public_buckets(false).build())
        .build();
    let params = ParametersInCacheKeyAndForwardedToOriginBuilder::new(false, Cookies::All, QueryString::All, Headers::Whitelist(vec!["authorization".to_string()]))
        .build();
    let pol = CachePolicyBuilder::new("policy", "unique-pol-name", 5, 0, 30, params)
        .build();
    let origin = OriginBuilder::new("originId")
        .s3_origin(&bucket, &oac, None)
        .build();
    let default_cache = DefaultCacheBehaviorBuilder::new(&origin, &pol, ViewerProtocolPolicy::RedirectToHttps)
        .build();
    let (cf, policies) = CloudFrontDistributionBuilder::new("distro", default_cache)
        .origins(vec![origin])
        .build();

    let stack_builder = StackBuilder::new();
    
    let stack = stack_builder
        .add_resource(bucket)
        .add_resource(cf)
        .add_resource(pol)
        .add_resources(policies)
        .add_resource(oac)
        .build()
        .unwrap();

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

fn get_bucket() -> Bucket {
    // not interested in testing the bucket macro here, so use the wrapper directly
    // if you want safety, you should use the bucket macro instead
    Bucket("some-bucket".to_ascii_lowercase())
}

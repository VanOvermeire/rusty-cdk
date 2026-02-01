# Rusty CDK

***This is not an official AWS project.***

Rather, it is an attempt to make Infrastructure as Code safer and easier to use by checking as much as possible at compile time.

Think of it as a safe wrapper around `unsafe` CloudFormation. Also see [this blog post](https://medium.com/@sam.van.overmeire/rusty-cdk-an-infrastructure-as-code-experiment-c10ed7804a2a).

## Table of Contents

- [Usage](#usage)
- [Concepts](#concepts)
- [Motivation](#motivation)
  - [Motivating Example](#motivating-example)
  - [Approach](#approach)
  - [Usage of CloudFormation](#usage-of-cloudformation)
  - [IDs are similar to the AWS CDK](#ids-are-similar-to-the-aws-cdk)
- [Supported services](#supported-services)
  - [Available builders](#available-builders)
- [FAQ](#faq)
- [TODO](#todo)

## Usage

Install using cargo:

`cargo add rusty-cdk`

Optionally, install the cargo plugin as well:

`cargo install cargo-rusty`

Now create a stack. 

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk::wrappers::*;

fn main() {
  // prepare a stack builder
  let mut stack_builder = StackBuilder::new();
  // create resource builders, and call `build` to add the resulting resources to the stack
  let stack = stack_builder.build().expect("this empty stack to build"); // create the stack
  let synthesized = stack.synth().unwrap(); // synth the template and deploy it yourself
  // or deploy it using the `deploy` function or `cargo-rusty`
}
```

You can add infrastructure (resources) to it using builders.
For example, to add a queue:

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk::sqs::QueueBuilder;
use rusty_cdk::wrappers::*;
use rusty_cdk_macros::*;

fn main() {
  let mut stack_builder = StackBuilder::new();
  // create a queue by calling its builder
  // the queue_ref can be used to reference the queue in other resource builders
  let queue_ref = QueueBuilder::new("queue")
          .fifo_queue()
          .content_based_deduplication(true)
          .delay_seconds(delay_seconds!(30))
          .message_retention_period(message_retention_period!(600))
          .build(&mut stack_builder); // add it to the stack builder
  let stack = stack_builder.build().expect("this stack to build");
  // `synth` and deploy yourself
  // or deploy with `deploy(string_with_only_alphanumerics_and_hyphens!("SomeStackName"), stack, false).await`
  // or use `cargo-rusty deploy`
}
```

See a list of all available builders below.

Once you've done that, you can synthesize the stack to get the template as a string, and print it to standard out.

```rust,compile_fail
let synthesized = stack.synth().unwrap();
println!("{}", synthesized);
```

With `cargo rusty deploy`, you can use that output to deploy your infrastructure to AWS.

Alternatively, you can also the built-in `deploy` function, which uses the stack and does the synth internally.

```rust,compile_fail
rusty_cdk::deploy(string_with_only_alphanumerics_and_hyphens!("MyStackName"), stack, false).await;
```

Or use your choice of an AWS tool (CLI, SDK, console) to deploy the synth output.
If you have Lambdas, you will have to upload the zip files to the correct bucket if you go for this route.

## Concepts 

At the core of this library are `stacks` and `resources`, two concepts from CloudFormation.

A `stack` is a collection of `resources` that you want to deploy together. Those `resources` are pieces of AWS infrastructure (a bucket, queue, database) that you want to create. Within a stack, you can easily link `resources` together. E.g. you can pass the bucket name to as a Lambda environment variable.
Or you can give that Lambda permission to send to an SNS topic.

Despite their importance, you do not interact with `resources` directly, and you only use the `stack` directly if you want to retrieve its string (JSON) representation. Instead, you use `builders`. 

The stack has a `builder`:

```rust
use rusty_cdk::stack::StackBuilder;

fn main() {
  let mut stack_builder = StackBuilder::new();
  let stack = stack_builder.build().expect("building to work");
  // ready to synth or deploy
}
```

And every supported `resource` has one too. With these `builders` you create the infrastructure you need step by step.
Once you're done, you call `build` and pass in a `StackBuilder`, at which point your `resource` is added to the stack.
If the `build` method of `builder` does not require a `StackBuilder` argument, it is not a real resource. 
It is a property that needs to be passed to a proper resource to have effect. Another clue is that only resources require an id when you call `new`.

For example, an `S3 Bucket` can have a cors configuration. You create that configuration with a `builder` that needs no arguments.
Once created, you pass the config to the bucket `builder`. When you're ready with configuring your bucket, you call `build` and are required to pass in the `StackBuilder`. This means the bucket is an actual `resource`. 

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk::dynamodb::*;
use rusty_cdk::lambda::*;
use rusty_cdk::wrappers::*;
use rusty_cdk::s3::*;
use rusty_cdk_macros::*;
use rusty_cdk::shared::HttpMethod;
use rusty_cdk::iam::{Effect,StatementBuilder};

fn main() {
  let mut stack_builder = StackBuilder::new();

  let cors_configuration = CorsConfigurationBuilder::new(vec![CorsRuleBuilder::new(vec!["*"], vec![HttpMethod::Get]).build()]).build(); // no param required
  BucketBuilder::new("buck") // a real resource requires an id
          .name(bucket_name!("sams-great-website"))
          .website("index.html")
          .cors_config(cors_configuration)
          .custom_bucket_policy_statements(vec![
            StatementBuilder::new(vec![iam_action!("s3:Put*")], Effect::Allow)
                    .resources(vec!["*".into()])
                    .build(),
          ])
          .build(&mut stack_builder); // resource is added to the stack (builder)
}
```

You can see that there are a lot of macro calls in the above code.
Those macros enforce additional rules at compile time to make sure that you don't pass in any disallowed values that could cause issues during or after deployment. For example, the `bucket_name!` macro makes sure the naming requirements of S3 are obeyed, _and_ it checks that the bucket is available for creation (bucket names have to be globally unique!). Every one of these macro calls generates a simple 'wrapper' (often a `newtype`) in the background.
In our example, the wrapper is called `BucketName`. If for some reason the macro does not work properly, you can fall back to direct use of these wrappers.
The docs of the macros will point you to the correct wrapper.

As noted, `resources` require a unique identifier, an `id`. These ids are very similar to those of the AWS CDK. They are used as a convenience to link a resource you created to the one described in the deployed CloudFormation template (stack). This also means that if you change an id, that's interpreted as a delete/create, and will throw away the existing resource to create a brand new one.

Finally, `refs`. While there is no need to interact with the `resources` directly, you do occasionally need to be able to reference them.
For example, you might have a `Lambda` that needs the name of a `DynamoDB` table that it wants to store items in.
To facilitate such interactions between `resources` (and to make it hard to make mistakes), every `resource` has a corresponding `ref` that offers
methods to retrieve things like the resource ARN.

In the below example, we create a `DynamoDB` table and get back a `ref` to that table. We use that `ref` to set the Lambda permissions, allowing it to read the table, and use `get_ref()` to get the name of the table, because that is what a `ref` in CloudFormation [would return for a DynamoDB table](https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-dynamodb-table.html#aws-resource-dynamodb-table-return-values)

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk::dynamodb::*;
use rusty_cdk::lambda::*;
use rusty_cdk::wrappers::*;
use rusty_cdk_macros::*;
use rusty_cdk::iam::{Permission};

fn main() {
  let mut stack_builder = StackBuilder::new();

  let read_capacity = non_zero_number!(1);
  let write_capacity = non_zero_number!(1);
  let key = string_with_only_alphanumerics_and_underscores!("test");
  let table_name = string_with_only_alphanumerics_and_underscores!("example_remove");
  // a table ref is returned
  let the_table_ref = TableBuilder::new("Dynamo", Key::new(key, AttributeType::String))
          .provisioned_billing()
          .table_name(table_name)
          .read_capacity(read_capacity)
          .write_capacity(write_capacity)
          .build(&mut stack_builder);

  let zip_file = zip_file!("./rusty-cdk/tests/example.zip");
  let memory = memory!(512);
  let timeout = timeout!(30);
  // not interested in testing bucket macro here, so use the wrapper directly
  // if you want more safety, you should use `bucket!`
  let bucket = Bucket("some-bucket".to_ascii_lowercase());
  FunctionBuilder::new("fun", Architecture::ARM64, memory, timeout)
          .add_permission(Permission::DynamoDBRead(&the_table_ref)) // we make sure our Lambda has permission to use the table
          .code(Code::Zip(Zip::new(bucket, zip_file)))
          .handler("bootstrap")
          .runtime(Runtime::ProvidedAl2023)
          .env_var(env_var_key!("TABLE_NAME"), the_table_ref.get_ref()) // and pass in the table name
          .build(&mut stack_builder);
}
```

If you need to get a reference to a resource outside of CloudFormation, there are macros that help you do that in a safe way as well.
For example, to use the name or ARN of a role that you create manually in your account, you can use `lookup_role_ref!`.
Alternatively, if you don't need this additional safety, you can create a `RoleRef` yourself using the `new` method.

## Motivation

Why did I create this library? (Besides the fact that there are few tools for writing IAC in Rust.)

### Motivating Example

This below CDK code is valid at compile time. I.e., it synthesizes (`cdk synth`) to a CloudFormation template.

```typescript
// imports

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
      super(scope, id, props);
  
      new Table(this, 'someId', {
          tableName: "examples!!!",
          partitionKey: {
              name: '',
              type: AttributeType.BINARY
          },
          billingMode: BillingMode.PAY_PER_REQUEST,
          maxReadRequestUnits: -1,
          maxWriteRequestUnits: 0,
      })
  }
}
```

But the code will fail at runtime, because it contains various errors:
- table names cannot contain exclamation marks
- the partition key name cannot be empty
- you cannot set `maxReadRequestUnits` when billing mode is `PAY_PER_REQUEST`
- a `maxReadRequestUnits` of `-1` does not make sense
- `maxWriteRequestUnits` is similarly not allowed in this situation, and a value of `0` is a bit special. You're not actually allowed to set this property to zero, but because this is Typescript, the value is interpreted as a falsy and ignored. Meaning the code does not fail, but the resulting stack config is not what you expected.

Fixing these errors can cost a lot of time because you'll only notice them when you're deploying the template.
That leads to a slow feedback loop, where you're constantly fixing issues and going through synth and deploy steps, waiting for AWS to tell you where the next issue might be. In other cases, everything will deploy, but it won't work as expected (see the `maxWriteRequestUnits` above).

Compare the above with the following:

```rust
use rusty_cdk::wrappers::*; // importing all wrappers simplifies larger projects
use rusty_cdk::{non_zero_number, string_with_only_alphanumerics_and_underscores};
use rusty_cdk::dynamodb::{AttributeType, Key, TableBuilder};
use rusty_cdk::stack::{Resource, Stack, StackBuilder};

fn iac() {
  let mut stack_builder = StackBuilder::new();
  
  let dynamo_key = string_with_only_alphanumerics_and_underscores!("test");
  let table_ref = TableBuilder::new("table", Key::new(dynamo_key, AttributeType::String))
            .provisioned_billing()
            .read_capacity(non_zero_number!(5))
            .write_capacity(non_zero_number!(1))
            .build(&mut stack_builder);
  
  let stack = stack_builder.build().unwrap();
  
  // ready to synth and deploy
}
```

It's about the same amount of code. But partition keys can now only contain alphanumeric characters and underscores, so we create them through a macro that validates this at compile time. And max read capacity cannot be set when you choose `provisioned_billing`. Also, adding the resources is less magical (you have to pass in the stack builder), but equally safe (you can't build a resource without passing it in).

With this kind of tooling, making mistakes becomes much harder, as some mistakes are caught at compile time and others become impossible.

The library does require you to be somewhat more explicit at times. For example, you have to pick a billing mode, as well as read and write capacity for provisioned billing. The CDK 'helps' you by setting sensible defaults (`5` in this particular case). Which can help you get up and running quickly, but is probably not what you want for any real application. Plus, the compile time guarantees should aid you just as much - if not more - in getting stuff deployed.

### Approach

This project intends to use the tools that Rust offers for ensuring infrastructure correctness at compile time.
In some cases, Rust offers help out of the box. E.g., it has multiple number types (both signed and unsigned) that aren't falsy.
In addition, macros and type state are the most important additional tools used here.
Const functions would be interesting as well, but they're too limited for the moment (e.g. I can check a `const` at compile time, but not a `let`).

But because compile time checks are sometimes impossible or more challenging, there are also some stack level checks that happen at runtime. Which is why building a stack returns a `Result` that you unwrap at your own risk.

### Usage of CloudFormation

Just like the AWS CDK, this project uses CloudFormation to actually create the AWS services you request (unlike Terraform which uses API calls).

The main advantage is that it allows me to build on the strong foundations of CloudFormation, but improving safety and ease of use by creating a facade for the infrastructure. And no need to reinvent the wheel of figuring out the dependency graph, etc.

It also has some disadvantages. One is that CloudFormation is slow, in part because it wants to be able to roll back to a stable version if something does go wrong. That's less important if we're able to make creating the infrastructure completely safe at compile time. No rollbacks = less time lost.

In time, the project might switch to using SDK calls, to try and make things faster as well as easier.

### IDs are similar to the AWS CDK

In its core idea (create a programmatic interface for CloudFormation), some terminology and usage, this project is similar to the AWS CDK.
And so, just like with the CDK, you should be careful with changing the ids you pass to the builders. 
These ids are used to identify deployed resources. As such, changing an id is a signal that the resource whose id has been 'removed', will be deleted.
Meanwhile, a new resource, with the new, changed id will be created. 

E.g. if you have a bucket with id `myBuck`, and you change the id to `myBucket`, the bucket in your account is deleted and a new empty one is created.
This can cause issues if you've chosen a name for the resource (again, e.g., a bucket), because CloudFormation want to guarantee rollbacks, meaning a resource is only deleted _after_ its replacement has been successfully created. But that creation cannot take place until the previous name has become available again.

## Supported services

Currently only a limited number of services are (partly) supported:
- Appsync
- API Gateway
- AppConfig
- CloudFront
- Cloudwatch logs
- DynamoDB
- IAM
- Lambda
- S3
- SNS
- SQS

In other words, you can create serverless applications with this library.

To be added at some point:
- DocumentDB
- CodeBuild
- CodePipeline
- Cloudwatch (-logs)
- EventBridge
- Athena
- RDS
- ECS
- Kinesis
- Step Functions
- and additional functionality from the already supported services

### Available builders

Based on `rg '^.*?(\w+Builder).*?$' -N -I -r '$1' | sort | uniq | sed -e 's/^/- /'` in `rusty-cdk-core`.

- ApiGatewayV2Builder
- AppSyncApiBuilder
- ApplicationBuilder
- AssumeRolePolicyDocumentBuilder
- AuthProviderBuilder
- BucketBuilder
- BucketNotificationBuilder
- BucketPolicyBuilder
- CachePolicyBuilder
- ChannelNamespaceBuilder
- ConfigurationProfileBuilder
- CorsConfigurationBuilder
- CorsRuleBuilder
- DefaultCacheBehaviorBuilder
- DeploymentStrategyBuilder
- DistributionBuilder
- EnvironmentBuilder
- EventConfigBuilder
- EventLogConfigBuilder
- FlexibleTimeWindowBuilder
- FunctionBuilder
- GenerateSecretStringBuilder
- IntelligentTieringConfigurationBuilder
- InventoryTableConfigurationBuilder
- JournalTableConfigurationBuilder
- LifecycleConfigurationBuilder
- LifecycleRuleBuilder
- LifecycleRuleTransitionBuilder
- LogGroupBuilder
- LoggingConfigBuilder
- MetadataConfigurationBuilder
- MetadataDestinationBuilder
- NonCurrentVersionTransitionBuilder
- OriginAccessControlBuilder
- OriginBuilder
- ParametersInCacheKeyAndForwardedToOriginBuilder
- PermissionBuilder
- PolicyBuilder
- PolicyDocumentBuilder
- PrincipalBuilder
- PublicAccessBlockConfigurationBuilder
- QueueBuilder
- QueuePolicyBuilder
- RecordExpirationBuilder
- RetryPolicyBuilder
- RoleBuilder
- RolePropertiesBuilder
- ScheduleBuilder
- SecretBuilder
- StackBuilder
- StatementBuilder
- TableBuilder
- TagFilterBuilder
- TargetBuilder
- TopicBuilder
- TopicPolicyBuilder
- ValidatorBuilder
- ViewerCertificateBuilder

## FAQ

- _"Where can I find examples of how to use this project?"_
  - Examples can be found in the `examples` dir
  - The snapshot tests in the `rusty-cdk` dir also provide some usage examples
- _"I can't find field X of resource Y. And I would like to use resource Z, which is currently not supported"_
  - Check whether it's a legacy field (like `maxTTL` in `DefaultCacheBehavior`). If so, I may not have added it, since there's a newer, recommended, alternative.
  - If it's not a legacy field, I may not have gotten around to adding it yet. I've focussed on the properties that I think are most commonly used/useful. You can always open an issue, or add it yourself.
  - The same goes for unsupported resources: open an issue or PR!
- _"How do I add tags to resources?"_
  - Currently, you can only add tags to the stack, not to individual resources. These tags are then applied when using the `deploy` method. They are not present in the CloudFormation template, because unfortunately, templates do not have a root property for tags. See an example below.
  - In theory, CloudFormation should propagate the tags to its resources, in practice it will do so in 80–90% of cases.
- _"I create a resource and my deployment failed"_
  - If you think that failure could have been avoided at compile time (or before synthesizing), please open an issue
- _"Wouldn't it be better if synth / another method was async?"_
  - Maybe? But keeping everything except for `deploy` synchronous is easiest for now.
- _"Won't this library always be behind on the latest additions/changes to AWS?"_
  - Sadly, yes. But for a long time that was the case with CloudFormation as well. And sometimes you have to wait for months or a few years before L2-3 constructs arrive in the AWS CDK.
- _"Why don't you use more borrowing in the internals of this library?"_
  - It started out with less borrowing because that's easier, less complex. And when I experimented with introducing borrowing everywhere, the performance gain was barely noticeable.
- _"Why not use regex for the macros?"_
  - I want to avoid any dependency that I don't strictly need, and _most_ validations are actually relatively simple. Still, if the project keeps growing, I might face the choice between using `regex`, or accepting a lot of additional complexity and code.

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk::sqs::QueueBuilder;

async fn tagging() {
  let mut stack_builder = StackBuilder::new();
  // add your resources
  stack_builder.add_tag("OWNER", "me").build();
  // ...
}
```

## TODO

- Additional stack build checks
  - Check duplicate ids in intelligent tiering
  - Check the app config json schema
  - ChannelNamespace name should be unique within the API
- Probably more idiomatic to implement display for the enums that have to become `String`
- Do some refactoring/splitting up of files
  - s3 builder is a good candidate for splitting up
- Allow imports of outputs
- Improve diff
  - Show whether resource has changed
- More help with IAM permissions
  - Additional checks for structure of iam policies
    - For example `resources` is not required in all cases, but in most contexts it is
- Try to replace `syn` with more something more compile-time lightweight - `facet`?
- GitHub actions
  - Testing for several platforms
  - Semver checks
  - Publishing
- Switch to uploading template to s3? helps avoid the 51 kb limit
  - Or at least offer that option
- Think about how to allow, for example, names based on Refs
  - E.g. you should be able to take a BucketRef, add a suffix (using !Sub maybe) and pass that in as the name of a log group

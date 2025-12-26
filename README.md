# Rusty CDK

***This is not an official AWS project.***

Rather, it is an attempt to make Infrastructure as Code safer and easier to use by checking as much as possible at compile time.
Think of it as a safe wrapper around `unsafe` CloudFormation. Also see [this blog post](https://medium.com/@sam.van.overmeire/rusty-cdk-an-infrastructure-as-code-experiment-c10ed7804a2a).

## Usage

Install using cargo:

`cargo add rusty-cdk`

Now create a stack, and add infrastructure to it by using builders. 

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk_core::wrappers::*;

fn main() {
  // prepare a stack builder
  let mut stack_builder = StackBuilder::new();
  // create resource builders, and call `build` to add the resulting resources to the stack
  let stack = stack_builder.build().expect("this stack to build"); // create the stack
  // now `synth` the template or use `deploy` to deploy the stack
}
```

For example, a queue:

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk_core::sqs::QueueBuilder;
use rusty_cdk_core::wrappers::*;
use rusty_cdk_macros::{delay_seconds,message_retention_period};

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
}
```

See a list of all available builders below.

Once you've done that, you can either synthesize the stack and use any AWS tool (CLI, SDK, console) to deploy it:

```rust,compile_fail
let synthesized = stack.synth().unwrap();
```

Or you can use the built-in `deploy` function:

```rust,compile_fail
rusty_cdk::deploy("MyStackName", stack).await;
```

## Motivation

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

## Approach

This project intends to use the tools that Rust offers for ensuring infrastructure correctness at compile time.
In some cases, Rust offers help out of the box. E.g., it has multiple number types (both signed and unsigned) that aren't falsy.
In addition, macros and type state are the most important additional tools used here.
Const functions would be interesting as well, but they're too limited for the moment (e.g. I can check a `const` at compile time, but not a `let`).

But because compile time checks are sometimes impossible or more challenging, there are also some stack level checks that happen at runtime. Which is why building a stack returns a `Result` that you unwrap at your own risk.

## Usage of CloudFormation

Just like the AWS CDK, this project uses CloudFormation to actually create the AWS services you request (unlike Terraform which uses API calls).

The main advantage is that it allows me to build on the strong foundations of CloudFormation, but improving safety and ease of use by creating a facade for the infrastructure. And no need to reinvent the wheel of figuring out the dependency graph, etc.

It also has some disadvantages. One is that CloudFormation is slow, in part because it wants to be able to roll back to a stable version if something does go wrong. That's less important if we're able to make creating the infrastructure completely safe at compile time. No rollbacks = less time lost.

In time, the project might switch to using SDK calls, to try and make things faster as well as easier.

## Supported services

Currently only a limited number of AWS services are (largely/partly) supported, though the safety varies:
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
- Appsync
- Athena
- Cloudwatch (-logs)
- CodeBuild
- CodePipeline
- DocumentDB
- ECS
- EventBridge
- Kinesis
- RDS
- Step Functions

### Available builders

Based on `rg ^.*?(\w+Builder).*?$' -N -I -r '$1' | sort | uniq | sed -e 's/^/- /'`

- ApiGatewayV2Builder
- ApplicationBuilder
- AssumeRolePolicyDocumentBuilder
- BucketBuilder
- BucketPolicyBuilder
- CachePolicyBuilder
- ConfigurationProfileBuilder
- CorsConfigurationBuilder
- CorsRuleBuilder
- DefaultCacheBehaviorBuilder
- DeploymentStrategyBuilder
- DistributionBuilder
- EnvironmentBuilder
- FunctionBuilder
- GenerateSecretStringBuilder
- LifecycleConfigurationBuilder
- LifecycleRuleBuilder
- LifecycleRuleTransitionBuilder
- LogGroupBuilder
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
- RoleBuilder
- RolePropertiesBuilder
- SecretBuilder
- StackBuilder
- StatementBuilder
- TableBuilder
- TopicBuilder
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
- _"How do I add tags to resources?"
  - Currently, you can only add tags to the stack, not to individual resources. These tags are then applied when using the `deploy` method. They are not present in the CloudFormation template, because unfortunately, templates do not have a root property for tags.
  - In theory, CloudFormation should propagate the tags to its resources, in practice it will do so in 80–90% of cases.
- _"I create a resource and my deployment failed"_
  - If you think that failure could have been avoided at compile time (or before synthesizing), please open an issue
- _"Wouldn't it be better if synth / another method was async?"_
  - Maybe? But keeping everything except for `deploy` synchronous is easiest for now.
- _"Won't this library always be behind on the latest additions/changes to AWS?"_
  - Sadly, yes. But for a long time that was the case with CloudFormation as well. And sometimes you have to wait for months or a few years before L2-3 constructs arrive in the AWS CDK.

```rust
use rusty_cdk::stack::StackBuilder;
use rusty_cdk::sqs::QueueBuilder;

async fn tagging() {
let mut stack_builder = StackBuilder::new();
// add your resources
stack_builder.add_tag("OWNER", "me").build();
// and deploy
}
```

## TODO

- Pick a style for ids (Camelcase?)
- UpdateReplacePolicy/DeletionPolicy for storage structs (will slow down testing, so not yet)
- more help with IAM permissions
  - additional checks for structure of iam policies
    - for example resources is not required in all cases, but in most contexts it is
- try to replace `syn` with more something more compile-time lightweight - `facet`?
- switch to uploading template to s3? helps avoid the 51 kb limit
  - or at least offer that option
- borrow all the things? see borrowing-example branch for an example
  - the gain in performance was not that impressive
  - so probably not worth the added complexity

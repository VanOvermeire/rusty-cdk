# Cloud Infra

***This is not an official AWS project.***
Rather, it is an experiment in making Infrastructure as Code (_IAC_) safer and easier to use by checking as much as possible at compile time.

It's a bit like a wrapper around `unsafe` CloudFormation.

## Usage

Install using cargo:

`cargo add ...`

## Motivation

This is some CDK code that is valid at compile time (i.e. it synthesizes to a CloudFormation template).

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

But the code will fail at compile time, because it contains various errors:
- table names cannot contain exclamation marks
- a partition key name cannot be empty
- you cannot set `maxReadRequestUnits` when billing mode is `PAY_PER_REQUEST`
- a `maxReadRequestUnits` of `-1` does not make sense
- `maxWriteRequestUnits` is similarly not allowed in this situation, and a value of `0` is a bit special. You're not actually allowed to set this property to zero, but because this is Typescript, the value is interpreted as a falsy and ignored

Fixing these errors will cost you a lot of time because CloudFormation will only notice these issues when creating the change set or deploying the template.
That leads to a slow feedback loop, where you're constantly fixing errors and going through synth and deploy steps, waiting for AWS to tell you where the next issue might be. In other cases, you might not be notified at all, and everything will deploy, but it won't work. In the above example, you do not want to allow writes, but your value of `0` is not valid and simply ignored.

Compare the above with the following:

```rust
use cloud_infra::wrappers::*; // importing all wrappers is a good idea to simplify larger setups
use cloud_infra::{non_zero_number, string_with_only_alpha_numerics_and_underscores};
use cloud_infra::dynamodb::{AttributeType, Key, TableBuilder};
use cloud_infra::stack::{Resource, Stack};
use cloud_infra::stack::StackBuilder;

fn iac() {
  let mut stack_builder = StackBuilder::new();
  
  let dynamo_key = string_with_only_alpha_numerics_and_underscores!("test");
  let table = TableBuilder::new("table", Key::new(dynamo_key, AttributeType::String))
            .provisioned_billing()
            .read_capacity(non_zero_number!(5))
            .write_capacity(non_zero_number!(1))
            .build(&mut stack_builder);
  
  let stack = stack_builder.build().unwrap();
  
  // ready to synth and deploy
}
```

Partition keys can only contain alphanumeric characters and underscores, so they can only be created through a macro that validates this at compile time.
And (max) read capacity can only be set when you choose the correct billing mode.

With this kind of tooling, making mistakes becomes much harder, as some mistakes are caught at compile time and others become impossible.

The library does require you to be somewhat more explicit at times.

For example, you have to pick a billing mode, as well as read and write capacity for provisioned. The CDK 'helps' you by setting sensible defaults (of `5` in this particular case). Which helps you get up and running quickly, but is probably not what you want for any real application!

Also, you have to add all created resources to the `Stack` (or list of resources), whereas the CDK does this automagically. 'Must use' annotations and a final check for the stack do make this a bit less error-prone.

## Approach

This project intends to use any and all tools that Rust offers for ensuring infrastructure correctness at compile time.
In some cases, Rust offers help out of the box. E.g., it has unsigned numbers, unlike TypeScript. 
In addition, macros and type state are the most important additional tools used here.
Const functions would be interesting as well, but too limited for the moment (e.g. I can check a `const` at compile time, but not a `let`).
It might also turn to macros to do more advanced, optional, checking of your actual environment: do you actually have a VPC with id `abc` in your AWS account?
When these are not enough, we can try to use newtype wrappers and `Try` methods (`TryFrom`) to indicate to users that the operation might fail.

## Usage of CloudFormation

Just like the AWS CDK, this project uses CloudFormation to actually create the AWS services you request, unlike for example Terraform which uses API calls.
This has several advantages and allows to focus on creating an easy to use facade for the infrastructure.
This approach also has disadvantages. One is that CloudFormation is slow, in part because it wants to be able to roll back to a stable version if something does go wrong.
That is less important if we're able to make creating the infrastructure completely safe at compile time.
In time, the project might switch to using SDK calls, to try and make things faster as well as easier.

## Supported services

Currently only a limited number of AWS services are largely supported, though the safety varies:
- DynamoDB
- Lambda
- SQS
- SNS
- Cloudwatch
- IAM
- API Gateway
- CloudFront

In other words, you can definitely create serverless applications with this library.

Next up:
- AppConfig

## Tags

Currently, you can only add tags to the stack, not to individual resources.
These tags are applied when using the `deploy` method. 
They are not present in the CloudFormation template, because a root property for tags does not exist there.

```rust
use cloud_infra::stack::StackBuilder;
use cloud_infra::sqs::QueueBuilder;

async fn tagging() {
  let mut stack_builder = StackBuilder::new();
  // add resources
  stack_builder.add_tag("OWNER", "me").build();
  // and deploy
}
```

In theory, CloudFormation should propagate the tags to its resources, in practice it will do so in 80–90% of cases.

## FAQ

- _"I can't find field X of resource Y"_
  - Check whether it's a legacy field (like `maxTTL` in `DefaultCacheBehavior`). If so, I may not have added it, since there's a newer, recommended, alternative. 
  - If it's not a legacy field, I may not have gotten around to adding it yet. I've focussed on the properties that I think are most commonly used/useful.
- _"Where can I find examples of how to use this project?"_
  - Examples can be found in the `examples` dir
  - The snapshot tests in the `cloud-infra` dir also provide some usage examples

## TODO

- docs
  - add docs with example code to all builders
  - note two phases in docs
    - first, compile time checking
    - next, stack result - err if other issues were detected
- probably want to do some more validation when building the stack, for stuff we cannot do at compile time
- help with avoiding missing IAM permissions? perhaps by having user optionally pass in cargo toml(s)
  - when you pass in an env var for a bucket or table, we can assume you want permission to read that? so if none found, error?
  - similar for secret
  - additional checks for structure of iam policies
    - for example resources is not required in all cases, but in most contexts it is
- UpdateReplacePolicy/DeletionPolicy for storage structs (will slow down testing, so not yet)
- try to replace syn and serde with more something more lightweight (at compile time) - facet?
  - note that `Value` is exposed in some cases...
- switch to uploading template to s3? helps avoid the 51 kb limit
- borrow all the things? see borrowing-example branch for an example
  - the gain in performance was not that impressive
- pass in the stack builder in `build` when required?
  - but you still need the resource (reference) to pass on to other resources
    - return a clone? could even be a 'reference' version with only an id

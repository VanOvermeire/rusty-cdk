# Cloud Infra

This is an experiment in making Infrastructure as Code (_IAC_) safer, easier to use by checking things at compile time.

## Usage

Install using cargo:

`cargo add ...`

## Motivating example

This is some CDK code that is valid at compile time (i.e. it synthesizes to a CloudFormation template).

```typescript
export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    new Table(this, 'someId', {
        tableName: "example!!!",
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

Compare the above with this example:

```rust
use cloud_infra::wrappers::{NonZeroNumber,StringWithOnlyAlphaNumericsAndUnderscores};
use cloud_infra::{non_zero_number, string_with_only_alpha_numerics_and_underscores};
use cloud_infra::dynamodb::{AttributeType, DynamoDBKey, DynamoDBTableBuilder};

fn iac() {
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let read_capacity = non_zero_number!(5);
    let write_capacity = non_zero_number!(1);
  
    let resources = vec![
      DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::String))
              .provisioned_billing()
              .read_capacity(read_capacity)
              .write_capacity(write_capacity)
              .build()
              .into()
    ];
    let result = cloud_infra::synth(resources).unwrap();
}
```

Partition keys can only contain alphanumeric characters and underscores, so they can only be created through a macro that validates this at compile time.
And (max) read capacity can only be set when you choose the correct billing mode.

With this kind of tooling, making mistakes becomes much harder, as some mistakes are caught at compile time and others become impossible.

The library does require you to be somewhat more explicit at times. For example, you have to pick a billing mode, as well as read and write capacity for provisioned. The CDK 'helps' you by setting sensible defaults (of `5` in this particular case). Which helps you get up and running quickly, but is probably not what you want for any real application!
 
## Approach

This project intends to use any and all tools that Rust offers for ensuring infrastructure correctness at compile time.
In some cases, Rust offers help out of the box. E.g., it has unsigned numbers, unlike TypeScript. 
In addition, macros and type state are the most important additional tools used here.
It might also turn to macros to do more advanced, optional, checking of your actual environment: do you actually have a VPC with id `abc` in your AWS account?
When these are not enough, we can try to use newtype wrappers and `Try` methods (`TryFrom`) to indicate to users that the operation might fail.

## Usage of CloudFormation

Just like the AWS CDK, this project uses CloudFormation to actually create the AWS services you request, unlike for example Terraform which uses API calls.
This has several advantages and allows to focus on creating an easy to use facade for the infrastructure.
This approach also has disadvantages. One is that CloudFormation is slow, in part because it wants to be able to roll back to a stable version if something does go wrong.
That is less important if we're able to make creating the infrastructure completely safe at compile time.
In time, the project might switch to using SDK calls, to try and make things faster as well as easier.

## Supported services

Currently only a limited number of AWS services are supported, with more on the way:
- DynamoDB
- Lambda
- SQS

Next up:
- Cloudwatch logs
- SNS
- S3
- API Gateway
- Secrets Manager
- AppConfig?
- CloudFront?

## TODO

- add and update docs
- help with avoiding missing IAM permissions? would at least need to check dependencies used by the code

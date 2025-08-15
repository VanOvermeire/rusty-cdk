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
    })
  }
}
```

But the code will fail at compile time, because it contains various errors:
- table names cannot contain exclamation marks
- a partition key name cannot be empty
- you cannot set `maxReadRequestUnits` when billing mode is `PAY_PER_REQUEST`
- a `maxReadRequestUnits` of `-1` does not make sense

Fixing these errors will cost you a lot of time because CloudFormation will only notice these issues when creating the change set or deploying the template.
That leads to a slow feedback loop, where you're constantly fixing errors and going through synth and deploy steps, waiting for AWS to tell you where the next issue might be. In other cases, you might not be notified at all, and everything will deploy, but it won't work.

Compare the above with this example:

```rust
fn iac() {
    let key = create_alphanumeric_underscore_string!("test");
    let resources = vec![DynamoDBTableBuilder::new(DynamoDBKey::new(key, AttributeType::STRING)).provisioned_billing()
        .read_capacity(5)
        .build()];
    let result = cloud_infra::synth(resources).unwrap();
}
```

Partition keys can only contain alphanumeric characters and underscores, so they can only be created through a macro that validates this at compile time.
And (max) read capacity can only be set when you choose the correct billing mode.

With this kind of tooling, making mistakes becomes much harder, as some mistakes are caught at compile time and others become impossible. 
 
## Approach

This project intends to use any and all tools that Rust offers for ensuring infrastructure correctness at compile time.
In some cases, Rust offers help out of the box. E.g., it has unsigned numbers, unlike TypeScript. 
In addition, macros, type state and constant functions are the most important developer tools we can use.
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

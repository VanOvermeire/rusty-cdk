# Adding Resources

The following describes the process of adding resources that are not yet supported by the library.

- Generate boilerplate DTOs and builders using the `resources-scraper` tool.
  - With the 'urls' bin you can find the documentation URLs for the resource group you want to add (e.g. DocDB)
  - The 'resources' bin will retrieve information from the chosen URLs in a sort of CSV format
  - And finally, the 'builder' bin will generate the boilerplate code
- Add the generated code to the `rusty-cdk-core` crate, and add the module to `lib.rs`. Add the resources to the enum in `stack/dto.rs`.
- Once that's done, you can fix the `// TODO` in the resource `build` methods. 
- At this point, you have working builders for the resource. But they won't be very safe yet. Look at the comments that were added to the builder properties, and see how you can improve type safety. Here are the most common ways to do this
  - If something only has a limit number of values, add an enum in the relevant builder method, and change it `Into` a `String`
  - If a `String` has other limitations, try to use one of the macros from `rusty-cdk-macros`, or - if none fit your use case, add one
  - If a reference (id, name, arn) from another AWS resource is required, use a `Ref` to retrieve the (for example) ARN. E.g. if you need the ARN of a KMS key, use the `KeyRef` struct. You will also have to change the type of the relevant `builder` property and `dto` property to `Value`.
    - If the resource for which you need a `Ref` is not supported yet, add the `Ref` and create a lookup in `rusty-cdk-lookups`
  - If a certain value can only be used together with another, consider combining them in a single enum or struct
  - If a group of values is mutually exclusive, use type-state to enforce this. For example, a DynamoDB table has different properties depending on whether it is provisioned or on-demand.
- Once you've improved the builders, you should test the output of your builder against `cfn-lint` (a sanity check that will stop at least some errors) and an actual deploy to AWS (the real check). Look at the `examples` directory for some examples (and optionally add your own).
- Finally, if the deployment succeeds, add one or more snapshot tests to `rusty-cdk/tests/snapshots.rs`.

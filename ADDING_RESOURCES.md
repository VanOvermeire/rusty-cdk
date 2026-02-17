# Adding Resources

The following describes the process of adding resources that are not yet supported by the library.
The information here is also meant as a reference for LLMs, hence why it's quite verbose at times.

## Generate the boilerplate

Generate boilerplate DTOs and builders using the `resources-scraper` tool.
  - With the 'urls' bin you can find the documentation URLs for the resource group you want to add (e.g. DocDB)
  - The 'resources' bin will retrieve information from the chosen URLs in a sort of CSV format
  - And finally, the 'builder' bin will generate the boilerplate code
Add the generated code to the `rusty-cdk-core` crate, and add the module to `lib.rs`. Add the resources to the enum in `stack/dto.rs`.
Once that's done, you can fix the `// TODO` in the resource `build` methods. 

## Improve the builders  
  
At this point, you have working builders for the resource.
But they won't be very safe yet. Look at the comments that were added to the builder properties, and see how you can improve type safety. 

Here are the most common ways to do this:
  - If something only has a limit number of values, add an enum in the relevant builder method, and change it `Into` a `String`
  - If a `String` has other limitations, try to use one of the macros from `rusty-cdk-macros`, or - if none fit your use case, add one
  - If a reference (id, name, arn) from another AWS resource is required, use a `Ref` to retrieve the (for example) ARN. You will also have to change the type of the relevant `builder` property and `dto` property to `Value`. E.g. if property key requires the ARN of a KMS key, add `KeyRef` as the builder parameter and retrieve and store the ARN: `pub fn key(key_ref: &KeyRef) -> Self { Self { key: key_ref.get_arn() }}`.
    - If the resource for which you need a `Ref` is not supported yet, add the `Ref` and create a lookup in `rusty-cdk-lookups`
  - If a certain value can only be used together with another, consider combining them in a single enum or struct
  - If a group of values is mutually exclusive, use type-state to enforce this. For example, a DynamoDB table has different properties depending on whether it is provisioned or on-demand. You can use the `type_state` macro to generate the necessary type trait and structs.

*Important:* the idea is to keep changes to the DTOs minimal. E.g. anything other than changing a `String` into a `Value` is probably bad. Instead, the input of the builder methods should be checked for validity. Once inside (as a builder property), and definitely once we're building and outputting the DTO, it should just be primitive values and existing DTOs.

*Important:* check one of the existing builders (for example the one for `DynamoDB`) to make sure you conform to the existing code style and validations.

## Verifying 

Make sure to verify the following:
- cargo check succeeds
- cargo test succeeds
- if you write a simple example using the new builders, the output is valid according to `cfn-lint` (a sanity check that will stop at least some errors)
- and, preferably, an example can also be deployed to AWS. Look at the `examples` directory for some examples (and optionally add your own).

## Adding snapshot tests & cleanup

If all the checks succeeds, add one or more snapshot tests to `rusty-cdk/tests/snapshots.rs`. 
You can also remove the inline comments that accompany all builder properties

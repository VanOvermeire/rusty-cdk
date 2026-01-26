# Rusty CDK Lookups

This crate provides _even more_ procedural macros for `rusty-cdk`. 

These macros are used to provide compile-time validation for when a resource needs to be referenced, but is defined outside your stack.
E.g., you have manually created a role that you use for event schedules. Or you have a KMS key used for encryption.
This crate allows you to retrieve a reference to a resource (KMS example: `lookup_kms_key_ref!("MyKey", "3e53f2ba-...")`), validating that the resource actually exists in your AWS account. All in the spirit of shifting errors to compile time.

As an override, in case you don't want this additional safety, you can also create such references directly with the `new` method, for instance `RoleRef::new("MyRole", "RoleName", "arn::...")`.

This crate is a dependency of `rusty-cdk` and `rusty-cdk-core`, and its macros are re-exported by `rusty-cdk`. You should not need to depend on this crate directly.

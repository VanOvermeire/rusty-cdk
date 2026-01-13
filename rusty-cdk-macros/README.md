# Rusty CDK Macros

This crate provides procedural macros for `rusty-cdk`. These macros are used to provide compile-time validation.

For example, macros are used to:

*   Validate IAM policies.
*   Check that rate expressions are valid.
*   Ensure that bucket names and other resource identifiers follow AWS naming requirements.

This crate is a dependency of `rusty-cdk` and `rusty-cdk-core`, and its macros are re-exported by `rusty-cdk`. You should not need to depend on this crate directly.

# Rusty CDK Core

This crate provides the core functionality for `rusty-cdk`. It contains the builders, DTOs (Data Transfer Objects), and wrappers for defining AWS resources.

Each AWS service has its own module (e.g., `s3`, `lambda`, `dynamodb`) which contains the necessary components to create and configure resources for that service.

This crate is not intended to be used directly. Instead, you should use the `rusty-cdk` crate, which re-exports components from this crate.


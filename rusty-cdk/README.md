# Rusty CDK

This crate is the main entry point for `rusty-cdk`. It combines and re-exports `rusty-cdk-core` and `rusty-cdk-macros` to provide a seamless experience for defining and deploying AWS infrastructure.

`rusty-cdk` is a safer alternative to the AWS CDK, written in Rust.

## Features

*   **Define Infrastructure in Rust:** Use the power of Rust's type system to define your AWS resources.
*   **Deploy, Diff, and Destroy:** Provides the command-line logic to manage the lifecycle of your CloudFormation stacks.
*   **Safety:** Leverages the Rust compiler to catch errors before deployment.

This crate is intended to be used as a library in your own binary crate that defines your infrastructure stack.

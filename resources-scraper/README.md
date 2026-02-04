# Resources Scraper

This crate contains some helpers for scraping resource info from AWS.

The idea is to translate AWS CloudFormation documentation into a scaffolding for the required Rust code.
At the time of writing, the following is created:
- a `dto.rs` file with DTOs generated based on the resource properties, with the correct serde annotations in place.
- a `builder.rs` file with scaffolding for builders - the correct type state and macros cannot be generated automatically, and need to be manually implemented
- a `mod.rs` file with the necessary imports and exports

All in a folder with the correct 'resource group' name (e.g. `s3`).

## Usage

Run one of the bin targets to generate the code:
- `cargo run --bin urls` outputs a list of all resource urls (e.g. the one for S3)
- `cargo run --bin resources` uses these urls to retrieve relevant info, and outputs it into a kind of CSV file
- `cargo run --bin builder` translates that CSV file into DTOs, builders, etc.

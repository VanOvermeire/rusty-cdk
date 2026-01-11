#![allow(dead_code)]
use rusty_cdk_macros::toml_file;
struct TomlFile(String);
fn main() {
    toml_file!("file_that_does_not_exist.toml");
}


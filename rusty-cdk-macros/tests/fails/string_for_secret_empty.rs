#![allow(dead_code)]
use rusty_cdk_macros::string_for_secret;
struct StringForSecret(String);
fn main() {
    string_for_secret!("");
}


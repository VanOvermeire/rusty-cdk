use rusty_cdk_macros::string_with_only_alphanumerics_and_underscores;

fn example() {
    let key = string_with_only_alphanumerics_and_underscores!("invalid-name");
}

fn main() {}
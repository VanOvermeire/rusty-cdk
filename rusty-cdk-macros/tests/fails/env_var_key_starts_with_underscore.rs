use rusty_cdk_macros::env_var_key;

fn example() {
    let key = env_var_key!("_INVALID");
}

fn main() {}
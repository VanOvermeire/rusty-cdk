use rusty_cdk_macros::env_var_key;

fn example() {
    let key = env_var_key!("INVALID-KEY");
}

fn main() {}
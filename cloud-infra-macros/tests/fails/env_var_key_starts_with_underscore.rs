use cloud_infra_macros::env_var_key;

fn example() {
    let key = env_var_key!("_INVALID");
}

fn main() {}
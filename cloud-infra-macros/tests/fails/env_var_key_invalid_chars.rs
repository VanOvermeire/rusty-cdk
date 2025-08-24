use cloud_infra_macros::env_var_key;

fn example() {
    let key = env_var_key!("INVALID-KEY");
}

fn main() {}
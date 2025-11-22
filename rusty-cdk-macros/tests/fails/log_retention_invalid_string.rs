use rusty_cdk_macros::log_retention;

fn example() {
    let retention = log_retention!("not_a_number");
}

fn main() {}
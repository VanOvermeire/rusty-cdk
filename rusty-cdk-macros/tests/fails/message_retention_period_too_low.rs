use rusty_cdk_macros::message_retention_period;

fn example() {
    let period = message_retention_period!(30);
}

fn main() {}
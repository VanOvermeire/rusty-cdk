use cloud_infra_macros::message_retention_period;

fn example() {
    let period = message_retention_period!(1500000);
}

fn main() {}
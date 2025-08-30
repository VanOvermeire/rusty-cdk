use cloud_infra_macros::log_retention;

fn example() {
    let retention = log_retention!("not_a_number");
}

fn main() {}
use cloud_infra_macros::non_zero_number;

fn example() {
    let key = non_zero_number!("not_a_number");
}

fn main() {}
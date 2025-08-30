use cloud_infra_macros::log_group_name;

fn example() {
    let name = log_group_name!("invalid@chars");
}

fn main() {}
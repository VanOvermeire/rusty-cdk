use cloud_infra_macros::visibility_timeout;

fn example() {
    let timeout = visibility_timeout!(50000);
}

fn main() {}
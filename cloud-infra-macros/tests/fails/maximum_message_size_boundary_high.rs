use cloud_infra_macros::maximum_message_size;

fn example() {
    let size = maximum_message_size!(1048577);
}

fn main() {}
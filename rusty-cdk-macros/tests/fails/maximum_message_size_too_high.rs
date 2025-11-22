use rusty_cdk_macros::maximum_message_size;

fn example() {
    let size = maximum_message_size!(2097152);
}

fn main() {}
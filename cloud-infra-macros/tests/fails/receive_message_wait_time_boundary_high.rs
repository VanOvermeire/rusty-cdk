use cloud_infra_macros::receive_message_wait_time;

fn example() {
    let wait_time = receive_message_wait_time!(21);
}

fn main() {}
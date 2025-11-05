use cloud_infra_macros::iam_action;

fn example() {
    let action = iam_action!("s3:InvalidAction");
}

fn main() {}

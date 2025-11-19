use cloud_infra_macros::bucket;

fn example() {
    let bucket = bucket!("arn:aws:s3:::some-examples-arn");
}

fn main() {}
use rusty_cdk_macros::bucket;

fn example() {
    let bucket = bucket!("arn:aws:s3:::some-example-arn");
}

fn main() {}
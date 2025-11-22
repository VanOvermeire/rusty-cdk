use rusty_cdk_macros::zip_file;

fn example() {
    let file = zip_file!("nonexistent.zip");
}

fn main() {}
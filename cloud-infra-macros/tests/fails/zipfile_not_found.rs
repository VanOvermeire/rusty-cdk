use cloud_infra_macros::zipfile;

fn example() {
    let file = zipfile!("nonexistent.zip");
}

fn main() {}
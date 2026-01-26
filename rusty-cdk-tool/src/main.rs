use clap::Parser;
use cargo_rusty::{entry_point, Args};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    entry_point(args.command).await;
}

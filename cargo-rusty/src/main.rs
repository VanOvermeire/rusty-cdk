use cargo_rusty::{Args, entry_point};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    entry_point(args.command).await;
}

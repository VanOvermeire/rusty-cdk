use clap::Parser;
use rusty_cdk_tool::{entry_point, Args};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    entry_point(args.command).await;
}

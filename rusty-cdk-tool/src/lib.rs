use std::sync::OnceLock;
use clap::Parser;
use clap::Subcommand;
use rusty_cdk::{destroy};
use rusty_cdk::wrappers::StringWithOnlyAlphaNumericsAndHyphens;

pub static DEBUG_CELL: OnceLock<bool> = OnceLock::new();

#[derive(Clone, Debug, Subcommand)]
pub enum RustyCommand {
    #[clap(about = "Deploy a stack")]
    Deploy {
        /// Name of the stack when it's deployed
        #[clap(short, long)]
        name: String,
    },
    #[clap(about = "Generate diff with a deployed template with the given name")]
    Diff {
        /// Name of the (deployed) stack that you want to compare with
        #[clap(short, long)]
        name: String,
    },
    #[clap(about = "Destroy a stack with the give name")]
    Destroy {
        /// Name of the (deployed) stack that you want to delete
        name: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: RustyCommand,
}

pub async fn entry_point(command: RustyCommand) {
    match command {
        RustyCommand::Deploy { name } => {
            println!("deploying stack with name {name}");
        }
        RustyCommand::Diff { name } => {
            println!("creating a diff with an existing stack (name {name})");
        }
        RustyCommand::Destroy { name } => {
            println!("destroying stack with name {name}");
            destroy(StringWithOnlyAlphaNumericsAndHyphens(name)).await;
        }
    }
}
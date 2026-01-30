use clap::Parser;
use clap::Subcommand;
use rusty_cdk::deploy;
use rusty_cdk::destroy;
use rusty_cdk::diff;
use rusty_cdk::stack::Stack;
use rusty_cdk::wrappers::StringWithOnlyAlphaNumericsAndHyphens;
use rusty_cdk::clean;
use std::fmt::Display;
use std::fs::{read_dir, read_to_string};
use std::process::exit;
use tokio::fs::remove_file;
use tokio::process::Command;

const CURRENT_DIR: &'static str = ".";

#[derive(Clone, Debug, Subcommand)]
pub enum RustyCommand {
    #[clap(about = "Deploy a stack")]
    Deploy {
        /// Name of the stack when it's deployed
        #[clap(short, long)]
        name: String,
        /// Path of synthesized stack relative to the current directory
        /// If no path is passed in, the command will generate a synthesized stack using `cargo run`
        #[clap(short, long)]
        synth_path: Option<String>,
        /// Cleans up the generated or passed-in synth file
        #[clap(short, long)]
        cleanup: bool,
    },
    #[clap(about = "Generate diff with a deployed template with the given name")]
    Diff {
        /// Name of the (deployed) stack that you want to compare with
        #[clap(short, long)]
        name: String,
        /// Path of synthesized stack relative to the current directory
        /// If no path is passed in, the command will generate a synthesized stack using `cargo run`
        #[clap(short, long)]
        synth_path: Option<String>,
        /// Cleans up the generated or passed-in synth file
        #[clap(short, long, default_missing_value = "false")]
        cleanup: bool,
    },
    #[clap(about = "Destroy a stack with the give name")]
    Destroy {
        /// Name of the (deployed) stack that you want to delete
        name: String,
        /// Force tries to make sure your stack deletes, avoiding common things that can throw a `DeleteFailed` error.
        /// To do this, `force` will:
        /// - empty S3 buckets that do not have a 'retain'
        /// - remove any archival policies from SNS topics
        /// As resources with a 'retain' policy are not deleted and cause no deletion issues, these are ignored.
        /// *Use with caution*, and only if you don't need to retain anything from your stack (that is not set to 'retain')
        #[clap(short, long, default_missing_value = "false")]
        force: bool,
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
        RustyCommand::Deploy { name, synth_path, cleanup } => {
            println!("deploying stack with name {name}");

            let path = if let Some(path) = synth_path {
                path
            } else {
                match run_synth_in_current_path().await {
                    Ok(path) => path,
                    Err(e) => print_err_and_exit(e)
                }
            };
            match get_path_as_stack(&path) {
                Ok(stack) => {
                    match deploy(StringWithOnlyAlphaNumericsAndHyphens(name), stack, true).await {
                        Ok(_) => {},
                        Err(e) => print_err_and_exit(e)
                    }
                }
                Err(e) => print_err_and_exit(e)
            }

            if cleanup {
                remove_fill_or_exit(&path).await;
            }
        }
        RustyCommand::Diff { name, synth_path, cleanup } => {
            println!("creating a diff with an existing stack (name {name})");

            let path = if let Some(path) = synth_path {
                path
            } else {
                match run_synth_in_current_path().await {
                    Ok(path) => path,
                    Err(e) => print_err_and_exit(e),
                }
            };
            match get_path_as_stack(&path) {
                Ok(stack) => {
                    match diff(StringWithOnlyAlphaNumericsAndHyphens(name), stack).await {
                        Ok(_) => {},
                        Err(e) => print_err_and_exit(e),
                    }
                }
                Err(e) => print_err_and_exit(e),
            }

            if cleanup {
                remove_fill_or_exit(&path).await;
            }
        }
        RustyCommand::Destroy { name, force } => {
            println!("destroying stack with name {name}");

            if force {
                match clean(StringWithOnlyAlphaNumericsAndHyphens(name.to_string()), true).await {
                    Ok(_) => {}
                    Err(e) => print_err_and_exit(e)
                }
            }
            match destroy(StringWithOnlyAlphaNumericsAndHyphens(name), true).await {
                Ok(_) => {},
                Err(e) => print_err_and_exit(e),
            }
        }
    }
}

// alternative to all these matches, an error that implements some Froms
async fn run_synth_in_current_path() -> Result<String, String> {
    let dir_content = read_dir(CURRENT_DIR);

    match dir_content {
        Ok(content) => {
            let is_rust_project = content
                .flat_map(|entry| entry.ok())
                .any(|entry| entry.file_name() == "Cargo.toml" && entry.file_type().map(|f| f.is_file()).unwrap_or(false));

            if is_rust_project {
                match Command::new("sh")
                    .args(&["-c", "cargo run > cargo-rusty-temporary-synth.json"])
                    .output()
                    .await
                {
                    Ok(_) => Ok("./cargo-rusty-temporary-synth.json".to_string()),
                    Err(e) => Err(format!(
                        "Could not run `cargo run` (required to synth when no synth_path is passed in): {e}"
                    )),
                }
            } else {
                Err("current dir does not seem to be a cargo project, could not find a Cargo.toml (required to synth when no synth_path is passed in)".to_string())
            }
        }
        Err(e) => Err(format!("could not read dir: {e}")),
    }
}

fn get_path_as_stack(path: &str) -> Result<Stack, String> {
    match read_to_string(path) {
        Ok(as_string) => match serde_json::from_str::<Stack>(&as_string) {
            Ok(stack) => Ok(stack),
            Err(e) => Err(format!(
                "content of file {path} could not be read as a `Stack` (is there non-json content present?): {e}"
            )),
        },
        Err(e) => Err(format!("could not read file with path {path}: {e}")),
    }
}

async fn remove_fill_or_exit(path: &String) {
    let removed = remove_file(&path).await;

    if let Err(e) = removed {
        print_err_and_exit(format!("error cleaning up file at {path}: {e}"));
    }
}

fn print_err_and_exit<T: Display>(e: T) -> ! {
    eprintln!("{e}");
    exit(1);
}
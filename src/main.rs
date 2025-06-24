use clap::{Parser, Subcommand};
use git::{fetch_files, squash, tree_structure, vibe_push, generate_commit_message, get_diff};
use utils::{openrouter_check_limits, initialize_config, get_config};
use std::path::PathBuf;
use std::io::{self, Write};
use anyhow::Context;


mod git;
mod model;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the gitzeug config
    Config {},
    /// Generates a commit message using AI
    Ai{
        /// Use AI to generate a commit message
        #[arg(short = 'm', long = "mesage")]
        message: bool,
        /// Check limits on AI usage
        #[arg(short = 'l', long = "limits")]
        limits: bool,
    },
    /// Downloads the specified directories and files from a git repo
    Dl {
        /// URL of the repo. Format: https://github.com/thaemisch/gitzeug.git
        #[arg(short = 'u', long = "url")]
        url: String,
        /// Output directory
        #[arg(short = 'o', long = "output")]
        path: PathBuf,
        /// Directories & Files to download (Comma-seperated). Format: README.md,src,xyz
        #[arg(short = 'f', long = "files", value_delimiter = ',')]
        files: Vec<String>,
    },
    /// Stages all changes in the current directory, commits and pushes
    Push {
        /// Generates a commit message using AI
        #[arg(short = 'a', long = "ai")]
        ai: bool,
        /// Commit message
        #[arg(short = 'm', long = "message", default_value = "")]
        cmtmsg: String,
    },
    /// Squashses the last n commits
    Squash {
        /// Number of commits to Squash
        number: u32,
        /// New commit message
        cmtmsg: String,
    },
    /// Shows the file tree for the git repo
    Browse {
        /// URL of the repo
        url: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Config {}) => {
            let _ = initialize_config();
        },
        Some(Commands::Ai {message, limits}) => {
            cmd_ai(*message, *limits).await;
        },
        Some(Commands::Dl { url, path, files }) => {
            let paths: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
            let _ = fetch_files(url, &paths, path);
        }
        Some(Commands::Push { ai, cmtmsg }) => {
            cmd_push(*ai, cmtmsg.to_string()).await;
        }
        Some(Commands::Squash { number, cmtmsg }) => {
            squash(number, cmtmsg);
        }
        Some(Commands::Browse { url }) => {
            let _ = tree_structure(url);
        }
        _none => {
            eprintln!("No command provided. Use --help for usage information.");
        }
    }
}


async fn cmd_ai(message: bool, limits: bool) {
    if message {
        let diff = get_diff().unwrap_or_else(|_| {
            eprintln!("Failed to get diff. Ensure you are in a git repository.");
            return String::new();
        });
        let output = generate_commit_message(&diff).await;
        println!("Generated commit message: {}", output.unwrap_or_else(|_| {
            eprintln!("Failed to generate commit message.");
            String::new()
        }));
    } else if limits {
        let config = match get_config().context("Failed to get config") {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };
        let ai_cmsg = match config.get("ai_cmsg").context("Missing 'ai_cmsg' section in config") {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };
        let key = match ai_cmsg.get("key").and_then(|v| v.as_str()).context("Missing or invalid 'key' in ai_cmsg config") {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        };
        match openrouter_check_limits(key).await {
            Ok(limits_output) => println!("{}", limits_output),
            Err(e) => eprintln!("Failed to check usage limits: {:?}", e),
        }
    } else {
        eprintln!("Please specify either --message or --limits.");
    }
}

async fn cmd_push(ai: bool, cmtmsg: String) {
    if ai && cmtmsg.is_empty() {
        let diff = get_diff().unwrap_or_else(|_| {
            eprintln!("Failed to get diff. Ensure you are in a git repository.");
            return String::new();
        });
        let generated_msg = generate_commit_message(&diff).await.unwrap_or_else(|_| {
            eprintln!("Failed to generate commit message.");
            return String::new();
        });
        println!("Generated commit message: {}", generated_msg);
        println!("Do you want to use the generated commit message? (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        let final_msg = if input == "y" || input == "yes" {
            generated_msg
        } else {
            println!("Enter your commit message: ");
            io::stdout().flush().unwrap();
            let mut edited_msg = String::new();
            io::stdin().read_line(&mut edited_msg).unwrap();
            edited_msg.trim().to_string()
        };
        println!("Using commit message: {}", final_msg);
        vibe_push(&final_msg);
    } else if ai && !cmtmsg.is_empty() {
        eprintln!("You cannot use both --ai and a commit message at the same time.");
    } else if !ai && cmtmsg.is_empty() {
        eprintln!("You must provide a commit message or use --ai to generate one.");
    } else if !ai && !cmtmsg.is_empty() {
        vibe_push(&cmtmsg);
    }
}
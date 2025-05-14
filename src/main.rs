use clap::{Parser, Subcommand};
use git::{fetch_files, squash, vibe_push};
use std::path::PathBuf;

mod git;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Downloads the specified directories and files from a git repo.
    Dl {
        /// URL of the repo. Format: https://github.com/thaemisch/gitzeug.git
        #[arg(short = 'u', long = "url")]
        url: String,
        /// Output directory
        #[arg(short = 'o', long = "output")]
        path: PathBuf,
        /// Directories & Files to download (Comma-seperated). Format: README.md,src/,xyz/
        #[arg(short = 'f', long = "files", value_delimiter = ',')]
        files: Vec<String>,
    },
    /// Stages all changes in the current directory, commits and pushes.
    Push {
        /// Commit message
        cmtmsg: String,
    },
    /// Squashses the last n commits
    Squash {
        /// Number of commits to Squash
        number: u32,
        /// New commit message
        cmtmsg: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Dl { url, path, files }) => {
            let paths: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
            let _ = fetch_files(url, &paths, path);
        }
        Some(Commands::Push { cmtmsg }) => {
            vibe_push(cmtmsg);
        }
        Some(Commands::Squash { number, cmtmsg }) => {
            squash(number, cmtmsg);
        }
        None => {}
    }
}

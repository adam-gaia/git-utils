use clap::{Parser, Subcommand};
use color_eyre::Result;
use log::info;
use which::which;
use std::path::PathBuf;
use std::str::FromStr;

mod command;
use command::run;

mod repo_url;
use repo_url::RepoUrl;

#[derive(Debug, Subcommand)]
enum Commands {
    SetUpstream {
        upstream: RepoUrl,
    },

    Fork {
        /// Url of upstream repo
        upstream: Option<RepoUrl>,
    },
}

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // TODO
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    // TODO: it would be cool to one day use gitoxide instead of shelling out to git
    let git = which("git")?;
    let gh = which("gh")?;

    let args = Cli::parse();
    match &args.command {
        Some(command) => {
            match command {
                Commands::SetUpstream{ upstream } => {
                    run("", &[""]).await?;
                },
                Commands::Fork{ upstream } => {
                    // TODO
                    info!("TODO: Command::Todo");
                }
            }
        }
        None => {
            // TODO
            info!("TODO: no subcommand provided");
        }
    }

    Ok(())
}

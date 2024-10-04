use clap::Parser;
use color_eyre::eyre::bail;
use color_eyre::Result;
use log::debug;
use log::info;
use log::warn;
use std::process::Command;
use std::str;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {}

fn git(args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).output()?;
    let stdout = str::from_utf8(&output.stdout)?.trim();
    let stderr = str::from_utf8(&output.stderr)?.trim();
    warn!("{}", stderr);
    Ok(stdout.to_string())
}

fn current_branch() -> Result<String> {
    git(&["branch", "--show-current"])
}

fn default_branch(remote: &str) -> Result<String> {
    let origin_info = git(&["remote", "show", remote])?;
    for line in origin_info.lines() {
        let line = line.trim();
        if line.starts_with("HEAD branch:") {
            let branch = line.replace("HEAD branch: ", "");
            return Ok(branch);
        }
    }
    bail!("Unable to find default branch");
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let args = Cli::parse();

    let current = current_branch()?;
    let default = default_branch("origin")?;
    if current == default {
        debug!("{}, {}", current, default);
    }

    Ok(())
}

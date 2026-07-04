use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(name = "git-ignore")]
#[command(version)]
#[command(about = "The maintenance toolkit for .gitignore files.")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Add {
        pattern: String,
    },
    Remove {
        pattern: String,
    },
    List,
    Dedupe,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Init => commands::init::run(),
            Command::Add { pattern } => commands::add::run(pattern),
            Command::Remove { pattern } => commands::remove::run(pattern),
            Command::List => commands::list::run(),
            Command::Dedupe => commands::dedupe::run(),
        }
    }
}
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
    /// Create or initialize a .gitignore
    Init,

    /// Add a pattern
    Add { pattern: String },

    /// Create from template e.g. Node, Rust, C++, Dotnet etc.
    Template {
        /// Name of the template to use
        name: Option<String>,

        /// List all available templates
        #[arg(short, long)]
        list: bool,
    },

    /// Remove a pattern
    Remove { pattern: String },

    /// List ignored patterns
    List,

    /// Remove duplicate rules
    Dedupe,

    /// Check if a pattern exists
    Check { pattern: String },

    /// Check for any anomalies, get recommendations and or apply them
    Audit {
        #[command(subcommand)]
        action: Option<AuditCommand>,
    },
}

#[derive(Subcommand)]
enum AuditCommand {
    Fix,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Init => commands::init::run(),
            Command::Add { pattern } => commands::add::run(pattern),
            Command::Template { name, list } => commands::template::run(name, list),
            Command::Remove { pattern } => commands::remove::run(pattern),
            Command::List => commands::list::run(),
            Command::Dedupe => commands::dedupe::run(),
            Command::Check { pattern } => commands::check::run(pattern),
            Command::Audit { action } => match action {
                Some(AuditCommand::Fix) => commands::audit::fix(),
                None => commands::audit::run(),
            },
        }
    }
}

use anyhow::Result;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};

use crate::commands;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold())
        .usage(AnsiColor::Yellow.on_default().bold())
        .literal(AnsiColor::Green.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default())
}

const BANNER: &str = r"
   _   _ _   _       _                       
  / |_(_) |_(_) __ _ _ __   ___  _ __ ___ 
 / _` | | __| |/ _` | '_ \ / _ \| '__/ _ \\
| (_| | | |_| | (_| | | | | (_) | | |  __/
 \__, |_|\__|_|\__, |_| |_|\___/|_|  \___|
 |___/         |___/                      
";

#[derive(Parser)]
#[command(name = "git-ignore")]
#[command(version)]
#[command(about = "The maintenance toolkit for .gitignore files.", before_help = BANNER)]
#[command(styles = styles())]
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

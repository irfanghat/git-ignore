use anyhow::Result;
use colored::Colorize;

use crate::git::repo::Repository;

pub fn run() -> Result<()> {
    let repo = Repository::discover()?;

    println!(
        "{} {}",
        "Repository:".bold().cyan(),
        repo.root.display().to_string().dimmed()
    );

    if repo.ensure_gitignore()? {
        println!("{} {}", "Created".green().bold(), repo.gitignore.display());
    } else {
        println!("{} {}", "Found".blue().bold(), repo.gitignore.display());
    }

    Ok(())
}

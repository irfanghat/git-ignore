use anyhow::Result;
use colored::Colorize;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run() -> Result<()> {
    let repo = Repository::discover()?;

    let doc = IgnoreDocument::load(&repo.gitignore)?;

    if doc.is_empty() {
        println!("{}", "No ignore rules found.".yellow());
        return Ok(());
    }

    println!("{}", "Current ignore rules:\n".bold().cyan());
    for rule in doc.rules() {
        println!("  {} {}", "-".yellow(), rule.pattern);
    }

    Ok(())
}

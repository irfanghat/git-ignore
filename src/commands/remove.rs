use anyhow::Result;
use colored::Colorize;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run(pattern: String) -> Result<()> {
    let repo = Repository::discover()?;

    let mut doc = IgnoreDocument::load(&repo.gitignore)?;

    if doc.remove(&pattern) {
        doc.save(&repo.gitignore)?;
        println!("{} '{}'", "Removed".red().bold(), pattern);
    } else {
        println!("'{}' was not found.", pattern.yellow());
    }

    Ok(())
}

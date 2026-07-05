use anyhow::Result;
use colored::Colorize;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run(pattern: String) -> Result<()> {
    let repo = Repository::discover()?;

    let mut doc = IgnoreDocument::load(&repo.gitignore)?;

    if doc.add(pattern.clone()) {
        doc.save(&repo.gitignore)?;
        println!("{} '{}'", "Added".green().bold(), pattern);
    } else {
        println!("'{}' is already ignored.", pattern.yellow());
    }

    Ok(())
}

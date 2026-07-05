use anyhow::Result;
use colored::Colorize;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run(pattern: String) -> Result<()> {
    let repo = Repository::discover()?;
    let doc = IgnoreDocument::load(&repo.gitignore)?;

    if doc.contains(&pattern) {
        println!("{} '{}' is ignored.", "✓".green().bold(), pattern);
    } else {
        println!("{} '{}' is not ignored.", "✗".red().bold(), pattern);
    }

    Ok(())
}

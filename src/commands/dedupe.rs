use anyhow::Result;
use colored::Colorize;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run() -> Result<()> {
    let repo = Repository::discover()?;

    let mut doc = IgnoreDocument::load(&repo.gitignore)?;

    let removed = doc.dedupe();

    doc.save(&repo.gitignore)?;

    if removed > 0 {
        println!(
            "{} removed {} duplicate rule(s).",
            "Deduplicated:".green().bold(),
            removed
        );
    } else {
        println!("{}", "No duplicate rules found.".dimmed());
    }

    Ok(())
}

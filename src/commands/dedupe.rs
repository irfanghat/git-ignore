use anyhow::Result;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run() -> Result<()> {
    let repo = Repository::discover()?;

    let mut doc = IgnoreDocument::load(&repo.gitignore)?;

    let removed = doc.dedupe();

    doc.save(&repo.gitignore)?;

    println!("Removed {} duplicate rule(s).", removed);

    Ok(())
}

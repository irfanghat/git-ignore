use anyhow::Result;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run(pattern: String) -> Result<()> {
    let repo = Repository::discover()?;

    let mut doc = IgnoreDocument::load(&repo.gitignore)?;

    if doc.remove(&pattern) {
        doc.save(&repo.gitignore)?;
        println!("Removed '{}'", pattern);
    } else {
        println!("'{}' was not found.", pattern);
    }

    Ok(())
}

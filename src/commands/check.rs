use anyhow::Result;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run(pattern: String) -> Result<()> {
    let repo = Repository::discover()?;
    let doc = IgnoreDocument::load(&repo.gitignore)?;

    if doc.contains(&pattern) {
        println!("✓ '{}' is ignored.", pattern);
    } else {
        println!("✗ '{}' is not ignored.", pattern);
    }

    Ok(())
}

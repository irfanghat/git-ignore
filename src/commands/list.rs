use anyhow::Result;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;

pub fn run() -> Result<()> {
    let repo = Repository::discover()?;

    let doc = IgnoreDocument::load(&repo.gitignore)?;

    if doc.is_empty() {
        println!("No ignore rules.");
        return Ok(());
    }

    for rule in doc.rules() {
        println!("{}", rule.pattern);
    }

    Ok(())
}

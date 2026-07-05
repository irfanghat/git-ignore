use anyhow::Result;

use crate::git::repo::Repository;

pub fn run() -> Result<()> {
    let repo = Repository::discover()?;

    println!("Repository: {}", repo.root.display());

    if repo.ensure_gitignore()? {
        println!("Created {}", repo.gitignore.display());
    } else {
        println!("Found {}", repo.gitignore.display());
    }

    Ok(())
}

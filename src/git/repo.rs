use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

pub fn find_repo_root(start: impl AsRef<Path>) -> Result<PathBuf> {
    let mut current = start.as_ref().canonicalize()?;

    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }

        if !current.pop() {
            bail!("Not inside a Git repository");
        }
    }
}
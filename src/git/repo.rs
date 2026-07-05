use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Repository {
    pub root: PathBuf,
    pub gitignore: PathBuf,
}

impl Repository {
    pub fn discover() -> Result<Self> {
        let cwd = env::current_dir()?;
        Self::from_path(cwd)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut current = fs::canonicalize(path)?;

        loop {
            let git = current.join(".git");

            if git.exists() {
                return Ok(Self {
                    root: current.clone(),
                    gitignore: current.join(".gitignore"),
                });
            }

            if !current.pop() {
                bail!("Not inside a Git repository.");
            }
        }
    }

    pub fn ensure_gitignore(&self) -> Result<bool> {
        if self.gitignore.exists() {
            return Ok(false);
        }

        fs::write(&self.gitignore, DEFAULT_GITIGNORE).context("Failed to create .gitignore")?;

        Ok(true)
    }
}

const DEFAULT_GITIGNORE: &str = r#"
# -----------------------------------------------------------------
# Managed by git-ignore
#
# This file is maintained by git-ignore.
# -----------------------------------------------------------------
"#;

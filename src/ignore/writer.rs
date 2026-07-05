use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::ignore::document::{Entry, IgnoreDocument};

impl IgnoreDocument {
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut output = String::new();

        for entry in &self.entries {
            match entry {
                Entry::Blank => {
                    output.push('\n');
                }

                Entry::Comment(comment) => {
                    output.push_str(&comment.text);
                    output.push('\n');
                }

                Entry::Rule(rule) => {
                    output.push_str(&rule.pattern);
                    output.push('\n');
                }
            }
        }

        fs::write(path, output)?;

        Ok(())
    }
}

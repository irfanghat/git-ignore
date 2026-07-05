use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::ignore::document::{Comment, Entry, IgnoreDocument};
use crate::ignore::rule::Rule;

impl IgnoreDocument {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self {
                entries: Vec::new(),
            });
        }

        let contents = fs::read_to_string(path)?;

        let mut entries = Vec::new();

        for line in contents.lines() {
            if line.is_empty() {
                entries.push(Entry::Blank);
            } else if line.starts_with('#') {
                // ---------------------------------
                // Store comment without mutation
                // ---------------------------------
                entries.push(Entry::Comment(Comment::new(line.to_string())));
            } else {
                // -----------------------------------------------------------
                // Only trim newline artifacts implicitly handled by lines()
                // -----------------------------------------------------------
                entries.push(Entry::Rule(Rule::new(line.to_string())));
            }
        }

        Ok(Self { entries })
    }
}

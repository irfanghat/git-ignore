use anyhow::{Context, Result};

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;
use crate::templates::{list_templates, load_template};

pub fn run(name: Option<String>, list: bool) -> Result<()> {
    if list {
        let templates = list_templates()?;
        println!("Available templates:\n");
        for t in templates {
            println!("  {}", t);
        }
        return Ok(());
    }

    let name = name.context("Template name is required. Use --list to see available templates.")?;

    let repo = Repository::discover()?;

    let body = load_template(&name)?;

    let mut doc = IgnoreDocument::load(&repo.gitignore)?;

    for line in body.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        doc.add(trimmed.to_string());
    }

    doc.save(&repo.gitignore)?;

    println!("Applied '{}' template", name);

    Ok(())
}

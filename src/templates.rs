use anyhow::{anyhow, Result};

use crate::generated::TEMPLATES;

/// Load an embedded gitignore template by name.
pub fn load_template(name: &str) -> Result<String> {
    TEMPLATES
        .iter()
        .find(|(template_name, _)| *template_name == name)
        .map(|(_, contents)| contents.to_string())
        .ok_or_else(|| anyhow!("Template '{}' not found", name))
}

/// Return all available embedded template names.
pub fn list_templates() -> Result<Vec<String>> {
    let mut templates = TEMPLATES
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect::<Vec<_>>();

    templates.sort();

    Ok(templates)
}

use anyhow::{Result, bail};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn load_template(name: &str) -> Result<String> {
    let path = PathBuf::from("src/gitignore").join(format!("{}.gitignore", name));

    if !path.exists() {
        bail!("Template '{}' not found", name);
    }

    Ok(fs::read_to_string(path)?)
}

pub fn list_templates() -> Result<Vec<String>> {
    let base_path = PathBuf::from("src/gitignore");
    if !base_path.exists() {
        bail!("Templates directory 'src/gitignore' not found");
    }

    let mut templates = Vec::new();

    for entry in WalkDir::new(&base_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "gitignore") {
            if let Ok(rel_path) = path.strip_prefix(&base_path) {
                let name = rel_path.to_string_lossy();
                // Strip the .gitignore extension and normalize separators for cross-platform usage
                let name = name
                    .strip_suffix(".gitignore")
                    .unwrap_or(&name)
                    .replace('\\', "/");
                templates.push(name);
            }
        }
    }

    templates.sort();
    Ok(templates)
}

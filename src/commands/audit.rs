use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;
use walkdir::WalkDir;

use crate::git::repo::Repository;
use crate::ignore::document::IgnoreDocument;
use crate::sensitive::loader::load_patterns;

/// Scan for issues
pub fn run() -> Result<()> {
    let repo = Repository::discover()?;
    let ignore = IgnoreDocument::load(&repo.gitignore)?;

    let patterns = load_patterns()?;

    let mut findings = Vec::new();

    for entry in WalkDir::new(&repo.root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let rel = path.strip_prefix(&repo.root)?;
        let rel_str = rel.to_string_lossy();

        for category in &patterns.categories {
            for pattern in &category.patterns {
                if matches_pattern(&rel_str, pattern) {
                    // --------------------------------------------------------------------
                    // Only report if the matching pattern isn't already in .gitignore
                    // --------------------------------------------------------------------
                    if !ignore.contains(pattern) {
                        findings.push((
                            category.severity.as_str(),
                            category.label.as_str(),
                            rel_str.to_string(),
                            pattern.clone(),
                        ));
                    }
                }
            }
        }
    }

    if findings.is_empty() {
        println!("{}", "✔ No sensitive files detected.".green().bold());
        return Ok(());
    }

    println!("{}", "Sensitive file audit results:\n".bold().cyan());

    for (severity, label, file, pattern) in findings {
        let severity_styled = match severity.to_uppercase().as_str() {
            "HIGH" => severity.to_uppercase().red().bold(),
            "MEDIUM" => severity.to_uppercase().yellow().bold(),
            _ => severity.to_uppercase().blue().bold(),
        };

        println!(
            "[{}] {} -> {} (matched: {})",
            severity_styled,
            label.bright_white(),
            file.dimmed(),
            pattern.italic()
        );
    }

    Ok(())
}

/// Fix detected issues
pub fn fix() -> Result<()> {
    let repo = Repository::discover()?;

    // -----------------------------------------------------------------------------
    // Ensure .gitignore exists. If not, create it with the default header
    // -----------------------------------------------------------------------------
    repo.ensure_gitignore()?;

    let patterns = load_patterns()?;
    let mut doc = IgnoreDocument::load(&repo.gitignore)?;

    let mut added = Vec::new();
    let mut detected_patterns = HashSet::new();

    // -----------------------------------------------------------------------------
    // Scan for files actually present in the repo that match sensitive patterns
    // -----------------------------------------------------------------------------
    for entry in WalkDir::new(&repo.root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let rel = path.strip_prefix(&repo.root)?;
        let rel_str = rel.to_string_lossy();

        for category in &patterns.categories {
            for pattern in &category.patterns {
                if matches_pattern(&rel_str, pattern) {
                    detected_patterns.insert(pattern.clone());
                }
            }
        }
    }

    // --------------------------------------------
    // Sort patterns for deterministic output
    // --------------------------------------------
    let mut sorted_patterns: Vec<_> = detected_patterns.into_iter().collect();
    sorted_patterns.sort();

    for pattern in sorted_patterns {
        if doc.add(pattern.clone()) {
            added.push(pattern);
        }
    }

    if added.is_empty() {
        println!("{}", "No new sensitive files detected to ignore.".yellow());
    } else {
        doc.save(&repo.gitignore)?;
        println!(
            "{}",
            "Audit fix applied (added detected patterns to .gitignore):\n"
                .bold()
                .green()
        );

        for p in added {
            println!("  {} {}", "+".green().bold(), p);
        }
    }

    Ok(())
}

fn matches_pattern(file: &str, pattern: &str) -> bool {
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();

        let mut start = 0;

        for part in parts {
            if part.is_empty() {
                continue;
            }

            if let Some(pos) = file[start..].find(part) {
                start += pos + part.len();
            } else {
                return false;
            }
        }

        true
    } else {
        file.ends_with(pattern) || file.contains(pattern) || file == pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_pattern_exact() {
        assert!(matches_pattern(".env", ".env"));
        assert!(matches_pattern("path/to/.env", ".env"));
    }

    #[test]
    fn test_matches_pattern_wildcard() {
        assert!(matches_pattern("debug.log", "*.log"));
        assert!(matches_pattern("logs/error.log", "*.log"));
        assert!(matches_pattern("secret_key.txt", "secret_*.txt"));
    }

    #[test]
    fn test_no_match() {
        assert!(!matches_pattern("main.rs", "*.log"));
        assert!(!matches_pattern("env_backup", ".env"));
    }
}

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

fn visit(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit(&path, files)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("gitignore") {
            files.push(path);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let templates_dir = manifest_dir.join("src").join("gitignore");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let dest = out_dir.join("generated_templates.rs");

    let mut output = File::create(dest)?;

    writeln!(output, "pub static TEMPLATES: &[(&str, &str)] = &[")?;

    let mut files = Vec::new();
    visit(&templates_dir, &mut files)?;
    files.sort();

    for path in files {
        let relative = path.strip_prefix(&templates_dir)?;

        let name = relative
            .to_string_lossy()
            .replace('\\', "/")
            .trim_end_matches(".gitignore")
            .to_string();

        writeln!(
            output,
            "    ({:?}, include_str!({:?})),",
            name,
            path.display().to_string(),
        )?;
    }

    writeln!(output, "];")?;

    println!("cargo:rerun-if-changed=src/gitignore");

    Ok(())
}

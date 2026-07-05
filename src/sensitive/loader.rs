use anyhow::Result;
use std::fs;

use crate::sensitive::model::PatternsFile;

pub fn load_patterns() -> Result<PatternsFile> {
    let data = fs::read_to_string("src/sensitive/patterns.json")?;
    let parsed: PatternsFile = serde_json::from_str(&data)?;
    Ok(parsed)
}

use anyhow::Result;

use crate::generated::SENSITIVE_PATTERNS;
use crate::sensitive::model::PatternsFile;

pub fn load_patterns() -> Result<PatternsFile> {
    let parsed: PatternsFile = serde_json::from_str(SENSITIVE_PATTERNS)?;
    Ok(parsed)
}

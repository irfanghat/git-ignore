use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PatternsFile {
    pub version: String,
    pub description: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
    pub label: String,
    pub severity: String,
    pub reason: String,
    pub patterns: Vec<String>,
}

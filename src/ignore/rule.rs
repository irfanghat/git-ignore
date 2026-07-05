#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub pattern: String,
    pub negated: bool,
}

impl Rule {
    pub fn new(pattern: impl Into<String>) -> Self {
        let pattern = pattern.into();

        Self {
            negated: pattern.starts_with('!'),
            pattern,
        }
    }
}

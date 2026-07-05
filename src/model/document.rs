pub struct Rule {
    pub pattern: String,
    pub negated: bool,
}

pub enum Entry {
    Rule(Rule),
    Comment(String),
    Blank,
}

pub struct IgnoreDocument {
    pub entries: Vec<Entry>,
}

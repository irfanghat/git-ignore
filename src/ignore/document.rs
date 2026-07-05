use std::collections::HashSet;

use crate::ignore::rule::Rule;

#[derive(Debug, Clone)]
pub struct Comment {
    pub text: String,
}

impl Comment {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Debug, Clone)]
pub enum Entry {
    Rule(Rule),
    Comment(Comment),
    Blank,
}

#[derive(Debug, Clone, Default)]
pub struct IgnoreDocument {
    pub entries: Vec<Entry>,
}

impl IgnoreDocument {
    /// Returns true if a rule already exists.
    pub fn contains(&self, pattern: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| matches!(entry, Entry::Rule(rule) if rule.pattern == pattern))
    }

    /// Returns all rules.
    pub fn rules(&self) -> Vec<&Rule> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Rule(rule) => Some(rule),
                _ => None,
            })
            .collect()
    }

    /// Returns all comments.
    pub fn comments(&self) -> Vec<&Comment> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Comment(comment) => Some(comment),
                _ => None,
            })
            .collect()
    }

    /// Adds a new rule.
    /// Returns true if the rule was added.
    pub fn add(&mut self, pattern: impl Into<String>) -> bool {
        let pattern = pattern.into();

        if self.contains(&pattern) {
            return false;
        }

        self.entries.push(Entry::Rule(Rule::new(pattern)));
        true
    }

    /// Removes a rule.
    /// Returns true if anything was removed.
    pub fn remove(&mut self, pattern: &str) -> bool {
        let before = self.entries.len();

        self.entries
            .retain(|entry| !matches!(entry, Entry::Rule(rule) if rule.pattern == pattern));

        before != self.entries.len()
    }

    /// Removes duplicate rules.
    /// Preserves comments and blank lines.
    pub fn dedupe(&mut self) -> usize {
        let mut seen = HashSet::new();
        let mut removed = 0;

        self.entries.retain(|entry| match entry {
            Entry::Rule(rule) => {
                if seen.insert(rule.pattern.clone()) {
                    true
                } else {
                    removed += 1;
                    false
                }
            }
            _ => true,
        });

        removed
    }

    /// Sorts only rules alphabetically.
    /// Comments and blank lines remain where they are.
    pub fn sort(&mut self) {
        let mut rules: Vec<Rule> = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Rule(rule) => Some(rule.clone()),
                _ => None,
            })
            .collect();

        rules.sort_by(|a, b| a.pattern.cmp(&b.pattern));

        let mut sorted = rules.into_iter();

        for entry in &mut self.entries {
            if matches!(entry, Entry::Rule(_)) {
                *entry = Entry::Rule(sorted.next().unwrap());
            }
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.rules().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn rule_count(&self) -> usize {
        self.rules().len()
    }
}

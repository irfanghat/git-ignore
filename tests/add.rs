use super::*;

#[test]
fn adding_duplicate_returns_false() {
    let mut doc = IgnoreDocument::default();

    assert!(doc.add(".env"));
    assert!(!doc.add(".env"));
}

#[test]
fn removing_rule() {
    let mut doc = IgnoreDocument::default();

    doc.add(".env");

    assert!(doc.remove(".env"));
    assert!(!doc.contains(".env"));
}

//! The library answers for exactly the languages the table names.
//!
//! A language lives in two files: the table (`crates/syntax/src/language.rs`), which every
//! build has, and this library's match, which only the application's release builds use. A
//! language added to the table and not here would open as plain text and say nothing.

/// The ids `lang("...")` of the table, in the order they appear.
fn table_ids() -> Vec<String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../syntax/src/language.rs");
    let text = std::fs::read_to_string(path).expect("the language table reads");
    let mut ids = Vec::new();
    let mut rest = text.as_str();
    while let Some(at) = rest.find("lang(") {
        rest = &rest[at + "lang(".len()..];
        let after = rest.trim_start();
        let Some(id) = after.strip_prefix('"') else {
            continue;
        };
        let Some(end) = id.find('"') else { continue };
        ids.push(id[..end].to_owned());
    }
    ids
}

/// The ids this library answers for, the arms of `birchpad_grammar`.
fn answered_ids() -> Vec<String> {
    let text = include_str!("../src/lib.rs");
    let mut ids = Vec::new();
    for line in text.lines() {
        let line = line.trim_start();
        let Some(id) = line.strip_prefix('"') else {
            continue;
        };
        let Some(end) = id.find('"') else { continue };
        if id[end + 1..].trim_start().starts_with("=>") {
            ids.push(id[..end].to_owned());
        }
    }
    ids
}

#[test]
fn every_language_of_the_table_has_a_grammar_here() {
    let table = table_ids();
    let answered = answered_ids();
    assert!(table.len() > 40, "{} languages found", table.len());
    let missing: Vec<&String> = table.iter().filter(|id| !answered.contains(id)).collect();
    assert!(
        missing.is_empty(),
        "the library has no grammar for {missing:?}; add it to birchpad_grammar"
    );
    let extra: Vec<&String> = answered.iter().filter(|id| !table.contains(id)).collect();
    assert!(
        extra.is_empty(),
        "the library answers for {extra:?}, which the table does not name"
    );
}

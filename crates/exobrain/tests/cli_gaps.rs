//! Each verb of the binary against the feature document of the canon. The
//! test names carry the requirement of `_docs/specs/SPEC-cli.md` they verify
//! as a prefix, so its `Verify:` filter selects the whole family.

#![allow(clippy::unwrap_used, reason = "a test aborts on a broken fixture")]

use std::path::Path;

use clap::CommandFactory;
use exobrain::cli::Cli;

const DOCUMENT: &str = "programming/project-bootstrap/COMPARISON-dev-tool-features.md";

fn document() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(DOCUMENT)).unwrap()
}

/// The name of each subcommand of the binary.
fn verbs() -> Vec<String> {
    Cli::command()
        .get_subcommands()
        .map(|c| c.get_name().to_owned())
        .collect()
}

/// Each verb that no table row of `doc` names as `exobrain <verb>`.
fn verbs_without_a_gap(doc: &str) -> Vec<String> {
    verbs()
        .into_iter()
        .filter(|verb| {
            let needle = format!("`exobrain {verb}");
            !doc.lines()
                .any(|line| line.starts_with('|') && line.contains(&needle))
        })
        .collect()
}

#[test]
fn a_verb_fills_a_named_gap_in_the_document() {
    assert!(!verbs().is_empty());
    assert_eq!(verbs_without_a_gap(&document()), Vec::<String>::new());
}

#[test]
fn a_verb_fills_a_named_gap_fails_when_a_row_drops() {
    let doc = document()
        .lines()
        .filter(|line| !line.contains("`exobrain host-check"))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(verbs_without_a_gap(&doc), vec!["host-check".to_owned()]);
}

//! `exobrain skill` against the package folders under `skills/`. The test
//! names carry the requirement of `_docs/specs/SPEC-cli.md` they verify as
//! a prefix, so its `Verify:` filter selects the whole family.

#![allow(clippy::unwrap_used, reason = "a test aborts on a broken fixture")]

use std::path::{Path, PathBuf};

use assert_cmd::Command;

fn exobrain(args: &[&str]) -> std::process::Output {
    Command::cargo_bin("exobrain")
        .unwrap()
        .args(args)
        .output()
        .unwrap()
}

fn skills_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("skills")
}

/// Every file under `dir`, as a path relative to `dir`, sorted.
fn files_under(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(next) = stack.pop() {
        for entry in std::fs::read_dir(&next).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path.strip_prefix(dir).unwrap();
                out.push(rel.to_str().unwrap().to_owned());
            }
        }
    }
    out.sort();
    out
}

fn folder_names() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(skills_dir())
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

#[test]
fn skill_prints_the_embedded_skills_list_names_each_folder() {
    let out = exobrain(&["skill", "list"]);
    assert!(out.status.success());
    let listed: Vec<String> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(listed, folder_names());
    assert!(out.stderr.is_empty());
}

#[test]
fn skill_prints_the_embedded_skills_show_prints_skill_md() {
    for name in folder_names() {
        let out = exobrain(&["skill", "show", &name]);
        assert!(out.status.success(), "{name}");
        let on_disk = std::fs::read(skills_dir().join(&name).join("SKILL.md")).unwrap();
        assert_eq!(out.stdout, on_disk, "{name}");
        assert!(
            String::from_utf8(out.stdout)
                .unwrap()
                .starts_with(&format!("---\nname: {name}\n")),
            "{name}"
        );
    }
}

#[test]
fn skill_prints_the_embedded_skills_every_file_of_the_folder() {
    for name in folder_names() {
        let dir = skills_dir().join(&name);
        for rel in files_under(&dir) {
            let out = exobrain(&["skill", "show", &name, "--file", &rel]);
            assert!(out.status.success(), "{name}: {rel} is not embedded");
            assert_eq!(out.stdout, std::fs::read(dir.join(&rel)).unwrap(), "{rel}");
        }
    }
}

#[test]
fn skill_prints_the_embedded_skills_refuses_an_unknown_name() {
    let out = exobrain(&["skill", "show", "no-such-skill"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    let err = String::from_utf8(out.stderr).unwrap();
    assert!(err.contains("no-such-skill"), "{err}");
    assert!(err.contains("exobrain skill list"), "{err}");
}

#[test]
fn skill_prints_the_embedded_skills_refuses_an_unknown_file() {
    let out = exobrain(&[
        "skill",
        "show",
        "exobrain-hooks-setup",
        "--file",
        "references/none.md",
    ]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    let err = String::from_utf8(out.stderr).unwrap();
    assert!(err.contains("references/none.md"), "{err}");
    assert!(err.contains("references/runner.md"), "{err}");
}

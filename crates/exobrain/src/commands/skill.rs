//! `exobrain skill`: print the agent skills that the binary carries.
//!
//! Each skill is a package folder under `crates/exobrain/skills/<name>/`:
//! a `SKILL.md` and the files it names, such as `references/<task>.md`.
//! The binary embeds every file at compile time, so a project reads the
//! skill of the CLI on its host with no checkout of exobrain-tech. A test
//! holds the table below equal to the folder.

use crate::cli::skill::{ShowArgs, SkillArgs, SkillCommand};
use crate::error::{Error, USAGE};
use crate::output;

/// The file that `show` prints when the call names none.
pub const ENTRY: &str = "SKILL.md";

/// One embedded skill: its name, and each file of its package with its
/// path relative to the package folder.
pub struct Skill {
    /// The name, which is also the name of the package folder.
    pub name: &'static str,
    /// Each file of the package, as `(path, contents)`.
    pub files: &'static [(&'static str, &'static str)],
}

/// Every skill that the binary carries.
pub const SKILLS: &[Skill] = &[Skill {
    name: "exobrain-hooks-setup",
    files: &[
        (
            "SKILL.md",
            include_str!("../../skills/exobrain-hooks-setup/SKILL.md"),
        ),
        (
            "references/gate.md",
            include_str!("../../skills/exobrain-hooks-setup/references/gate.md"),
        ),
        (
            "references/host-cli.md",
            include_str!("../../skills/exobrain-hooks-setup/references/host-cli.md"),
        ),
        (
            "references/runner.md",
            include_str!("../../skills/exobrain-hooks-setup/references/runner.md"),
        ),
        (
            "references/sets.md",
            include_str!("../../skills/exobrain-hooks-setup/references/sets.md"),
        ),
    ],
}];

/// Run one skill command, returning the exit code.
///
/// # Errors
///
/// None: an unknown skill or file prints its reason and exits 2.
pub fn run(args: &SkillArgs) -> Result<u8, Error> {
    match &args.command {
        SkillCommand::List => {
            let names: String = SKILLS.iter().flat_map(|s| [s.name, "\n"]).collect();
            output::result(&names);
            Ok(0)
        }
        SkillCommand::Show(show_args) => Ok(show(show_args)),
    }
}

fn show(args: &ShowArgs) -> u8 {
    let Some(skill) = SKILLS.iter().find(|s| s.name == args.name) else {
        output::note(&format!(
            "exobrain: no skill named {:?}; `exobrain skill list` names each one",
            args.name
        ));
        return USAGE;
    };
    let path = args.file.as_deref().unwrap_or(ENTRY);
    let Some((_, text)) = skill.files.iter().find(|(p, _)| *p == path) else {
        let known: Vec<&str> = skill.files.iter().map(|(p, _)| *p).collect();
        output::note(&format!(
            "exobrain: skill {} holds no file {path:?}; it holds {}",
            skill.name,
            known.join(", ")
        ));
        return USAGE;
    };
    output::result(text);
    0
}

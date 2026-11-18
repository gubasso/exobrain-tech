//! Arguments of `exobrain skill`.

use clap::{Args, Subcommand};

/// Print the agent skills that the binary carries.
#[derive(Debug, Args)]
pub struct SkillArgs {
    /// Which skill command to run.
    #[command(subcommand)]
    pub command: SkillCommand,
}

/// Every command of `exobrain skill`.
#[derive(Debug, Subcommand)]
pub enum SkillCommand {
    /// List the skills that the binary carries.
    List,
    /// Print one skill's `SKILL.md`, or one file of its package.
    Show(ShowArgs),
}

/// Arguments of `exobrain skill show`.
#[derive(Debug, Args)]
pub struct ShowArgs {
    /// The skill's name.
    #[arg(value_name = "NAME")]
    pub name: String,
    /// One file of the package, relative to the skill's folder, such as
    /// `references/runner.md`.
    #[arg(long, value_name = "PATH")]
    pub file: Option<String>,
}

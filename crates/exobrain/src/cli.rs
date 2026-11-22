//! Command-line surface: clap derive types only, one module per
//! subcommand's arguments. No behavior lives here.

pub mod hooks;
pub mod host_check;
pub mod outdated;
pub mod skill;
pub mod store_guard;

use clap::{Parser, Subcommand};

/// The exobrain-tech CLI: guards that enforce the rules of the knowledge base.
#[derive(Debug, Parser)]
#[command(name = "exobrain", version, about, propagate_version = true)]
pub struct Cli {
    /// Which subcommand to run.
    #[command(subcommand)]
    pub command: Commands,
}

/// Every subcommand the binary offers.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check the host tools that host-requirements.toml lists.
    HostCheck(host_check::HostCheckArgs),
    /// Hold the hook revisions of a project to its exobrain-tech tag.
    Hooks(hooks::HooksArgs),
    /// Report what is behind, change nothing, and always exit 0 (for `.envrc`).
    ///
    /// Each lock whose nixpkgs pin is over 30 days old prints one line on
    /// stderr. The limit is fixed. Each root input pinned at a version tag
    /// of a GitHub repository with a newer tag prints one line too. The
    /// latest tag of each repository is read through `gh` once a week. A
    /// newer exobrain CLI, and the hook revisions of a newer exobrain-tech
    /// tag that `.pre-commit-config.yaml` selects, print one line each.
    Outdated(outdated::OutdatedArgs),
    /// Print the agent skills that the binary carries.
    Skill(skill::SkillArgs),
    /// Find copies of this checkout in the Nix store.
    StoreGuard(store_guard::StoreGuardArgs),
}

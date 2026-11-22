//! Arguments of `exobrain hooks`.

use std::path::PathBuf;

use clap::{Args, Subcommand};

/// The config that `sync` reads when a call names none.
pub const DEFAULT_CONFIG: &str = ".pre-commit-config.yaml";

/// Hold the hook revisions of a project to the reference sets of its
/// exobrain-tech tag.
#[derive(Debug, Args)]
pub struct HooksArgs {
    /// Which hooks command to run.
    #[command(subcommand)]
    pub command: HooksCommand,
}

/// Every command of `exobrain hooks`.
#[derive(Debug, Subcommand)]
pub enum HooksCommand {
    /// Compare the revisions of a config with the named sets, offline.
    Check(CheckArgs),
    /// Move a config to the sets of an exobrain-tech tag.
    Sync(SyncArgs),
}

/// Arguments of `exobrain hooks check`.
#[derive(Debug, Args)]
pub struct CheckArgs {
    /// The folder that holds the set files, `<name>.yaml` each.
    #[arg(long, value_name = "DIR")]
    pub sets_dir: PathBuf,
    /// A set to compare with, repeatable.
    #[arg(long = "set", value_name = "NAME", required = true)]
    pub sets: Vec<String>,
    /// The prek config of the project.
    #[arg(value_name = "CONFIG")]
    pub config: PathBuf,
}

/// Arguments of `exobrain hooks sync`.
#[derive(Debug, Args)]
pub struct SyncArgs {
    /// The exobrain-tech tag to move to. With none, the latest release.
    #[arg(long, value_name = "TAG")]
    pub to: Option<String>,
    /// The prek config of the project.
    #[arg(value_name = "CONFIG", default_value = DEFAULT_CONFIG)]
    pub config: PathBuf,
}

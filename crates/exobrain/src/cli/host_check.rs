//! Arguments of `exobrain host-check`.

use clap::Args;

/// Check the host tools that `host-requirements.toml` lists.
#[derive(Debug, Args)]
pub struct HostCheckArgs {
    /// Validate the file and check nothing on the host.
    #[arg(long, conflicts_with = "tools")]
    pub parse_only: bool,
    /// The tools to check. With none, every tool of the file.
    #[arg(value_name = "TOOL")]
    pub tools: Vec<String>,
}

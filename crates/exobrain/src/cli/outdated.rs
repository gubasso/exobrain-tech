//! Arguments of `exobrain outdated`.

use std::path::PathBuf;

use clap::Args;

/// The lock that a call checks when it names none.
pub const DEFAULT_LOCK: &str = "flake.lock";

/// The arguments of `exobrain outdated`. The variant of `Commands` holds
/// the help text.
#[derive(Debug, Args)]
pub struct OutdatedArgs {
    /// The flake locks to check, each on its own. With none, `flake.lock`,
    /// and a folder with no `flake.lock` stays silent.
    #[arg(value_name = "LOCK")]
    pub locks: Vec<PathBuf>,
    /// Read the network now for the checks that cache a network read. The
    /// age of the nixpkgs pin reads only the lock.
    #[arg(long)]
    pub refresh: bool,
}

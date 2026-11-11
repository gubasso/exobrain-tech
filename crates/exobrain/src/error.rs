//! The one error type and its exit code.

use thiserror::Error;

/// The exit code of a usage or environment error.
pub const USAGE: u8 = 2;

/// A failure that stops a command.
#[derive(Debug, Error)]
pub enum Error {
    /// The working folder is not inside a git work tree.
    #[error("not inside a git work tree: {0}")]
    NotGit(String),
    /// A filesystem read failed.
    #[error("{what}: {source}")]
    Io {
        /// What the command was reading.
        what: String,
        /// The underlying failure.
        source: std::io::Error,
    },
    /// The `nix path-info` query failed.
    #[error("nix path-info failed: {0}")]
    Nix(String),
}

impl Error {
    /// The process exit code: every error is a usage or environment error.
    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        USAGE
    }
}

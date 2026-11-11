//! Command-line surface: clap derive types only, one module per
//! subcommand's arguments. No behavior lives here.

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
    /// Find copies of this checkout in the Nix store.
    StoreGuard(store_guard::StoreGuardArgs),
}

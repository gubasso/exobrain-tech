//! Binary entry point.
//!
//! `main` is the process boundary and nothing else: it parses, dispatches,
//! and maps the outcome to an exit code. Behavior lives in `commands`.

use std::process::ExitCode;

use clap::Parser;
use exobrain::cli::{Cli, Commands};
use exobrain::{commands, output};

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            // Help and version are successes that clap reports through
            // its error type. A real usage error exits 2.
            let _ = err.print();
            return if err.exit_code() == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(exobrain::error::USAGE)
            };
        }
    };
    let result = match &cli.command {
        Commands::StoreGuard(args) => commands::store_guard::run(args),
    };
    match result {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            output::error(&e);
            ExitCode::from(e.exit_code())
        }
    }
}

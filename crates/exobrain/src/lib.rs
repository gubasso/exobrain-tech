//! exobrain: the guards that enforce the rules of the exobrain-tech
//! knowledge base.
//!
//! The library exists so the crate's own tests can link the modules; the
//! `exobrain` binary in `main.rs` is the product. `cli` holds the argument
//! surface, `commands` the handlers, `store` the store listing and the
//! `nix path-info` query, `origin` the checkout and the attribution of a
//! copy, `stamp` the daily stamp and its lock, `switch` the off switches,
//! `output` the one place that prints, and `error` the exit codes.

pub mod cli;
pub mod commands;
pub mod error;
pub mod origin;
pub mod output;
pub mod stamp;
pub mod store;
pub mod switch;

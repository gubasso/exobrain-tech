//! The one place that prints.
//!
//! Stdout carries the result, human text by
//! default or one JSON object under `--json`; stderr carries everything
//! else. A failed write is ignored: the exit code already carries the
//! outcome.

use std::fmt::Write as _;
use std::io::Write as _;

use crate::commands::store_guard::Report;
use crate::error::Error;

/// The rule that a store copy breaks.
pub const RULE: &str = "project-platform:a-flake-is-read-from-its-git-tree";

/// The remedy line of every report.
fn remedy() -> String {
    format!("run `nix store gc` once no process uses these paths ({RULE})")
}

fn to_stdout(text: &str) {
    let _ = std::io::stdout().lock().write_all(text.as_bytes());
}

/// Write one line to stderr.
pub fn note(line: &str) {
    let _ = writeln!(std::io::stderr(), "{line}");
}

/// Render a failure on stderr.
pub fn error(error: &Error) {
    note(&format!("exobrain: {error}"));
}

/// A size in decimal megabytes, one decimal.
#[must_use]
pub fn megabytes(bytes: u64) -> String {
    let tenths = bytes.saturating_add(50_000) / 100_000;
    format!("{}.{} MB", tenths / 10, tenths % 10)
}

/// A registration time as a local date.
fn local_date(seconds: i64) -> String {
    jiff::Timestamp::from_second(seconds).map_or_else(
        |_| seconds.to_string(),
        |ts| ts.to_zoned(jiff::tz::TimeZone::system()).date().to_string(),
    )
}

/// Print the operator report, human or JSON.
pub fn report(report: &Report, json: bool) {
    if json {
        let text = serde_json::to_string_pretty(report).unwrap_or_default();
        to_stdout(&format!("{text}\n"));
        return;
    }
    let mut out = String::new();
    let checkout = report.checkout.display();
    if report.copies.is_empty() {
        let _ = writeln!(out, "no store copy of {checkout}");
    } else {
        let _ = writeln!(out, "store copies of {checkout}:");
        for copy in &report.copies {
            let _ = writeln!(
                out,
                "  {}  {}  {}  {}",
                copy.path,
                megabytes(copy.nar_size),
                local_date(copy.registered),
                copy.origin
            );
        }
        let _ = writeln!(
            out,
            "total: {} in {} {}",
            megabytes(report.total_nar_size),
            report.copies.len(),
            if report.copies.len() == 1 {
                "copy"
            } else {
                "copies"
            }
        );
        let _ = writeln!(out, "{}", remedy());
    }
    if !report.unattributed.is_empty() {
        let _ = writeln!(out, "origin unknown:");
        for entry in &report.unattributed {
            let _ = writeln!(out, "  {}  {}", entry.path, entry.reason);
        }
    }
    to_stdout(&out);
}

/// The three-line warning of `--daily` when copies exist.
pub fn daily_warning(report: &Report) {
    note(&format!(
        "exobrain store-guard: {} store {} of this checkout hold {}",
        report.copies.len(),
        if report.copies.len() == 1 {
            "copy"
        } else {
            "copies"
        },
        megabytes(report.total_nar_size)
    ));
    note("exobrain store-guard: run `exobrain store-guard` for the list");
    note(&format!("exobrain store-guard: {}", remedy()));
}

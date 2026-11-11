//! `exobrain store-guard`: find the copies of this checkout in the Nix
//! store, which a flake reference that does not read the git tree leaves
//! behind.

use camino::Utf8PathBuf;
use serde::Serialize;

use crate::cli::store_guard::StoreGuardArgs;
use crate::error::Error;
use crate::origin::{self, Attribution, Checkout};
use crate::stamp::{self, Acquired};
use crate::{output, store, switch};

/// One store copy of this checkout.
#[derive(Debug, Clone, Serialize)]
pub struct StoreCopy {
    /// The store path.
    pub path: Utf8PathBuf,
    /// The registered NAR size in bytes.
    pub nar_size: u64,
    /// The registration time, in seconds since the epoch.
    #[serde(serialize_with = "rfc3339")]
    pub registered: i64,
    /// Where the copy came from.
    pub origin: String,
}

/// A copy whose origin cannot be decided.
#[derive(Debug, Clone, Serialize)]
pub struct Unattributed {
    /// The store path.
    pub path: Utf8PathBuf,
    /// Why the origin is unknown.
    pub reason: String,
}

/// What one scan found.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    /// The canonical top level of the checkout.
    pub checkout: std::path::PathBuf,
    /// The copies of this checkout at or over the minimum size.
    pub copies: Vec<StoreCopy>,
    /// The copies whose origin is unknown.
    pub unattributed: Vec<Unattributed>,
    /// The sum of the copies' NAR sizes.
    pub total_nar_size: u64,
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde's serialize_with passes the field by reference"
)]
fn rfc3339<S: serde::Serializer>(seconds: &i64, serializer: S) -> Result<S::Ok, S::Error> {
    match jiff::Timestamp::from_second(*seconds) {
        Ok(ts) => serializer.collect_str(&ts),
        Err(_) => serializer.serialize_i64(*seconds),
    }
}

/// Run the verb, returning the exit code.
///
/// # Errors
///
/// In operator mode, a folder outside git, an unreadable store, or a
/// failing `nix path-info`. `--daily` never fails.
pub fn run(args: &StoreGuardArgs) -> Result<u8, Error> {
    if args.daily {
        daily(args.min_size);
        return Ok(0);
    }
    let checkout = origin::checkout()?;
    let report = scan(&checkout, args.min_size)?;
    output::report(&report, args.json);
    Ok(u8::from(!report.copies.is_empty()))
}

/// The `.envrc` mode: switched off in CI or by the operator, at most one
/// scan a day per checkout, quiet on a clean store, and never a failure.
fn daily(min_size: u64) {
    if switch::switched_off() || switch::in_ci() {
        return;
    }
    let Ok(checkout) = origin::checkout() else {
        return;
    };
    let Some(dir) = stamp::state_dir() else {
        output::note("exobrain store-guard: neither XDG_STATE_HOME nor HOME is set; skipped");
        return;
    };
    let key = stamp::key(&checkout.root);
    let lock = match std::fs::create_dir_all(&dir)
        .map(|()| stamp::acquire(&dir.join(format!("{key}.lock"))))
    {
        Ok(Acquired::Held(lock)) => lock,
        Ok(Acquired::Contended) => return,
        Ok(Acquired::Unavailable(e)) | Err(e) => {
            output::note(&format!(
                "exobrain store-guard: state folder {} is not writable ({e}); skipped",
                dir.display()
            ));
            return;
        }
    };
    let stamp_path = dir.join(format!("{key}.stamp"));
    if stamp::stamped_today(&stamp_path) {
        return;
    }
    if let Err(e) = stamp::write_stamp(&stamp_path) {
        output::note(&format!(
            "exobrain store-guard: cannot write {} ({e}); skipped",
            stamp_path.display()
        ));
        return;
    }
    match scan(&checkout, min_size) {
        Ok(report) if !report.copies.is_empty() => output::daily_warning(&report),
        Ok(_) => {}
        Err(e) => output::note(&format!("exobrain store-guard: {e}")),
    }
    drop(lock);
}

/// List, attribute, and measure the store copies of `checkout`.
///
/// # Errors
///
/// An unreadable store or a failing `nix path-info`.
pub fn scan(checkout: &Checkout, min_size: u64) -> Result<Report, Error> {
    let mut ours = Vec::new();
    let mut unattributed = Vec::new();
    for candidate in store::candidates()? {
        match origin::attribute(&candidate, checkout) {
            Attribution::Ours(origin) => ours.push((candidate.path, origin)),
            Attribution::Unknown(reason) => unattributed.push(Unattributed {
                path: candidate.path,
                reason,
            }),
            Attribution::Foreign => {}
        }
    }
    let paths: Vec<Utf8PathBuf> = ours.iter().map(|(path, _)| path.clone()).collect();
    let info = store::path_info(&paths)?;
    let copies: Vec<StoreCopy> = ours
        .into_iter()
        .filter_map(|(path, origin)| {
            let found = info.get(&path)?;
            (found.nar_size >= min_size).then_some(StoreCopy {
                nar_size: found.nar_size,
                registered: found.registration_time,
                origin,
                path,
            })
        })
        .collect();
    let total_nar_size = copies.iter().map(|copy| copy.nar_size).sum();
    Ok(Report {
        checkout: checkout.root.clone(),
        copies,
        unattributed,
        total_nar_size,
    })
}

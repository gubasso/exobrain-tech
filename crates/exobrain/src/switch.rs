//! The off switches of `store-guard --daily`, with release-kit's
//! semantics: the operator's switch counts only as `0`, and a CI variable
//! counts unless it is empty, `0`, or `false`.

/// The variables a CI runner exports; any of them switches the guard off.
pub const CI_VARS: [&str; 6] = [
    "CI",
    "GITHUB_ACTIONS",
    "GITLAB_CI",
    "BUILDKITE",
    "CIRCLECI",
    "TF_BUILD",
];

/// The operator's own off switch.
pub const SWITCH_VAR: &str = "EXOBRAIN_STORE_GUARD";

/// Whether the operator switched the guard off with `EXOBRAIN_STORE_GUARD=0`.
#[must_use]
pub fn switched_off() -> bool {
    std::env::var(SWITCH_VAR).is_ok_and(|value| value.trim() == "0")
}

/// Whether a CI variable is set to anything but an explicit no.
#[must_use]
pub fn in_ci() -> bool {
    CI_VARS.iter().any(|var| {
        std::env::var(var).is_ok_and(|value| {
            let value = value.trim().to_ascii_lowercase();
            !(value.is_empty() || value == "0" || value == "false")
        })
    })
}

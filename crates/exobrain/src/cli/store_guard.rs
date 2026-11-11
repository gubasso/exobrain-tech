//! Arguments of `exobrain store-guard`.

use clap::Args;

/// Find the store copies that a `path:` style flake reference left behind.
#[derive(Debug, Args)]
pub struct StoreGuardArgs {
    /// Run at most once a day, quietly, and always exit 0 (for `.envrc`).
    #[arg(long, conflicts_with = "json")]
    pub daily: bool,
    /// Print one JSON object instead of the report.
    #[arg(long)]
    pub json: bool,
    /// Ignore copies smaller than this NAR size (bytes, or with a K, M, or G suffix).
    #[arg(long, default_value = "50M", value_parser = parse_size)]
    pub min_size: u64,
}

/// Parse a decimal size: plain bytes, or a K, M, or G suffix (powers of
/// 1000), with an optional trailing `B`.
///
/// # Errors
///
/// A message clap shows when the value is not a size.
pub fn parse_size(text: &str) -> Result<u64, String> {
    let text = text.trim();
    let text = text
        .strip_suffix(['B', 'b'])
        .filter(|rest| !rest.is_empty())
        .unwrap_or(text);
    let (digits, factor) = match text.chars().last() {
        Some('K' | 'k') => (&text[..text.len() - 1], 1_000),
        Some('M' | 'm') => (&text[..text.len() - 1], 1_000_000),
        Some('G' | 'g') => (&text[..text.len() - 1], 1_000_000_000),
        _ => (text, 1),
    };
    digits
        .parse::<u64>()
        .ok()
        .and_then(|n| n.checked_mul(factor))
        .ok_or_else(|| format!("`{text}` is not a size such as 50M or 1000000"))
}

#[cfg(test)]
mod tests {
    use super::parse_size;

    #[test]
    fn sizes_parse_with_and_without_a_suffix() {
        assert_eq!(parse_size("50M"), Ok(50_000_000));
        assert_eq!(parse_size("50MB"), Ok(50_000_000));
        assert_eq!(parse_size("0"), Ok(0));
        assert_eq!(parse_size("2k"), Ok(2_000));
        assert!(parse_size("M").is_err());
        assert!(parse_size("ten").is_err());
    }
}

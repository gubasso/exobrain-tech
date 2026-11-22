//! `exobrain hooks`: hold the hook revisions of a project to the
//! reference sets of its exobrain-tech tag.
//!
//! `check` reads only local files. `sync` reads GitHub through
//! the host `gh`, through the bounded call of the release cache. The set
//! fetch, the set cache, and the comparison with the sets of a tag serve
//! `exobrain outdated` too.
//!
//! The config parse reads the block style that prek documents and
//! `prek update` keeps: a top-level `repos:` block sequence whose entries
//! hold one-line `repo:` and `rev:` values. It records the byte span of
//! each value, so `sync` rewrites a `rev` and keeps every other byte. A
//! layout outside that style, such as a flow sequence or an escaped
//! string, fails the parse rather than reach a wrong answer.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::cli::hooks::{CheckArgs, HooksArgs, HooksCommand, SyncArgs};
use crate::error::{Error, USAGE};
use crate::{output, probe, release_cache, stamp};

/// The exit code of `check` when a revision differs, and of `sync` on
/// any error.
pub const FAILED: u8 = 1;

/// The folder of the set files inside an exobrain-tech tag.
pub const SETS_PATH: &str = "programming/project-bootstrap/hook-sets";

/// The hook whose `args` name the sets of a project.
pub const HOOK_ID: &str = "hook-revisions";

/// The name of the repository whose `rev` selects the sets.
const EXOBRAIN_TECH: &str = "exobrain-tech";

/// The repositories that prek resolves without a clone, and so without a
/// `rev`.
const LOCAL_REPOS: [&str; 3] = ["local", "meta", "builtin"];

/// The markers of a block that release-kit or sdd renders.
const RENDERED: [(&str, &str); 2] = [
    ("# BEGIN release-kit", "# END release-kit"),
    ("# BEGIN spec-driven-docs", "# END spec-driven-docs"),
];

/// Run one hooks command, returning the exit code.
///
/// # Errors
///
/// `check` fails when it cannot read the config. `sync` reports its own
/// failures and never returns an error.
pub fn run(args: &HooksArgs) -> Result<u8, Error> {
    match &args.command {
        HooksCommand::Check(args) => check(args),
        HooksCommand::Sync(args) => Ok(sync(args)),
    }
}

// --- The config parse ---

/// One scalar value, with the byte span of its text inside any quotes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scalar {
    /// The value, unquoted.
    pub text: String,
    /// The bytes of the value in the file, inside any quotes.
    pub span: Range<usize>,
}

/// One hook of an entry: the fields that `sync` reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hook {
    /// The hook id.
    pub id: String,
    /// The hook arguments.
    pub args: Vec<String>,
}

/// One entry of `repos:`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The repository URL, or `local`, `meta`, or `builtin`.
    pub repo: Scalar,
    /// The pinned revision; a remote repository always has one.
    pub rev: Option<Scalar>,
    /// The hooks of the entry.
    pub hooks: Vec<Hook>,
    /// The line of the entry, from 1.
    pub line: usize,
    /// Whether a rendered block holds the entry.
    pub rendered: bool,
}

impl Entry {
    fn remote(&self) -> bool {
        !LOCAL_REPOS.contains(&self.repo.text.as_str())
    }

    fn holds(&self, id: &str) -> bool {
        self.hooks.iter().any(|hook| hook.id == id)
    }

    /// The exobrain-tech entry: the one that holds `hook-revisions`, or
    /// any entry of a repository named `exobrain-tech`.
    fn is_exobrain_tech(&self) -> bool {
        self.holds(HOOK_ID) || repo_name(&self.repo.text) == EXOBRAIN_TECH
    }
}

/// A parsed prek config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Every entry of `repos:`, in file order.
    pub entries: Vec<Entry>,
}

impl Config {
    /// The entry outside a rendered block that holds `hook-revisions`.
    fn hook_entry(&self) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|entry| !entry.rendered && entry.holds(HOOK_ID))
    }

    /// The exobrain-tech entry outside a rendered block.
    fn exobrain_tech(&self) -> Option<&Entry> {
        self.hook_entry().or_else(|| {
            self.entries
                .iter()
                .find(|entry| !entry.rendered && entry.is_exobrain_tech())
        })
    }
}

/// One line that carries structure: not blank and not only a comment.
#[derive(Debug)]
struct Line<'a> {
    /// The line number, from 1.
    no: usize,
    /// The byte offset of the line in the file.
    start: usize,
    /// The line without its line ending.
    raw: &'a str,
    /// The count of leading spaces.
    indent: usize,
    /// The end of the content, before a comment and trailing blanks.
    end: usize,
    /// Whether a rendered block holds the line.
    rendered: bool,
}

impl Line<'_> {
    fn body(&self) -> &str {
        &self.raw[self.indent..self.end]
    }

    fn is_dash(&self) -> bool {
        let body = self.body();
        body == "-" || body.starts_with("- ")
    }
}

/// The end of the content of `raw` from `from`: before a comment that
/// sits outside quotes, and before trailing blanks.
fn content_end(raw: &str, from: usize) -> usize {
    let bytes = raw.as_bytes();
    let mut quote = None;
    let mut i = from;
    let mut end = raw.len();
    while i < bytes.len() {
        let byte = bytes[i];
        match quote {
            Some(b'"') if byte == b'\\' => i += 1,
            Some(q) if byte == q => quote = None,
            None if byte == b'\'' || byte == b'"' => quote = Some(byte),
            None if byte == b'#'
                && (i == from || bytes[i - 1] == b' ' || bytes[i - 1] == b'\t') =>
            {
                end = i;
                break;
            }
            _ => {}
        }
        i += 1;
    }
    from + raw[from..end].trim_end().len()
}

/// Split the file into its structural lines, and mark the lines of each
/// rendered block.
fn lines(text: &str) -> Result<Vec<Line<'_>>, String> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut block: Option<(&str, usize)> = None;
    for (index, chunk) in text.split_inclusive('\n').enumerate() {
        let no = index + 1;
        let raw = chunk.trim_end_matches(['\n', '\r']);
        let trimmed = raw.trim();
        if let Some((end_marker, _)) = block {
            if trimmed.starts_with(end_marker) {
                block = None;
            }
        } else if let Some((_, end_marker)) = RENDERED
            .iter()
            .find(|(begin, _)| trimmed.starts_with(begin))
        {
            block = Some((end_marker, no));
        } else if RENDERED.iter().any(|(_, end)| trimmed.starts_with(end)) {
            return Err(format!("line {no}: an END marker with no BEGIN marker"));
        }
        let indent = raw.len() - raw.trim_start_matches(' ').len();
        let end = content_end(raw, indent);
        if end > indent && trimmed != "---" {
            if raw[indent..].starts_with('\t') {
                return Err(format!("line {no}: a tab indents the line"));
            }
            out.push(Line {
                no,
                start,
                raw,
                indent,
                end,
                rendered: block.is_some(),
            });
        }
        start += chunk.len();
    }
    match block {
        Some((_, begin)) => Err(format!("line {begin}: a rendered block has no END marker")),
        None => Ok(out),
    }
}

/// The key of a `key: value` line from `from`, and the range of its
/// value, which is empty when the value continues on the next lines.
fn key_at<'a>(line: &Line<'a>, from: usize) -> Option<(&'a str, Range<usize>)> {
    let raw = line.raw;
    let text = &raw[from..line.end];
    let colon = text
        .find(": ")
        .or_else(|| text.ends_with(':').then(|| text.len() - 1))?;
    let key = text[..colon].trim_end();
    if key.is_empty()
        || !key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        return None;
    }
    let after = from + colon + 1;
    let value_start =
        after + (raw[after..line.end].len() - raw[after..line.end].trim_start().len());
    Some((key, value_start..line.end))
}

/// Read one scalar of `line` at `range`: plain, single-quoted, or
/// double-quoted without escapes.
fn scalar(line: &Line<'_>, range: Range<usize>) -> Result<Scalar, String> {
    let no = line.no;
    let text = &line.raw[range.clone()];
    let Some(first) = text.chars().next() else {
        return Err(format!("line {no}: the value is empty"));
    };
    let (value, inner) = match first {
        '\'' | '"' => {
            if text.len() < 2 || !text.ends_with(first) {
                return Err(format!("line {no}: the quoted value does not end"));
            }
            let inner = range.start + 1..range.end - 1;
            let body = &line.raw[inner.clone()];
            if first == '"' && body.contains('\\') {
                return Err(format!("line {no}: an escape in a quoted value"));
            }
            let value = if first == '\'' {
                body.replace("''", "'")
            } else {
                body.to_owned()
            };
            (value, inner)
        }
        '[' | '{' | '&' | '*' | '!' | '|' | '>' | '%' | '@' | '`' => {
            return Err(format!("line {no}: `{text}` is not a one-line plain value"));
        }
        _ => (text.to_owned(), range),
    };
    Ok(Scalar {
        text: value,
        span: line.start + inner.start..line.start + inner.end,
    })
}

/// One item of a block sequence.
struct Item<'l, 'a> {
    /// The lines of the item, the dash line first.
    lines: &'l [Line<'a>],
    /// The column of the keys of the item.
    key_col: usize,
    /// Whether the dash line holds the first key.
    inline: bool,
}

/// The items of a block sequence of mappings.
fn sequence<'l, 'a>(block: &'l [Line<'a>]) -> Result<Vec<Item<'l, 'a>>, String> {
    let Some(first) = block.first() else {
        return Ok(Vec::new());
    };
    let indent = first.indent;
    let mut starts = Vec::new();
    for (i, line) in block.iter().enumerate() {
        if line.indent < indent {
            return Err(format!("line {}: the indentation breaks the list", line.no));
        }
        if line.indent == indent {
            if !line.is_dash() {
                return Err(format!(
                    "line {}: a list item must start with `- `",
                    line.no
                ));
            }
            starts.push(i);
        }
    }
    let mut items = Vec::new();
    for (n, &start) in starts.iter().enumerate() {
        let end = starts.get(n + 1).copied().unwrap_or(block.len());
        let lines = &block[start..end];
        let dash = &lines[0];
        let after = &dash.raw[dash.indent + 1..dash.end];
        let item = if after.trim().is_empty() {
            let Some(next) = lines.get(1) else {
                return Err(format!("line {}: the list item is empty", dash.no));
            };
            Item {
                lines,
                key_col: next.indent,
                inline: false,
            }
        } else {
            Item {
                lines,
                key_col: dash.indent + 1 + (after.len() - after.trim_start().len()),
                inline: true,
            }
        };
        items.push(item);
    }
    Ok(items)
}

/// One `key: value` of a mapping, with the lines nested under it.
struct Field<'l, 'a> {
    line: &'l Line<'a>,
    key: &'a str,
    value: Range<usize>,
    children: &'l [Line<'a>],
}

/// The fields of one sequence item, which must be a mapping.
fn fields<'l, 'a>(item: &Item<'l, 'a>) -> Result<Vec<Field<'l, 'a>>, String> {
    let lines = if item.inline {
        item.lines
    } else {
        &item.lines[1..]
    };
    let mut heads = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        // A dash line at the key column continues the list of the key
        // above it: YAML lets a list under a key share the key's column.
        let head = i == 0 || (line.indent == item.key_col && !line.is_dash());
        if head {
            let from = if i == 0 && item.inline {
                item.key_col
            } else {
                line.indent
            };
            if line.indent > item.key_col && !(i == 0 && item.inline) {
                return Err(format!(
                    "line {}: the indentation breaks the mapping",
                    line.no
                ));
            }
            let Some((key, value)) = key_at(line, from) else {
                return Err(format!("line {}: expected a `key: value` line", line.no));
            };
            heads.push((i, key, value));
        } else if line.indent < item.key_col {
            return Err(format!(
                "line {}: the indentation breaks the mapping",
                line.no
            ));
        }
    }
    let mut out = Vec::new();
    for (n, (i, key, value)) in heads.iter().enumerate() {
        let next = heads.get(n + 1).map_or(lines.len(), |head| head.0);
        if out.iter().any(|field: &Field<'_, '_>| field.key == *key) {
            return Err(format!("line {}: the key `{key}` repeats", lines[*i].no));
        }
        out.push(Field {
            line: &lines[*i],
            key,
            value: value.clone(),
            children: &lines[i + 1..next],
        });
    }
    Ok(out)
}

/// A field whose value is one scalar on its own line.
fn field_scalar(field: &Field<'_, '_>) -> Result<Scalar, String> {
    if !field.children.is_empty() {
        return Err(format!(
            "line {}: `{}` must hold a one-line value",
            field.line.no, field.key
        ));
    }
    scalar(field.line, field.value.clone())
}

/// Split a flow sequence `[a, 'b', "c"]` into its plain values.
fn flow_list(line: &Line<'_>, range: Range<usize>) -> Result<Vec<String>, String> {
    let text = &line.raw[range.clone()];
    if !(text.starts_with('[') && text.ends_with(']')) {
        return Err(format!("line {}: `{text}` is not a list", line.no));
    }
    let inner = range.start + 1..range.end - 1;
    let bytes = line.raw.as_bytes();
    let mut parts = Vec::new();
    let mut quote = None;
    let mut from = inner.start;
    for i in inner.clone() {
        match (quote, bytes[i]) {
            (Some(q), byte) if byte == q => quote = None,
            (None, b'\'' | b'"') => quote = Some(bytes[i]),
            (None, b',') => {
                parts.push(from..i);
                from = i + 1;
            }
            _ => {}
        }
    }
    parts.push(from..inner.end);
    let mut out = Vec::new();
    for part in parts {
        let text = &line.raw[part.clone()];
        let lead = text.len() - text.trim_start().len();
        let trimmed = part.start + lead..part.start + lead + text.trim().len();
        if trimmed.is_empty() {
            continue;
        }
        out.push(scalar(line, trimmed)?.text);
    }
    Ok(out)
}

/// The `args` of a hook: a flow list, or a block list of one-line values.
fn hook_args(field: &Field<'_, '_>) -> Result<Vec<String>, String> {
    if !field.value.is_empty() {
        return flow_list(field.line, field.value.clone());
    }
    field
        .children
        .iter()
        .map(|line| {
            if !line.is_dash() {
                return Err(format!(
                    "line {}: an `args` item must start with `- `",
                    line.no
                ));
            }
            let raw = line.raw;
            let after = line.indent + 1;
            let start =
                after + (raw[after..line.end].len() - raw[after..line.end].trim_start().len());
            scalar(line, start..line.end).map(|value| value.text)
        })
        .collect()
}

/// The hooks of an entry.
fn hooks(field: &Field<'_, '_>) -> Result<Vec<Hook>, String> {
    let value = &field.line.raw[field.value.clone()];
    if value == "[]" {
        return Ok(Vec::new());
    }
    if !value.is_empty() {
        return Err(format!(
            "line {}: write `hooks:` as a block list",
            field.line.no
        ));
    }
    let mut out = Vec::new();
    for item in sequence(field.children)? {
        let mut id = None;
        let mut args = Vec::new();
        for hook_field in fields(&item)? {
            match hook_field.key {
                "id" => id = Some(field_scalar(&hook_field)?.text),
                "args" => args = hook_args(&hook_field)?,
                _ => {}
            }
        }
        let Some(id) = id else {
            return Err(format!("line {}: a hook has no `id`", item.lines[0].no));
        };
        out.push(Hook { id, args });
    }
    Ok(out)
}

/// Parse a prek config.
///
/// # Errors
///
/// A line, and the reason, where the file leaves the block style that
/// this parse reads, or where a remote entry has no `rev`.
pub fn parse(text: &str) -> Result<Config, String> {
    let lines = lines(text)?;
    let top = lines.iter().enumerate().find_map(|(i, line)| {
        let (key, value) = key_at(line, 0)?;
        (line.indent == 0 && key == "repos").then_some((i, value))
    });
    let Some((top, value)) = top else {
        return Err("the config has no top-level `repos:` key".to_owned());
    };
    let value_text = &lines[top].raw[value];
    if value_text == "[]" {
        return Ok(Config {
            entries: Vec::new(),
        });
    }
    if !value_text.is_empty() {
        return Err(format!(
            "line {}: write `repos:` as a block list",
            lines[top].no
        ));
    }
    let rest = &lines[top + 1..];
    let len = rest
        .iter()
        .take_while(|line| line.indent > 0 || line.is_dash())
        .count();
    let mut entries = Vec::new();
    for item in sequence(&rest[..len])? {
        let mut repo = None;
        let mut rev = None;
        let mut entry_hooks = Vec::new();
        for field in fields(&item)? {
            match field.key {
                "repo" => repo = Some(field_scalar(&field)?),
                "rev" => rev = Some(field_scalar(&field)?),
                "hooks" => entry_hooks = hooks(&field)?,
                _ => {}
            }
        }
        let first = &item.lines[0];
        let Some(repo) = repo else {
            return Err(format!("line {}: an entry has no `repo`", first.no));
        };
        let entry = Entry {
            repo,
            rev,
            hooks: entry_hooks,
            line: first.no,
            rendered: first.rendered,
        };
        if entry.remote() && entry.rev.is_none() {
            return Err(format!(
                "line {}: the entry of {} has no `rev`",
                first.no, entry.repo.text
            ));
        }
        entries.push(entry);
    }
    Ok(Config { entries })
}

// --- Revisions ---

/// A repository URL without a trailing slash or `.git`.
fn normalized(url: &str) -> &str {
    let url = url.trim_end_matches('/');
    url.strip_suffix(".git").unwrap_or(url)
}

/// The last path segment of a repository URL.
fn repo_name(url: &str) -> &str {
    normalized(url).rsplit('/').next().unwrap_or_default()
}

/// The `<owner>/<name>` of a GitHub repository URL.
fn github_slug(url: &str) -> Option<String> {
    let rest = [
        "https://github.com/",
        "http://github.com/",
        "ssh://git@github.com/",
        "git@github.com:",
    ]
    .iter()
    .find_map(|prefix| url.strip_prefix(prefix))?;
    let rest = normalized(rest);
    let (owner, name) = rest.split_once('/')?;
    let plain = |part: &str| {
        !part.is_empty()
            && part
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-_.".contains(&byte))
    };
    (plain(owner) && plain(name)).then(|| format!("{owner}/{name}"))
}

/// Accept a tag or a revision that fits a URL path and a plain YAML value.
fn plain_rev(rev: &str) -> Result<(), String> {
    let fits = !rev.is_empty()
        && rev.len() <= 128
        && !rev.starts_with(['-', '.'])
        && rev
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_.+".contains(&byte));
    if fits {
        Ok(())
    } else {
        Err(format!("`{rev}` is not a plain tag or revision"))
    }
}

/// Accept a set name: lower-case letters, digits, and dashes.
fn plain_set(name: &str) -> Result<(), String> {
    let fits = !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');
    if fits {
        Ok(())
    } else {
        Err(format!("`{name}` is not a set name"))
    }
}

/// The revision that the sets expect for one repository.
#[derive(Debug, Clone)]
struct Expected {
    rev: String,
    set: String,
}

/// The revision of each remote repository that the named sets hold.
fn expected(sets: &[(String, String)]) -> Result<BTreeMap<String, Expected>, String> {
    let mut map: BTreeMap<String, Expected> = BTreeMap::new();
    for (name, text) in sets {
        let config = parse(text).map_err(|e| format!("set {name}: {e}"))?;
        for entry in config.entries.iter().filter(|entry| entry.remote()) {
            let Some(rev) = &entry.rev else { continue };
            if repo_name(&entry.repo.text) == EXOBRAIN_TECH {
                continue;
            }
            let key = normalized(&entry.repo.text).to_owned();
            match map.get(&key) {
                Some(found) if found.rev != rev.text => {
                    return Err(format!(
                        "sets {} and {name} pin {key} at {} and {}",
                        found.set, found.rev, rev.text
                    ));
                }
                Some(_) => {}
                None => {
                    map.insert(
                        key,
                        Expected {
                            rev: rev.text.clone(),
                            set: name.clone(),
                        },
                    );
                }
            }
        }
    }
    Ok(map)
}

/// One repository whose `rev` differs from its set.
struct Difference<'c> {
    entry: &'c Entry,
    pinned: &'c str,
    expected: Expected,
}

/// The entries outside a rendered block whose `rev` differs from the
/// sets. The exobrain-tech entry is never one of them.
fn differences<'c>(
    config: &'c Config,
    expected: &BTreeMap<String, Expected>,
) -> Vec<Difference<'c>> {
    config
        .entries
        .iter()
        .filter(|entry| !entry.rendered && entry.remote() && !entry.is_exobrain_tech())
        .filter_map(|entry| {
            let pinned = entry.rev.as_ref()?;
            let found = expected.get(normalized(&entry.repo.text))?;
            (found.rev != pinned.text).then(|| Difference {
                entry,
                pinned: &pinned.text,
                expected: found.clone(),
            })
        })
        .collect()
}

/// The sets that the `args` of `hook-revisions` name.
fn set_names(entry: &Entry) -> Vec<String> {
    let mut names = Vec::new();
    for hook in entry.hooks.iter().filter(|hook| hook.id == HOOK_ID) {
        let mut args = hook.args.iter();
        while let Some(arg) = args.next() {
            if arg == "--set" {
                names.extend(args.next().cloned());
            } else if let Some(name) = arg.strip_prefix("--set=") {
                names.push(name.to_owned());
            }
        }
    }
    names
}

// --- check ---

fn check(args: &CheckArgs) -> Result<u8, Error> {
    let fail = |line: String| {
        output::note(&format!("exobrain hooks check: {line}"));
        Ok(USAGE)
    };
    let text = fs::read_to_string(&args.config).map_err(|source| Error::Io {
        what: args.config.display().to_string(),
        source,
    })?;
    let config = match parse(&text) {
        Ok(config) => config,
        Err(e) => return fail(format!("{}: {e}", args.config.display())),
    };
    let mut sets = Vec::new();
    for name in &args.sets {
        if let Err(e) = plain_set(name) {
            return fail(e);
        }
        let path = args.sets_dir.join(format!("{name}.yaml"));
        match fs::read_to_string(&path) {
            Ok(text) => sets.push((name.clone(), text)),
            Err(e) => return fail(format!("set {name}: {}: {e}", path.display())),
        }
    }
    let expected = match expected(&sets) {
        Ok(expected) => expected,
        Err(e) => return fail(e),
    };
    let tag = config
        .exobrain_tech()
        .and_then(|entry| entry.rev.as_ref())
        .map(|rev| rev.text.as_str());
    let of_tag = tag.map_or_else(String::new, |tag| format!(" of exobrain-tech {tag}"));
    let found = differences(&config, &expected);
    for difference in &found {
        output::note(&format!(
            "exobrain hooks check: {} pins {}, and set {}{of_tag} pins {} (line {})",
            difference.entry.repo.text,
            difference.pinned,
            difference.expected.set,
            difference.expected.rev,
            difference.entry.line,
        ));
    }
    if found.is_empty() {
        return Ok(0);
    }
    output::note(&tag.map_or_else(
        || "fix: exobrain hooks sync".to_owned(),
        |tag| format!("fix: exobrain hooks sync --to {tag}"),
    ));
    Ok(FAILED)
}

// --- GitHub through gh ---

/// The host `gh`.
fn gh() -> Result<PathBuf, String> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    probe::which("gh", &path).ok_or_else(|| "gh is not on PATH".to_owned())
}

/// Run `gh api` with `args` through the bounded call of the release
/// cache, and return its stdout.
fn gh_api(gh: &Path, args: &[&str], deadline: Instant) -> Result<String, String> {
    release_cache::gh_api(gh, args, deadline)
}

/// The deadline of one bounded `gh` call that starts now.
fn deadline() -> Instant {
    Instant::now() + release_cache::TIMEOUT
}

/// The tag of the latest release of `slug`.
fn latest_tag(gh: &Path, slug: &str) -> Result<String, String> {
    let text = gh_api(
        gh,
        &[
            &format!("repos/{slug}/releases/latest"),
            "--jq",
            ".tag_name",
        ],
        deadline(),
    )?;
    let tag = text.trim().to_owned();
    plain_rev(&tag).map_err(|e| format!("the latest release: {e}"))?;
    Ok(tag)
}

/// The text of set `name` at `tag` of `slug`, read by `deadline`.
fn fetch_set(
    gh: &Path,
    slug: &str,
    tag: &str,
    name: &str,
    deadline: Instant,
) -> Result<String, String> {
    gh_api(
        gh,
        &[
            "-H",
            "Accept: application/vnd.github.raw",
            &format!("repos/{slug}/contents/{SETS_PATH}/{name}.yaml?ref={tag}"),
        ],
        deadline,
    )
}

/// Write `bytes` at `path` through a temporary file beside it and a
/// rename, with the mode of the file it replaces.
pub(crate) fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let name = path
        .file_name()
        .ok_or_else(|| std::io::Error::other("the path has no file name"))?;
    let mut tmp_name = std::ffi::OsString::from(".");
    tmp_name.push(name);
    tmp_name.push(format!(".tmp.{}", std::process::id()));
    let tmp = path.with_file_name(tmp_name);
    let mode = fs::metadata(path).ok().map(|meta| meta.permissions());
    let written = fs::File::create(&tmp)
        .and_then(|mut file| file.write_all(bytes).and_then(|()| file.sync_all()))
        .and_then(|()| mode.map_or(Ok(()), |mode| fs::set_permissions(&tmp, mode)))
        .and_then(|()| fs::rename(&tmp, path));
    if written.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    written
}

// --- sync ---

fn sync(args: &SyncArgs) -> u8 {
    match try_sync(args) {
        Ok(lines) => {
            for line in lines {
                output::note(&format!("exobrain hooks sync: {line}"));
            }
            0
        }
        Err(e) => {
            output::note(&format!(
                "exobrain hooks sync: {e}; {} is unchanged",
                args.config.display()
            ));
            FAILED
        }
    }
}

/// Fetch and check every input, then write the config once.
fn try_sync(args: &SyncArgs) -> Result<Vec<String>, String> {
    let shown = args.config.display();
    let text = fs::read_to_string(&args.config).map_err(|e| format!("{shown}: {e}"))?;
    let config = parse(&text).map_err(|e| format!("{shown}: {e}"))?;
    let entry = config
        .hook_entry()
        .ok_or_else(|| format!("{shown} enables no `{HOOK_ID}` hook"))?;
    let names = set_names(entry);
    if names.is_empty() {
        return Err(format!("`{HOOK_ID}` names no `--set` in {shown}"));
    }
    for name in &names {
        plain_set(name)?;
    }
    let slug = github_slug(&entry.repo.text)
        .ok_or_else(|| format!("{} is not a GitHub repository", entry.repo.text))?;
    let gh = gh()?;
    let tag = match &args.to {
        Some(tag) => tag.clone(),
        None => latest_tag(&gh, &slug)?,
    };
    plain_rev(&tag)?;
    let mut sets = Vec::new();
    for name in &names {
        let text = fetch_set(&gh, &slug, &tag, name, deadline())
            .map_err(|e| format!("cannot fetch set {name} of {slug} {tag}: {e}"))?;
        sets.push((name.clone(), text));
    }
    let expected = expected(&sets)?;
    let mut edits: Vec<(Range<usize>, String)> = Vec::new();
    let mut lines = Vec::new();
    if let Some(rev) = entry.rev.as_ref().filter(|rev| rev.text != tag) {
        edits.push((rev.span.clone(), tag.clone()));
        lines.push(format!("{}: {} -> {tag}", entry.repo.text, rev.text));
    }
    for difference in differences(&config, &expected) {
        plain_rev(&difference.expected.rev)
            .map_err(|e| format!("set {}: {e}", difference.expected.set))?;
        let Some(rev) = &difference.entry.rev else {
            continue;
        };
        edits.push((rev.span.clone(), difference.expected.rev.clone()));
        lines.push(format!(
            "{}: {} -> {}",
            difference.entry.repo.text, difference.pinned, difference.expected.rev
        ));
    }
    if edits.is_empty() {
        return Ok(vec![format!("{shown} already pins the sets of {tag}")]);
    }
    edits.sort_by_key(|(span, _)| span.start);
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    for (span, value) in &edits {
        out.push_str(&text[at..span.start]);
        out.push_str(value);
        at = span.end;
    }
    out.push_str(&text[at..]);
    parse(&out).map_err(|e| format!("the rewrite does not parse: {e}"))?;
    write_atomic(&args.config, out.as_bytes()).map_err(|e| format!("cannot write {shown}: {e}"))?;
    Ok(lines)
}

// --- The hook-set check of `exobrain outdated` ---

/// The project facts that the hook-set check reads from the config.
#[derive(Debug, Clone)]
pub(crate) struct Project {
    config: Config,
    /// The `<owner>/<name>` of the exobrain-tech entry.
    pub(crate) slug: String,
    /// The exobrain-tech `rev` of the config.
    tag: Option<String>,
    /// The sets that `hook-revisions` names, each a plain set name.
    pub(crate) sets: Vec<String>,
}

impl Project {
    /// The project facts of `config`, or `None` when it has no
    /// `hook-revisions` hook on a GitHub repository.
    pub(crate) fn of(config: Config) -> Option<Self> {
        let entry = config.hook_entry()?;
        let slug = github_slug(&entry.repo.text)?;
        let tag = entry.rev.as_ref().map(|rev| rev.text.clone());
        let sets = set_names(entry)
            .into_iter()
            .filter(|name| plain_set(name).is_ok())
            .collect();
        Some(Self {
            config,
            slug,
            tag,
            sets,
        })
    }
}

/// Read and parse the config at `path`, or `None` where it is missing or
/// does not parse.
pub(crate) fn read_config(path: &Path) -> Option<Config> {
    parse(&fs::read_to_string(path).ok()?).ok()
}

/// The `<owner>/<name>` of the exobrain-tech entry of `config`, where it
/// is a GitHub repository.
pub(crate) fn tech_slug(config: &Config) -> Option<String> {
    github_slug(&config.exobrain_tech()?.repo.text)
}

/// The slug of the repository that ships this binary.
pub(crate) fn own_slug() -> Option<String> {
    github_slug(env!("CARGO_PKG_REPOSITORY"))
}

/// The folder of the cached set files.
fn sets_root() -> Option<PathBuf> {
    Some(stamp::state_dir()?.parent()?.join("hooks"))
}

/// The folder of the cached sets of `tag` of `slug`, or `None` where no
/// state root is set or `tag` does not fit a path.
pub(crate) fn sets_dir(slug: &str, tag: &str) -> Option<PathBuf> {
    plain_rev(tag).ok()?;
    Some(sets_root()?.join(slug.replace('/', "-")).join(tag))
}

/// The sets `names` from the cache folder `dir`, or `None` where one of
/// them is missing.
pub(crate) fn cached_sets(dir: &Path, names: &[String]) -> Option<Vec<(String, String)>> {
    names
        .iter()
        .map(|name| {
            fs::read_to_string(dir.join(format!("{name}.yaml")))
                .ok()
                .map(|text| (name.clone(), text))
        })
        .collect()
}

/// The sets `names` of `tag` of `slug`: each from the cache folder `dir`
/// where it holds the set, else read through `gh` by `deadline` and
/// written to `dir`. The set files of a tag never change.
///
/// # Errors
///
/// The set, the tag, and the reason, where a read fails.
pub(crate) fn load_sets(
    slug: &str,
    tag: &str,
    names: &[String],
    dir: Option<&Path>,
    deadline: Instant,
) -> Result<Vec<(String, String)>, String> {
    plain_rev(tag)?;
    let mut gh_path = None;
    let mut sets = Vec::new();
    for name in names {
        let path = dir.map(|dir| dir.join(format!("{name}.yaml")));
        if let Some(text) = path.as_ref().and_then(|path| fs::read_to_string(path).ok()) {
            sets.push((name.clone(), text));
            continue;
        }
        let gh = match &gh_path {
            Some(gh) => gh,
            None => gh_path.insert(gh().map_err(|e| format!("set {name} of {tag}: {e}"))?),
        };
        let text = fetch_set(gh, slug, tag, name, deadline)
            .map_err(|e| format!("set {name} of {tag}: {e}"))?;
        if let (Some(dir), Some(path)) = (dir, &path) {
            let _ = fs::create_dir_all(dir).and_then(|()| write_atomic(path, text.as_bytes()));
        }
        sets.push((name.clone(), text));
    }
    Ok(sets)
}

/// The line that names a newer CLI, where `latest` is a greater version
/// than the running binary.
pub(crate) fn cli_line(latest: &str) -> Option<String> {
    let running = env!("CARGO_PKG_VERSION");
    release_cache::newer(latest, running).then(|| {
        format!(
            "exobrain hooks: exobrain-tech {latest} is out, and this host runs exobrain {running}"
        )
    })
}

/// Whether `latest` is newer than the exobrain-tech `rev` of the project.
pub(crate) fn behind(project: &Project, latest: &str) -> bool {
    let Some(tag) = &project.tag else {
        return false;
    };
    match (
        release_cache::tag_version(latest),
        release_cache::tag_version(tag),
    ) {
        (Some(_), Some(_)) => release_cache::newer(latest, tag),
        _ => latest != tag,
    }
}

/// The lines that report the revisions of `latest` that differ from the
/// project, given the sets of `latest` where they are at hand. Nothing
/// where the project is not behind `latest`.
pub(crate) fn project_lines(
    project: &Project,
    latest: &str,
    sets: Option<&[(String, String)]>,
) -> Vec<String> {
    let Some(tag) = project.tag.as_ref().filter(|_| behind(project, latest)) else {
        return Vec::new();
    };
    let mut lines = Vec::new();
    if let Some(expected) = sets.and_then(|sets| expected(sets).ok()) {
        for difference in differences(&project.config, &expected) {
            lines.push(format!(
                "exobrain hooks: {latest} pins {} at {}, and this project pins {}",
                difference.entry.repo.text, difference.expected.rev, difference.pinned
            ));
        }
    }
    lines.push(format!(
        "exobrain hooks: this project selects {tag}; move it with `exobrain hooks sync --to {latest}` on the branch of your next change"
    ));
    lines
}

#[cfg(test)]
mod tests {
    use super::{github_slug, parse, set_names};

    const CONFIG: &str = concat!(
        "minimum_prek_version: '0.5.7'\n",
        "repos:\n",
        "  - repo: https://github.com/gubasso/exobrain-tech\n",
        "    rev: v1.2.0\n",
        "    hooks:\n",
        "      - id: hook-revisions\n",
        "        args: [--set, common, '--set=rust']\n",
        "  # a comment\n",
        "  - repo: https://github.com/crate-ci/typos\n",
        "    rev: 'v1.52.2'  # frozen\n",
        "    hooks:\n",
        "      - id: typos\n",
        "        args:\n",
        "          - --force-exclude\n",
        "  -   repo: builtin\n",
        "      hooks:\n",
        "      - id: check-yaml\n",
        "        args:\n",
        "        - --allow-multiple-documents\n",
    );

    #[test]
    fn the_parse_reads_the_block_style() {
        let config = parse(CONFIG).unwrap();
        assert_eq!(config.entries.len(), 3);
        let typos = &config.entries[1];
        let rev = typos.rev.as_ref().unwrap();
        assert_eq!(rev.text, "v1.52.2");
        assert_eq!(&CONFIG[rev.span.clone()], "v1.52.2");
        assert_eq!(typos.hooks[0].args, ["--force-exclude"]);
        assert_eq!(set_names(&config.entries[0]), ["common", "rust"]);
        assert!(config.entries[2].rev.is_none());
        assert_eq!(
            config.entries[2].hooks[0].args,
            ["--allow-multiple-documents"]
        );
    }

    #[test]
    fn the_parse_refuses_what_it_does_not_read() {
        assert!(parse("repos:\n  - repo: https://x/y\n    hooks: []\n").is_err());
        assert!(parse("repos: [{repo: local}]\n").is_err());
        assert!(parse("repos:\n\t- repo: local\n").is_err());
        assert!(parse("repos:\n  - repo: https://x/y\n    rev: \"a\\\"b\"\n").is_err());
        assert!(parse("# BEGIN release-kit\nrepos: []\n").is_err());
        assert!(parse("default_stages: [pre-commit]\n").is_err());
    }

    #[test]
    fn a_slug_comes_from_a_github_url() {
        assert_eq!(
            github_slug("https://github.com/o/n.git").as_deref(),
            Some("o/n")
        );
        assert_eq!(github_slug("https://gitlab.com/o/n"), None);
    }
}

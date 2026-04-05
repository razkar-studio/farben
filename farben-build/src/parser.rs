//! Parser for `.frb.toml` config files.
//!
//! Reads INI-like files with `[styles]` and `[prefixes]` section headers, optional
//! namespace qualifiers like `[styles.my]`, and `key = "value"` pairs. Blank lines
//! and `#` comments are ignored. Produces a `FrbConfig` whose keys are namespaced
//! as `namespace:key` when a namespace is present.

use std::collections::HashMap;

/// The parsed result of a `.frb` config file.
///
/// `styles` maps tag names (optionally namespaced as `namespace:key`) to farben
/// markup strings. `prefixes` maps tag names to prefix strings.
pub struct FrbConfig {
    pub styles: HashMap<String, String>,
    pub prefixes: HashMap<String, String>,
}

/// Tracks which section header is currently active during parsing.
///
/// The `Option<String>` holds the namespace qualifier when present, such as
/// `my` from a `[styles.my]` header.
enum Section {
    Styles(Option<String>),
    Prefixes(Option<String>),
    None,
}

/// Errors produced while parsing a `.frb` config file.
pub enum ParseError {
    /// A section header was unrecognized or malformed (e.g. missing the closing `]`).
    InvalidSection(String),
    /// A line within a section could not be parsed as `key = "value"`.
    InvalidKeyValue(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSection(s) => write!(f, "invalid section: '{s}'"),
            Self::InvalidKeyValue(s) => write!(f, "invalid key value: '{s}'"),
        }
    }
}

/// Parses the full text of a `.frb` config file into a [`FrbConfig`].
///
/// Blank lines and lines starting with `#` are skipped. Section headers like
/// `[styles]` or `[styles.myns]` set the active section. Key-value lines like
/// `key = "value"` are inserted into the appropriate map, with namespaced keys
/// stored as `namespace:key`.
///
/// Returns `Err(ParseError::InvalidSection)` for unrecognized headers and
/// `Err(ParseError::InvalidKeyValue)` for malformed value lines.
pub fn parse(input: &str) -> Result<FrbConfig, ParseError> {
    let mut styles = HashMap::new();
    let mut prefixes = HashMap::new();
    let mut current_section = Section::None;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') {
            let Some(closing) = line.find(']') else {
                return Err(ParseError::InvalidSection(line.to_string()));
            };
            let section_str = &line[1..closing];
            let mut parts = section_str.splitn(2, '.');
            let section_name = parts.next().unwrap();
            let namespace = parts.next().map(|s| s.to_string());

            current_section = match section_name {
                "styles" => Section::Styles(namespace),
                "prefixes" => Section::Prefixes(namespace),
                _ => return Err(ParseError::InvalidSection(section_str.to_string())),
            };
        } else if line.contains('=') {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap().trim();
            let value = parts
                .next()
                .ok_or(ParseError::InvalidKeyValue(line.to_string()))?
                .trim()
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or(ParseError::InvalidKeyValue(line.to_string()))?;
            let full_key = match &current_section {
                Section::Styles(Some(ns)) | Section::Prefixes(Some(ns)) => format!("{ns}:{key}"),
                _ => key.to_string(),
            };
            match &current_section {
                Section::Styles(_) => styles.insert(full_key, value.to_string()),
                Section::Prefixes(_) => prefixes.insert(full_key, value.to_string()),
                Section::None => return Err(ParseError::InvalidKeyValue(line.to_string())),
            };
        }
    }

    Ok(FrbConfig { styles, prefixes })
}

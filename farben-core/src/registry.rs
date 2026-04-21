//! Global named style registry.
//!
//! Stores user-defined [`Style`] values under string keys, allowing markup like
//! `[danger]` to expand into a pre-configured combination of colors and emphasis.
//!
//! The registry is process-global and backed by a [`Mutex`]-protected [`HashMap`].
//! All operations are safe to call from multiple threads, though the typical usage
//! pattern is to register styles once at program startup.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

use crate::{ansi::Style, errors::RegistryError};

static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Style>>>> = OnceLock::new();

/// Registers a named style in the global registry.
///
/// If a style with `name` already exists, it is replaced.
///
/// # Errors
///
/// Returns [`RegistryError::InvalidName`] if `name` contains `[` or `]`.
pub fn insert_style(name: impl Into<String>, style: Style) -> Result<(), RegistryError> {
    let name = name.into();
    if name.contains('[') || name.contains(']') {
        return Err(RegistryError::InvalidName(name));
    }
    REGISTRY
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(name, Arc::new(style));
    Ok(())
}

/// Sets the prefix string for an already-registered named style.
///
/// The prefix is prepended to the style's escape sequence at render time,
/// allowing a named style to inject arbitrary text before its ANSI codes.
///
/// # Errors
///
/// Returns [`crate::errors::RegistryError::UnknownStyle`] if `name` has not been registered via
/// [`insert_style`] (or the `style!` macro from the `farben` crate).
pub fn set_prefix(name: impl Into<String>, prefix: impl Into<String>) -> Result<(), RegistryError> {
    let name = name.into();
    let prefix = prefix.into();

    let mut map = REGISTRY
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    if let Some(style) = map.get_mut(&name) {
        let s = Arc::make_mut(style);
        s.prefix = Some(prefix);
        Ok(())
    } else {
        Err(RegistryError::UnknownStyle(name))
    }
}

/// Looks up a named style in the global registry.
///
/// # Errors
///
/// Returns `LexError::InvalidTag` if `query` does not match any registered style name.
pub fn search_registry(query: impl Into<String>) -> Result<Arc<Style>, RegistryError> {
    let map = REGISTRY
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    let query = query.into();
    match map.get(&query) {
        Some(style) => Ok(Arc::clone(style)),
        None => Err(RegistryError::UnknownStyle(query)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::Style;

    fn dummy_style() -> Style {
        Style::parse("[bold]").unwrap()
    }

    #[test]
    fn test_insert_style_valid_name_succeeds() {
        assert!(insert_style("mytest-style", dummy_style()).is_ok());
    }

    #[test]
    fn test_insert_style_bracket_in_name_rejected() {
        assert!(insert_style("[danger]", dummy_style()).is_err());
    }

    #[test]
    fn test_insert_style_open_bracket_rejected() {
        let err = insert_style("bad[name", dummy_style()).unwrap_err();
        assert!(matches!(err, RegistryError::InvalidName(_)));
    }

    #[test]
    fn test_insert_style_close_bracket_rejected() {
        let err = insert_style("bad]name", dummy_style()).unwrap_err();
        assert!(matches!(err, RegistryError::InvalidName(_)));
    }

    #[test]
    fn test_insert_style_registered_and_retrievable() {
        insert_style("retrieve-test", dummy_style()).unwrap();
        assert!(search_registry("retrieve-test").is_ok());
    }

    #[test]
    fn test_search_registry_unknown_returns_error() {
        assert!(search_registry("nonexistent-zzzz").is_err());
    }
}

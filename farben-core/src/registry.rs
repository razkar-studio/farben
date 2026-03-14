use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use crate::ansi::Style;
use crate::errors::LexError;

static REGISTRY: OnceLock<Mutex<HashMap<String, Style>>> = OnceLock::new();

pub fn insert_style(name: impl Into<String>, style: Style) {
    REGISTRY
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(name.into(), style);
}

pub(crate) fn search_registry(query: impl Into<String>) -> Result<Style, LexError> {
    let map = REGISTRY
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    let query = query.into();
    match map.get(&query) {
        Some(style) => Ok(style.clone()),
        None => Err(LexError::InvalidTag(query)),
    }
}

#[macro_export]
macro_rules! style {
    ($name:expr, $markup:expr) => {
        farben_core::registry::insert_style(
            $name,
            farben_core::ansi::Style::parse($markup).unwrap(),
        );
    };
}

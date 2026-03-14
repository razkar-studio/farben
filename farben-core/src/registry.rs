use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use crate::ansi::Style;

static REGISTRY: OnceLock<Mutex<HashMap<String, Style>>> = OnceLock::new();

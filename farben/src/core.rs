/*
 * Copyright (c) 2026 RazkarStudio
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Core Farben internals that are used by certain macros.
//!
//! This module is exposed by `farben::*`, which is why that is discouraged. Use `farben::prelude::*` instead to avoid
//! exposing this module.

pub use farben_core::ansi::style_to_ansi;
pub use farben_core::debug::tokens_to_markup;
pub use farben_core::lexer::tokenize;

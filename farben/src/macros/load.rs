/*
 * Copyright (c) 2026 RazkarStudio
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! The [`load_styles!`] macro for including build-generated style registrations at compile time.

/// Includes the build-generated `farben_styles.rs` file from `OUT_DIR`.
///
/// Registers all styles and prefixes defined in `.frb` config files. Call this
/// once at the top of `main` when using the farben build integration.
#[macro_export]
macro_rules! load_styles {
    () => {
        include!(concat!(env!("OUT_DIR"), "/farben_styles.rs"));
    };
}

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

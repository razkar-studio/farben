/// Parses farben markup and converts the result into an [`anstyle::Style`].
///
/// This macro parses the given farben markup string at compile time (if the
/// `compile` feature is enabled) or runtime and converts the resulting [`Style`]
/// into an [`anstyle::Style`]. Requires the `anstyle` feature flag.
///
/// # Panics
///
/// Panics if the markup string is invalid farben markup.
///
/// # Example
///
/// ```
/// use anstyle::Style;
/// let style: Style = farben::anstyle!("[bold red]Warning: ");
/// ```
#[macro_export]
macro_rules! anstyle {
    ($markup:expr) => {
        Into::<anstyle::Style>::into(
            farben_core::ansi::Style::parse($markup)
                .unwrap_or_else(|e| panic!("farben: invalid markup: {e}")),
        )
    };
}

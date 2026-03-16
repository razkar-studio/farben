/// A parsed inline markdown token.
///
/// Each variant carries its own content as an owned `String`, allowing the
/// renderer to process tokens independently without a separate open/close pass.
/// Produced by [`tokenize`] and consumed by [`render`].
pub enum MdToken {
    /// A run of plain text with no markdown formatting.
    Text(String),
    /// A bold span, delimited by `**...**`.
    Bold(String),
    /// An italic span, delimited by `*...*` or `_..._`.
    Italic(String),
    /// An inline code span, delimited by `` `...` ``.
    Code(String),
    /// A strikethrough span, delimited by `~~...~~`.
    Strikethrough(String),
    /// An underlined span, delimited by `__...__`.
    Underline(String),
}

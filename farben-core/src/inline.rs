//! Inline syntax sugar pre-processor.
//!
//! Transforms shorthand emphasis syntax into farben markup tags before tokenization.
//! This runs as a pure `String → String` pass; the farben pipeline is unchanged.
//!
//! |        Syntax         | Output |
//! |-----------------------|--------|
//! | `*bold*`              | `[bold]bold[/bold]` |
//! | `_underline_`         | `[underline]underline[/underline]` |
//! | `` `code` ``          | `[code]code[/code]` |
//! | `~strikethrough~`     | `[strikethrough]strikethrough[/]` |
//! | `/italic/`            | `[italic]italic[/italic]` |
//! | `**`                  | `*` (escaped literal) |
//! | `__`                  | `_` (escaped literal) |
//! | content inside `[...]`| passed through untouched |
//! | unclosed delimiter    | passed through as literal |

struct PendingSpan {
    delimiter: char,
    position: usize,
    open_tag: &'static str,
}

fn delimiter_tags(c: char) -> Option<(&'static str, &'static str)> {
    match c {
        '*' => Some(("[bold]", "[/bold]")),
        '_' => Some(("[underline]", "[/underline]")),
        '`' => Some(("[code]", "[/code]")),
        '~' => Some(("[strikethrough]", "[/strikethrough]")),
        '/' => Some(("[italic]", "[/italic]")),
        _ => None,
    }
}

/// Transforms inline shorthand syntax into farben markup tags.
///
/// Runs as a pre-processing pass before tokenization. See module-level
/// docs for the full syntax table.
#[must_use]
pub fn preprocess(input: &str) -> String {
    let mut output = String::with_capacity(input.len() + 16);
    let mut chars = input.chars().peekable();
    let mut bracket_depth: u32 = 0;
    let mut open_spans: Vec<PendingSpan> = Vec::new();

    while let Some(c) = chars.next() {
        if c == '[' {
            bracket_depth += 1;
            output.push('[');
            continue;
        }
        if c == ']' {
            bracket_depth = bracket_depth.saturating_sub(1);
            output.push(']');
            continue;
        }

        if bracket_depth > 0 {
            output.push(c);
            continue;
        }

        if let Some(&next) = chars.peek()
            && next == c
            && delimiter_tags(c).is_some()
        {
            chars.next();
            output.push(c);
            continue;
        }

        if let Some((open_tag, close_tag)) = delimiter_tags(c) {
            if let Some(pos) = open_spans.iter().rposition(|s| s.delimiter == c) {
                open_spans.remove(pos);
                output.push_str(close_tag);
            } else {
                let position = output.len();
                open_spans.push(PendingSpan {
                    delimiter: c,
                    position,
                    open_tag,
                });
                output.push_str(open_tag);
            }
            continue;
        }

        output.push(c);
    }

    open_spans.sort_by(|a, b| b.position.cmp(&a.position));
    for span in open_spans {
        output.replace_range(
            span.position..span.position + span.open_tag.len(),
            &span.delimiter.to_string(),
        );
    }

    output
}

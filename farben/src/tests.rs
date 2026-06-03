//! Comprehensive integration tests for the `farben` crate.
//!
//! Covers `try_color`, `color_runtime`, `colorb`, all print/write macros via writers,
//! strip/escape macros, format macros (`cformat!`, `cformatb!`), and `FarbenStr`.
//! All tests force ANSI color on to verify exact escape sequences.
//!
//! Run with `-- --test-threads=1` if env-var races appear.

use std::io::{Cursor, Write};

use super::*;

// ============================================================================
// Foreword: env-var discipline
// ============================================================================
// Every test that produces ANSI output sets `FORCE_COLOR=1` first. The value
// is never unset within a test, so adjacent tests in the same thread must also
// set it if they expect color. Races are avoided by running with `--test-threads=1`.

// ============================================================================
// try_color -- basic cases
// ============================================================================

#[test]
fn test_try_color_named_color() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[red]I'm red!");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[31mI'm red!\x1b[0m");
}

#[test]
fn test_try_color_appends_trailing_reset() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[blue]text");
    assert!(result.is_ok());
    assert!(result.unwrap().ends_with("\x1b[0m"));
}

#[test]
fn test_try_color_plain_text_no_tags() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("no markup here");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "no markup here\x1b[0m");
}

#[test]
fn test_try_color_empty_string() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[0m");
}

#[test]
fn test_try_color_invalid_tag_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[error]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_unclosed_tag_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[red");
    assert!(result.is_err());
}

#[test]
fn test_try_color_rgb_color() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[rgb(255,0,0)]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
}

#[test]
fn test_try_color_bold_and_named_color() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[bold red]hi");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[1m\x1b[31mhi\x1b[0m");
}

#[test]
fn test_try_color_escaped_bracket() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[[not a tag]");
    assert!(result.is_ok());
    assert!(result.unwrap().starts_with('['));
}

#[test]
fn test_try_color_inline_reset() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[red]before[/]after");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[31mbefore\x1b[0mafter\x1b[0m");
}

// ============================================================================
// try_color -- advanced color formats
// ============================================================================

#[test]
fn test_try_color_hsl() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hsl(0,100,50)]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
}

#[test]
fn test_try_color_hsl_with_spaces() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hsl( 120 , 100 , 50 )]green");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;0;255;0mgreen\x1b[0m");
}

#[test]
fn test_try_color_hsv() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hsv(0,100,100)]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
}

#[test]
fn test_try_color_hwb() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hwb(0,0,0)]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
}

#[test]
fn test_try_color_lab() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[lab(0,0,0)]black");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;0;0;0mblack\x1b[0m");
}

#[test]
fn test_try_color_lch() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[lch(0,0,0)]black");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;0;0;0mblack\x1b[0m");
}

#[test]
fn test_try_color_oklch() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[oklch(0,0,0)]black");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;0;0;0mblack\x1b[0m");
}

#[test]
fn test_try_color_hex_6digit() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[#ff0000]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
}

#[test]
fn test_try_color_hex_3digit() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[#f00]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
}

#[test]
fn test_try_color_ansi256() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[ansi(196)]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;5;196mred\x1b[0m");
}

#[test]
fn test_try_color_ansi256_bg() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[bg:ansi(196)]red bg");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[48;5;196mred bg\x1b[0m");
}

#[test]
fn test_try_color_rgb_with_spaces() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[rgb( 0 , 255 , 0 )]green");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;0;255;0mgreen\x1b[0m");
}

#[test]
fn test_try_color_hsl_invalid_hue_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hsl(400,50,50)]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_hsl_wrong_arg_count_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hsl(100,50)]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_hex_invalid_length_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[#ff00]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_hex_invalid_chars_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[#xyz]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_ansi256_invalid_value_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[ansi(300)]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_hsv_hue_out_of_range_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hsv(400,50,50)]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_lab_l_out_of_range_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[lab(150,0,0)]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_hwb_wb_too_high_returns_error() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[hwb(0,60,60)]text");
    assert!(result.is_err());
}

// ============================================================================
// try_color -- emphasis styles (all 11)
// ============================================================================

#[test]
fn test_try_color_emphasis_bold() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[bold]text").unwrap(), "\x1b[1mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_dim() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[dim]text").unwrap(), "\x1b[2mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_italic() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[italic]text").unwrap(), "\x1b[3mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_underline() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[underline]text").unwrap(), "\x1b[4mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_double_underline() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[double-underline]text").unwrap(), "\x1b[21mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_blink() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[blink]text").unwrap(), "\x1b[5mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_rapid_blink() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[rapid-blink]text").unwrap(), "\x1b[6mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_reverse() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[reverse]text").unwrap(), "\x1b[7mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_invisible() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[invisible]text").unwrap(), "\x1b[8mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_strikethrough() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[strikethrough]text").unwrap(), "\x1b[9mtext\x1b[0m");
}

#[test]
fn test_try_color_emphasis_overline() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert_eq!(try_color("[overline]text").unwrap(), "\x1b[53mtext\x1b[0m");
}

// ============================================================================
// try_color -- compound and combined tags
// ============================================================================

#[test]
fn test_try_color_compound_tag() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[bold italic underline red bg:blue]compound");
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "\x1b[1m\x1b[3m\x1b[4m\x1b[31m\x1b[44mcompound\x1b[0m"
    );
}

#[test]
fn test_try_color_fg_bg_same_bracket() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[fg:red bg:blue]text");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[31m\x1b[44mtext\x1b[0m");
}

#[test]
fn test_try_color_background_named() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[bg:green]text");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[42mtext\x1b[0m");
}

#[test]
fn test_try_color_background_rgb() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[bg:rgb(255,0,0)]text");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[48;2;255;0;0mtext\x1b[0m");
}

#[test]
fn test_try_color_multiple_tags_sequentially() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[red]R [green]G [blue]B");
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "\x1b[31mR \x1b[32mG \x1b[34mB\x1b[0m"
    );
}

#[test]
fn test_try_color_reset_mid_string() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[red]red[/]normal[blue]blue[/]end");
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "\x1b[31mred\x1b[0mnormal\x1b[34mblue\x1b[0mend\x1b[0m"
    );
}

#[test]
fn test_try_color_reset_at_start() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[/]text");
    assert!(result.is_ok());
    // A bare [/] at the start emits a reset code, then text, then trailing reset
    assert_eq!(result.unwrap(), "\x1b[0mtext\x1b[0m");
}

#[test]
fn test_try_color_escape_double_bracket_in_text() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("use [[bold]] for bold");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "use [bold] for bold\x1b[0m");
}

#[test]
fn test_try_color_double_close_bracket_in_text() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("end]]");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "end]\x1b[0m");
}

// ============================================================================
// try_color -- long / unicode strings
// ============================================================================

#[test]
fn test_try_color_unicode_text() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[green]Guten Tag, Welt!");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[32mGuten Tag, Welt!\x1b[0m");
}

#[test]
fn test_try_color_emoji_preserved() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = try_color("[red]Error: \u{26a0} warning!");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[31mError: \u{26a0} warning!\x1b[0m");
}

// ============================================================================
// try_color -- error variants detail
// ============================================================================

#[test]
fn test_try_color_invalid_arg_count_rgb() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let err = try_color("[rgb(255,0)]text").unwrap_err();
    assert!(matches!(err, LexError::InvalidArgumentCount { .. }));
}

#[test]
fn test_try_color_invalid_value_rgb() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let err = try_color("[rgb(300,0,0)]text").unwrap_err();
    assert!(matches!(err, LexError::InvalidValue { .. }));
}

#[test]
fn test_try_color_unclosed_color_fn() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let err = try_color("[rgb(255,0,0").unwrap_err();
    // Without the closing `)`, the lexer never sees a complete color
    // function, so it reports an unclosed tag (the outer `[` is never closed).
    assert!(matches!(err, LexError::UnclosedTag(..)));
}

#[test]
fn test_try_color_empty_bracket_is_consumed() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    // An empty bracket pair `[]` is parsed as a valid but empty tag.
    // It produces no ANSI and is silently consumed.
    let result = try_color("[]text");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "text\x1b[0m");
}

// ============================================================================
// color_runtime -- bleed variant
// ============================================================================

#[test]
fn test_color_runtime_bleed_no_trailing_reset() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_runtime("[red]text", true);
    // Bleed variant: no trailing reset
    assert_eq!(result, "\x1b[31mtext");
    assert!(!result.ends_with("\x1b[0m"));
}

#[test]
fn test_color_runtime_non_bleed_has_reset() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_runtime("[red]text", false);
    assert_eq!(result, "\x1b[31mtext\x1b[0m");
    assert!(result.ends_with("\x1b[0m"));
}

#[test]
fn test_color_runtime_bleed_chaining() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let part1 = color_runtime("[red]hello ", true);
    let part2 = color_runtime("[bold]world", true);
    // Chaining two bleed strings should produce valid ANSI
    let combined = format!("{}{}", part1, part2);
    assert_eq!(combined, "\x1b[31mhello \x1b[1mworld");
}

#[test]
fn test_color_runtime_plain_text_bleed() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_runtime("hello", true);
    // Plain text with bleed: no ANSI at all
    assert_eq!(result, "hello");
}

#[test]
fn test_color_runtime_plain_text_non_bleed() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_runtime("hello", false);
    assert_eq!(result, "hello\x1b[0m");
}

#[test]
fn test_color_runtime_empty_bleed() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_runtime("", true);
    assert_eq!(result, "");
}

#[test]
fn test_color_runtime_empty_non_bleed() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_runtime("", false);
    assert_eq!(result, "\x1b[0m");
}

// ============================================================================
// color / colorb (non-compile runtime functions)
// ============================================================================

#[test]
fn test_color_valid_input_returns_string() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_runtime("[green]ok", false);
    assert_eq!(result, "\x1b[32mok\x1b[0m");
}

#[test]
#[should_panic]
fn test_color_invalid_input_panics() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    color_runtime("[notacolor]text", false);
}

// ============================================================================
// cformat! / cformatb! -- edge cases
// ============================================================================

#[test]
fn test_cformat_positional() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let user = "Alice";
    let result = cformat!("[red]{}", user);
    assert_eq!(result, "\x1b[31mAlice\x1b[0m");
}

#[test]
fn test_cformat_explicit_named() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("[green]{greeting}", greeting = "Hello");
    assert_eq!(result, "\x1b[32mHello\x1b[0m");
}

#[test]
fn test_cformat_positional_still_works() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("[blue]{}", "world");
    assert_eq!(result, "\x1b[34mworld\x1b[0m");
}

#[test]
fn test_cformat_implicit_capture() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let name = "Bob";
    let result = cformat!("[dim]{name}");
    assert_eq!(result, "\x1b[2mBob\x1b[0m");
}

#[test]
fn test_cformat_no_args() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("[red]hello");
    assert_eq!(result, "\x1b[31mhello\x1b[0m");
}

#[test]
fn test_cformat_empty_format() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("");
    assert_eq!(result, "\x1b[0m");
}

#[test]
fn test_cformat_no_markup() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("plain {}", "text");
    assert_eq!(result, "plain text\x1b[0m");
}

#[test]
fn test_cformat_multiple_args() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("[red]{}, {} and {}", "a", "b", "c");
    assert_eq!(result, "\x1b[31ma, b and c\x1b[0m");
}

#[test]
fn test_cformat_format_spec() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("[green]{:.2}", 3.14159);
    assert_eq!(result, "\x1b[32m3.14\x1b[0m");
}

#[test]
fn test_cformat_mixed_markup_and_args() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformat!("[bold]{val}[/] normal", val = "bold text");
    assert_eq!(result, "\x1b[1mbold text\x1b[0m normal\x1b[0m");
}

// --- cformatb! (bleed) ---

#[test]
fn test_cformatb_no_trailing_reset() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformatb!("[red]hello {}", "world");
    assert_eq!(result, "\x1b[31mhello world");
    assert!(!result.ends_with("\x1b[0m"));
}

#[test]
fn test_cformatb_plain_text() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = cformatb!("plain {}", "text");
    assert_eq!(result, "plain text");
}

// ============================================================================
// Strip / escape macros
// ============================================================================

#[test]
fn test_unansi_named_arg() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let plain = unansi!("\x1b[31m{val}\x1b[0m", val = "stripped");
    assert_eq!(plain, "stripped");
}

#[test]
fn test_unansi_no_ansi() {
    let plain = unansi!("plain text");
    assert_eq!(plain, "plain text");
}

#[test]
fn test_unansi_empty() {
    let plain = unansi!("");
    assert_eq!(plain, "");
}

#[test]
fn test_unansi_only_ansi() {
    let plain = unansi!("\x1b[1m\x1b[31m");
    assert_eq!(plain, "");
}

#[test]
fn test_unansi_expr_variant() {
    let s = String::from("\x1b[32mgreen");
    let plain = unansi!(s);
    assert_eq!(plain, "green");
}

#[test]
fn test_untag_named_arg() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let safe = untag!("[bold]{name}", name = "x");
    assert_eq!(safe, "[[bold]]x");
}

#[test]
fn test_untag_no_tags() {
    let safe = untag!("plain text");
    assert_eq!(safe, "plain text");
}

#[test]
fn test_untag_all_tag_types() {
    let safe = untag!("[bold red]hello[/]");
    assert_eq!(safe, "[[bold red]]hello[[/]]");
}

#[test]
fn test_untag_already_escaped() {
    // `escape_tags` doubles every bracket, so `[` -> `[[`, already-escaped
    // `[[` -> `[[[[` (each `[` doubled independently).
    let safe = untag!("[[bold]]hello");
    assert_eq!(safe, "[[[[bold]]]]hello");
}

#[test]
fn test_untag_empty() {
    let safe = untag!("");
    assert_eq!(safe, "");
}

#[test]
fn test_unmarkup_implicit_capture() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let msg = "hey!";
    let stripped = unmarkup!("[bold red]{msg}");
    assert_eq!(stripped, "hey!");
}

#[test]
fn test_unmarkup_no_tags() {
    let stripped = unmarkup!("plain text");
    assert_eq!(stripped, "plain text");
}

#[test]
fn test_unmarkup_removes_tags_preserves_text() {
    let stripped = unmarkup!("[red]hello [green]world[/]!");
    assert_eq!(stripped, "hello world!");
}

#[test]
fn test_unmarkup_invalid_markup_is_unchanged() {
    let stripped = unmarkup!("[I'm unclosed");
    assert_eq!(stripped, "[I'm unclosed");
}

#[test]
fn test_unmarkup_empty() {
    let stripped = unmarkup!("");
    assert_eq!(stripped, "");
}

#[test]
fn test_unmarkup_only_tags() {
    let stripped = unmarkup!("[red][bold][/]");
    assert_eq!(stripped, "");
}

// ============================================================================
// cwrite / cwriteln / cwriteb / cwritebln (writer-based)
// ============================================================================
// Format-argument variants require the `compile` feature because the non-compile
// cwrite pattern `$(, $arg:tt)*` produces adjacent string literals that Rust
// concatenates, losing the argument/format-string distinction. No-argument calls
// work regardless. The compile variant delegates to `cformat!` which handles
// args correctly.

#[cfg(feature = "compile")]
#[test]
fn test_cwrite_to_buffer() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwrite!(buf, "[red]Error: {}", "critical").unwrap();
    assert_eq!(
        String::from_utf8(buf.into_inner()).unwrap(),
        "\x1b[31mError: critical\x1b[0m"
    );
}

#[test]
fn test_cwriteb_to_buffer() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwriteb!(buf, "[red]Error: ").unwrap();
    let output = String::from_utf8(buf.into_inner()).unwrap();
    assert_eq!(output, "\x1b[31mError: ");
    assert!(!output.ends_with("\x1b[0m"));
}

#[test]
fn test_cwritebln_to_buffer() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwritebln!(buf, "[bold red]Fatal").unwrap();
    let output = String::from_utf8(buf.into_inner()).unwrap();
    assert!(output.starts_with("\x1b[1m\x1b[31mFatal"));
    assert!(output.ends_with('\n'));
    assert!(!output.contains("\x1b[0m"));
}

#[test]
fn test_cwrite_no_args() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwrite!(buf, "[blue]text").unwrap();
    assert_eq!(
        String::from_utf8(buf.into_inner()).unwrap(),
        "\x1b[34mtext\x1b[0m"
    );
}

#[test]
fn test_cwrite_plain_text() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwrite!(buf, "hello").unwrap();
    assert_eq!(
        String::from_utf8(buf.into_inner()).unwrap(),
        "hello\x1b[0m"
    );
}

#[cfg(feature = "compile")]
#[test]
fn test_cwrite_named_arg() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwrite!(buf, "[red]{err}", err = "fail").unwrap();
    assert_eq!(
        String::from_utf8(buf.into_inner()).unwrap(),
        "\x1b[31mfail\x1b[0m"
    );
}

#[cfg(feature = "compile")]
#[test]
fn test_cwrite_multiple_args() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwrite!(buf, "[bold]{}: {} errors", "ERR", 5).unwrap();
    assert_eq!(
        String::from_utf8(buf.into_inner()).unwrap(),
        "\x1b[1mERR: 5 errors\x1b[0m"
    );
}

#[test]
fn test_cwriteb_chained_then_cwrite() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwriteb!(buf, "[red]error: ").unwrap();
    cwrite!(buf, "something broke").unwrap();
    let output = String::from_utf8(buf.into_inner()).unwrap();
    assert_eq!(output, "\x1b[31merror: something broke\x1b[0m");
}

#[cfg(feature = "compile")]
#[test]
fn test_cwriteln_to_buffer() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let mut buf = Cursor::new(Vec::new());
    cwriteln!(buf, "[green]Success: {}", "done").unwrap();
    let output = String::from_utf8(buf.into_inner()).unwrap();
    assert!(output.starts_with("\x1b[32mSuccess: done\x1b[0m"));
    assert!(output.ends_with('\n'));
}

// ============================================================================
// FarbenStr
// ============================================================================

#[test]
fn test_farben_str_resolve_with_color() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let s = FarbenStr {
        styled: "\x1b[31mhello\x1b[0m",
    };
    let resolved = s.resolve();
    assert_eq!(resolved, "\x1b[31mhello\x1b[0m");
}

#[test]
fn test_farben_str_display_with_color() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let s = FarbenStr {
        styled: "\x1b[32mgreen\x1b[0m",
    };
    assert_eq!(format!("{}", s), "\x1b[32mgreen\x1b[0m");
}

// ============================================================================
// Deprecated macros (still need to compile and function)
// ============================================================================

#[test]
#[allow(deprecated)]
fn test_color_fmt_deprecated_still_works() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let result = color_fmt!("[red]{}", "test");
    assert_eq!(result, "\x1b[31mtest\x1b[0m");
}

#[test]
#[allow(deprecated)]
fn test_ansi_strip_deprecated() {
    let plain = ansi_strip!("\x1b[1mtext\x1b[0m");
    assert_eq!(plain, "text");
}

#[test]
#[allow(deprecated)]
fn test_markup_strip_deprecated() {
    let stripped = markup_strip!("[bold]text[/]");
    assert_eq!(stripped, "text");
}

// ============================================================================
// Format feature tests (style! / prefix!)
// ============================================================================

#[cfg(feature = "format")]
#[test]
fn test_style_and_prefix() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    style!("test-danger", "[bold red]");
    prefix!("test-danger", "!! ");
    let result = try_color("[test-danger]critical").unwrap();
    // The prefix is injected as literal text after the ANSI escape sequence:
    assert_eq!(result, "\x1b[1m\x1b[31m!! critical\x1b[0m");
}

#[cfg(feature = "format")]
#[test]
fn test_style_retrieval() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    style!("test-warn", "[yellow]");
    let result = try_color("[test-warn]caution").unwrap();
    assert_eq!(result, "\x1b[33mcaution\x1b[0m");
}

// ============================================================================
// core::tokenize helper
// ============================================================================

#[test]
fn test_core_tokenize_plain() {
    let tokens = crate::core::tokenize("hello").unwrap();
    assert_eq!(tokens.len(), 1);
}

#[test]
fn test_core_tokenize_tag() {
    let tokens = crate::core::tokenize("[red]text").unwrap();
    assert_eq!(tokens.len(), 2); // tag + text
}

// ============================================================================
// color_enabled helper
// ============================================================================

#[test]
fn test_color_enabled_with_force_color() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    assert!(color_enabled());
}

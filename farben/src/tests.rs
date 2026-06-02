//! Integration tests for `color`, `try_color`, and related functions.

use super::*;

// --- try_color ---

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

// --- color ---

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

// --- format-arg styles (implicit capture & named) ---

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
fn test_unansi_named_arg() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let plain = unansi!("\x1b[31m{val}\x1b[0m", val = "stripped");
    assert_eq!(plain, "stripped");
}

#[test]
fn test_untag_named_arg() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let safe = untag!("[bold]{name}", name = "x");
    assert_eq!(safe, "[[bold]]x");
}

#[test]
fn test_unmarkup_implicit_capture() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let msg = "hey!";
    let stripped = unmarkup!("[bold red]{msg}");
    assert_eq!(stripped, "hey!");
}

#[test]
fn test_cprint_implicit_capture() {
    unsafe { std::env::set_var("FORCE_COLOR", "1") };
    let name = "Bob";
    let result = cformat!("[dim]{}", name);
    assert_eq!(result, "\x1b[2mBob\x1b[0m");
}

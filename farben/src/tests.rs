use super::*;

// --- try_color ---

#[test]
fn test_try_color_named_color() {
    let result = try_color("[red]I'm red!");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[31mI'm red!\x1b[0m");
}

#[test]
fn test_try_color_appends_trailing_reset() {
    let result = try_color("[blue]text");
    assert!(result.is_ok());
    assert!(result.unwrap().ends_with("\x1b[0m"));
}

#[test]
fn test_try_color_plain_text_no_tags() {
    let result = try_color("no markup here");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "no markup here\x1b[0m");
}

#[test]
fn test_try_color_empty_string() {
    let result = try_color("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[0m");
}

#[test]
fn test_try_color_invalid_tag_returns_error() {
    let result = try_color("[error]text");
    assert!(result.is_err());
}

#[test]
fn test_try_color_unclosed_tag_returns_error() {
    let result = try_color("[red");
    assert!(result.is_err());
}

#[test]
fn test_try_color_rgb_color() {
    let result = try_color("[rgb(255,0,0)]red");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[38;2;255;0;0mred\x1b[0m");
}

#[test]
fn test_try_color_bold_and_named_color() {
    let result = try_color("[bold red]hi");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[1m\x1b[31mhi\x1b[0m");
}

#[test]
fn test_try_color_escaped_bracket() {
    let result = try_color("\\[not a tag]");
    assert!(result.is_ok());
    assert!(result.unwrap().starts_with('['));
}

#[test]
fn test_try_color_inline_reset() {
    let result = try_color("[red]before[/]after");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\x1b[31mbefore\x1b[0mafter\x1b[0m");
}

// --- color ---

#[test]
fn test_color_valid_input_returns_string() {
    let result = color("[green]ok");
    assert_eq!(result, "\x1b[32mok\x1b[0m");
}

#[test]
#[should_panic]
fn test_color_invalid_input_panics() {
    color("[notacolor]text");
}

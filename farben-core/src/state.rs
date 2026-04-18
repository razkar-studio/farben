//! Per-thread persistent style state.
//!
//! Style "bleed" calls (e.g. cprintb!) intentionally don't reset at the end,
//! so subsequent calls see the leftover ANSI styles in the terminal. To handle
//! `[/red]`-style targeted resets correctly across calls, we persist the
//! active style stack between renders.
//!
//! Per-thread because terminal output is inherently per-thread state — two
//! threads writing to the same stdout are already racing; they shouldn't also
//! be racing over style state.

use std::cell::RefCell;
use crate::lexer::TagType;

thread_local! {
    static ACTIVE_STACK: RefCell<Vec<TagType>> = const { RefCell::new(Vec::new()) };
}

/// Returns a clone of the current persisted stack.
/// Used by the parser at the start of `render` to resume from prior state.
pub fn active_stack() -> Vec<TagType> {
    ACTIVE_STACK.with(|s| s.borrow().clone())
}

/// Replaces the persisted stack.
/// Used by the parser at the end of `render` to save its working state.
pub fn set_active_stack(new_stack: Vec<TagType>) {
    ACTIVE_STACK.with(|s| *s.borrow_mut() = new_stack);
}

/// Clears the persisted stack.
/// Called by the upper layer after a non-bleed print (which appends \x1b[0m,
/// so the terminal is back to a clean slate).
pub fn clear_active_stack() {
    ACTIVE_STACK.with(|s| s.borrow_mut().clear());
}

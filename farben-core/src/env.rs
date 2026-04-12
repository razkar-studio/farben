//! Runtime detection of whether ANSI color output should be enabled.
//!
//! Respects the [`NO_COLOR`](https://no-color.org) and
//! [`FORCE_COLOR`](https://force-color.org) environment variable conventions,
//! in that order of precedence, before falling back to TTY detection.
//!
//! The result is computed once per process and cached in a [`OnceLock`].

use std::sync::OnceLock;

static COLOR_ENABLED: OnceLock<bool> = OnceLock::new();

/// Returns whether ANSI color output is enabled for this process.
///
/// The decision is made once and cached. Subsequent calls return the cached
/// value without re-checking the environment.
///
/// # Detection order
///
/// 1. `NO_COLOR` set (any value) -> `false`
/// 2. `FORCE_COLOR` set (any value) -> `true`
/// 3. stdout is a TTY (Unix or Windows) -> `true`, otherwise `false`
pub fn color_enabled() -> bool {
    *COLOR_ENABLED.get_or_init(|| {
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }

        if std::env::var("FORCE_COLOR").is_ok() {
            return true;
        }

        is_tty()
    })
}

#[cfg(unix)]
unsafe extern "C" {
    fn isatty(fd: i32) -> i32;
}

/// Reports whether stdout is connected to a terminal.
///
/// - Unix: calls `isatty(1)` via the POSIX C interface.
/// - Windows: calls `GetStdHandle(STD_OUTPUT_HANDLE)` and checks that
///   `GetConsoleMode` succeeds, which fails on redirected handles.
/// - All other targets: returns `false`.
fn is_tty() -> bool {
    #[cfg(unix)]
    {
        // SAFETY: fd 1 is stdout, which is always a valid open file descriptor for this process.
        // isatty() is async-signal-safe and does not mutate any Rust-owned memory.
        unsafe { isatty(1) != 0 }
    }

    #[cfg(all(not(unix), windows))]
    {
        // SAFETY: is_tty_windows only calls Win32 handle query APIs. GetStdHandle returns a
        // pseudo-handle owned by the OS, not the caller, so it must not be closed. GetConsoleMode
        // writes into a local u32 on the stack with no aliasing concerns.
        unsafe { is_tty_windows() }
    }

    #[cfg(all(not(unix), not(windows)))]
    {
        return false;
    }
}

#[cfg(windows)]
unsafe extern "system" {
    fn GetStdHandle(nStdHandle: u32) -> *mut u8;
    fn GetConsoleMode(hConsoleHandle: *mut u8, lpMode: *mut u32) -> i32;
}

#[cfg(windows)]
fn is_tty_windows() -> bool {
    const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5;
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode: u32 = 0;
        GetConsoleMode(handle, &mut mode) != 0
    }
}

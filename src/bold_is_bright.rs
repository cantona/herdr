//! Whether bold + one of the 8 base ANSI colours renders as its bright variant.
//!
//! Most terminals (xterm, rio, alacritty, gnome-terminal) promote bold+30..37
//! to the bright palette entry -- a habit inherited from PC hardware where
//! "bold" was an intensity bit. Ghostty renders SGR literally instead, so
//! defaults written for that convention (Ubuntu's LS_COLORS `di=01;34`, the
//! stock PS1) look duller inside a pane than in the host terminal.
//!
//! Process-global like kitty_graphics: the render path has no config handle.

use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);

pub(crate) fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
}

pub(crate) fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Map palette index 0-7 to its bright counterpart 8-15; leave the rest alone.
pub(crate) fn brighten_index(index: u8) -> u8 {
    if index < 8 {
        index + 8
    } else {
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_first_eight_are_promoted() {
        assert_eq!(brighten_index(4), 12);
        assert_eq!(brighten_index(0), 8);
        assert_eq!(brighten_index(7), 15);
        // already bright, or 256-colour cube: untouched
        assert_eq!(brighten_index(8), 8);
        assert_eq!(brighten_index(12), 12);
        assert_eq!(brighten_index(200), 200);
    }

    #[test]
    fn disabled_by_default() {
        assert!(!is_enabled());
    }
}

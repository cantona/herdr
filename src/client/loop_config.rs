use super::*;

pub(super) struct ClientLoopConfig {
    pub(super) sound_config: crate::config::SoundConfig,
    pub(super) copy_to_primary: bool,
    pub(super) mouse_scroll_lines: usize,
    pub(super) redraw_on_focus_gained: bool,
    pub(super) host_cursor: crate::config::HostCursorModeConfig,
    pub(super) kitty_graphics_enabled: bool,
    pub(super) pixel_geometry_enabled: bool,
    pub(super) pixel_geometry_fallback: bool,
    pub(super) mouse_capture_active: bool,
    pub(super) host_escape_disambiguation_active: bool,
    /// The host terminal answered DECRQM for mode 1016, so it can report mouse pixels.
    pub(super) host_sgr_pixel_mouse_supported: bool,
    /// The host terminal answered OK to a zlib-deflated Kitty graphics query.
    pub(super) host_kitty_zlib_supported: bool,
    pub(super) initial_host_input: Vec<u8>,
    pub(super) endpoint_keybindings: bool,
    pub(super) remote_image_paste_key:
        Option<(crossterm::event::KeyCode, crossterm::event::KeyModifiers)>,
    pub(super) shell_config: Option<shell::ClientShellConfig>,
}

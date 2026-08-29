use super::*;

pub(super) fn decode_clipboard_payload(data: &str) -> Option<Vec<u8>> {
    use base64::Engine;

    base64::engine::general_purpose::STANDARD.decode(data).ok()
}

pub(super) fn forward_clipboard(data: &str, copy_to_primary: bool) -> bool {
    let Some(bytes) = decode_clipboard_payload(data) else {
        warn!("received invalid clipboard payload from server");
        return false;
    };
    crate::selection::write_selection_bytes(&bytes, copy_to_primary)
}

use tauri::{AppHandle, RunEvent};

/// Global runtime event hook. Currently a no-op; reserved for future
/// menu, tray, and global-hotkey handlers.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn handle(_app: &AppHandle, _event: RunEvent) {}

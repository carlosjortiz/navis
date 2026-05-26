use tauri::{AppHandle, Manager as _};

use crate::lifecycle::SELECTOR_LABEL;

/// Second-launch callback: bring the selector forward if it still exists;
/// otherwise focus whatever window is open. The `unminimize → show → set_focus`
/// sequence works around Win32's foreground-steal restrictions.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn handle(app: &AppHandle, _argv: Vec<String>, _cwd: String) {
    if let Some(win) = app.get_webview_window(SELECTOR_LABEL) {
        let _ = win.unminimize();
        let _ = win.show();
        let _ = win.set_focus();
        return;
    }
    if let Some(win) = app.webview_windows().values().next() {
        let _ = win.unminimize();
        let _ = win.show();
        let _ = win.set_focus();
    }
}

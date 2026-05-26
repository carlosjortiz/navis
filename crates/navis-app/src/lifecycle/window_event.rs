use tauri::{Emitter as _, Manager as _, Window, WindowEvent};

use crate::lifecycle::workspace_name_from_label;
use crate::state::AppState;

pub(crate) const OPEN_WORKSPACES_EVENT: &str = "open-workspaces-changed";

pub(crate) fn handle(window: &Window, event: &WindowEvent) {
    if !matches!(event, WindowEvent::Destroyed) {
        return;
    }

    let label = window.label();
    let Some(name) = workspace_name_from_label(label) else {
        return;
    };

    let app = window.app_handle();
    let state = app.state::<AppState>();

    let snapshot: Vec<String> = {
        let mut open = state.open_workspaces.lock();
        open.remove(name);
        open.iter().cloned().collect()
    };

    let _ = app.emit(OPEN_WORKSPACES_EVENT, &snapshot);

    // macOS keeps the app alive after the last window closes (NSApplication
    // default); other platforms already exit. Force parity here.
    #[cfg(target_os = "macos")]
    if snapshot.is_empty() {
        app.exit(0);
    }
}

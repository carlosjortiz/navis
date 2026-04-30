//! `RunEvent` handler for the Navis Tauri app.
//!
//! Idiomatic separation per `nv-tauri-best-practices` rule #4: the handler is
//! a free function, not a closure, so it can be unit-tested in isolation.

use tauri::{AppHandle, Manager, RunEvent, WindowEvent};

/// Run-loop event handler. Called by Tauri after every event-loop tick.
///
/// Native close behavior: when a webview window is destroyed and no visible
/// windows remain in this process, exit. Other instances of the app run as
/// separate processes and are unaffected.
pub fn handle(app: &AppHandle, event: RunEvent) {
    if let RunEvent::WindowEvent {
        event: WindowEvent::Destroyed,
        ..
    } = event
    {
        let any_visible = app
            .webview_windows()
            .values()
            .any(|w| w.is_visible().unwrap_or(false));
        if !any_visible {
            app.exit(0);
        }
    }
}

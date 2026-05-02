use anyhow::Context as _;
use navis_runtime::settings::Settings;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::error::CommandResult;

// Settings stays well under 1 KB today, so JSON5 parse + write run inline on
// the runtime worker. If the file grows past ~10 KB or load/save latency
// exceeds a few ms, wrap the navis_runtime call in
// `tauri::async_runtime::spawn_blocking` to free the worker thread.
#[tauri::command]
#[specta::specta]
pub async fn get_settings() -> CommandResult<Settings> {
    navis_runtime::settings::load_settings().map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn save_settings(settings: Settings) -> CommandResult<()> {
    navis_runtime::settings::save_settings(&settings).map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn open_settings(app: AppHandle) -> CommandResult<()> {
    const LABEL: &str = "navis-settings";

    if let Some(existing) = app.get_webview_window(LABEL) {
        existing.unminimize().context("failed to unminimize settings window")?;
        existing.show().context("failed to show settings window")?;
        existing.set_focus().context("failed to focus settings window")?;
        return Ok(());
    }

    WebviewWindowBuilder::new(&app, LABEL, WebviewUrl::App("settings.html".into()))
        .title("Navis — Settings")
        .inner_size(640.0, 480.0)
        .decorations(false)
        .transparent(true)
        .build()
        .context("failed to build settings window")?;

    Ok(())
}

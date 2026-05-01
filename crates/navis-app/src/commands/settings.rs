use anyhow::Context as _;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::error::CommandResult;

#[tauri::command]
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

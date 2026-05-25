use anyhow::Context as _;
use navis_runtime::workspace::Workspace;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::error::CommandResult;

#[tauri::command]
#[specta::specta]
pub async fn open_workspace(app: AppHandle, name: String) -> CommandResult<()> {
    // Validate existence on disk before any window-side work.
    navis_runtime::workspace::load_workspace(&name)
        .with_context(|| format!("failed to load workspace {name:?}"))?;

    let label = format!("navis-{name}");

    if let Some(existing) = app.get_webview_window(&label) {
        existing.unminimize().context("failed to unminimize workspace window")?;
        existing.show().context("failed to show workspace window")?;
        existing.set_focus().context("failed to focus workspace window")?;
        return Ok(());
    }

    // Pass the workspace name to the new window via URL query so the FE can
    // request its context. URL encoding covers spaces, Unicode, and any other
    // filesystem-safe character allowed in workspace names.
    let url_path = format!("index.html?workspace={}", urlencoding::encode(&name));

    // Build the main window first; closing the selector before build() returns
    // can trigger Tauri's "last window destroyed" exit if the timing is unlucky.
    WebviewWindowBuilder::new(&app, &label, WebviewUrl::App(url_path.into()))
        .title(format!("Navis — {name}"))
        .inner_size(1280.0, 800.0)
        .decorations(false)
        .transparent(true)
        .build()
        .context("failed to build workspace window")?;

    if let Some(selector) = app.get_webview_window("navis-ws-selector") {
        selector
            .close()
            .context("failed to close workspace selector")?;
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_workspace(name: String) -> CommandResult<Workspace> {
    navis_runtime::workspace::load_workspace(&name).map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn list_workspaces() -> CommandResult<Vec<Workspace>> {
    navis_runtime::workspace::list_workspaces().map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn create_workspace(name: String, description: Option<String>) -> CommandResult<Workspace> {
    navis_runtime::workspace::create_workspace(&name, description.as_deref()).map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn rename_workspace(old: String, new_name: String) -> CommandResult<Workspace> {
    navis_runtime::workspace::rename_workspace(&old, &new_name).map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_workspace(name: String) -> CommandResult<()> {
    navis_runtime::workspace::delete_workspace(&name).map_err(Into::into)
}

mod commands;
mod error;

use tauri::Manager;
use tauri_specta::{Builder, collect_commands};

/// Boots the Tauri runtime and runs the Navis application until exit.
///
/// # Panics
///
/// Panics if the Tauri context fails to initialize or the runtime cannot
/// be started.
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::workspaces::open_workspace,
        commands::workspaces::list_workspaces,
        commands::workspaces::create_workspace,
        commands::workspaces::rename_workspace,
        commands::workspaces::delete_workspace,
        commands::workspaces::get_workspace,
        commands::settings::open_settings,
        commands::settings::get_settings,
        commands::settings::save_settings,
    ]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../../src/bindings.ts",
        )
        .expect("failed to export typescript bindings");

    let mut builder = tauri::Builder::default().invoke_handler(specta_builder.invoke_handler());

    #[cfg(desktop)]
    {
        // Second-launch callback: bring the selector forward if it still
        // exists; otherwise focus whatever window is open. The
        // unminimize → show → set_focus sequence works around Win32's
        // foreground-steal restrictions.
        builder = builder.plugin(tauri_plugin_single_instance::init(
            |app, _argv, _cwd| {
                if let Some(win) = app.get_webview_window("navis-ws-selector") {
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
            },
        ));
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

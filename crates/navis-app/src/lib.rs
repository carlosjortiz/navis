mod commands;
mod error;

use tauri::Manager;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let mut builder = tauri::Builder::default();

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
                for win in app.webview_windows().values() {
                    let _ = win.unminimize();
                    let _ = win.show();
                    let _ = win.set_focus();
                    break;
                }
            },
        ));
    }

    builder
        .invoke_handler(tauri::generate_handler![
            commands::workspaces::open_workspace,
            commands::settings::open_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

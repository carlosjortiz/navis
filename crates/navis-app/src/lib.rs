//! Navis Tauri application — library entry.
//!
//! `main.rs` is a 3-line stub that calls [`run`]. Real assembly lives here so
//! the lifecycle handlers and (future) commands can be unit-tested without
//! launching the binary.

mod lifecycle;

/// Build and run the Navis Tauri app.
pub fn run() {
    tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(lifecycle::run_event::handle);
}

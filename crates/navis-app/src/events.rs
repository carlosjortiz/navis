use navis_runtime::settings::Settings;
use serde::{Deserialize, Serialize};

/// Broadcast after `save_settings` persists a new value, so every window can
/// re-apply theme, language, and opacity without an extra IPC roundtrip.
#[derive(Clone, Debug, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct SettingsChanged(pub Settings);

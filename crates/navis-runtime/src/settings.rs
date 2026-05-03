use std::path::{Path, PathBuf};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

use crate::io::{read_json5, write_json5};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Es,
}

/// Persistent user settings stored at `~/.navis/settings.json5`.
///
/// Forward-compatibility rules when evolving this struct:
/// - **Adding a field**: include `#[serde(default = "fn")]` so old files
///   that lack the field still deserialize cleanly.
/// - **Renaming a field**: use `#[serde(alias = "old_name", rename = "new_name")]`
///   so old files keep working; the next save rewrites with the new name.
/// - **Removing a field**: just delete it from the struct. serde ignores
///   unknown JSON fields by default.
///
/// For non-trivial migrations (type changes, splits, joins) see
/// `carlosjortiz/navis-prd#37`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct Settings {
    #[serde(default = "default_theme")]
    pub theme: Theme,
    #[serde(default = "default_language")]
    pub language: Language,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
}

fn default_theme() -> Theme {
    Theme::System
}

fn default_language() -> Language {
    Language::En
}

fn default_opacity() -> f32 {
    1.0
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            language: default_language(),
            opacity: default_opacity(),
        }
    }
}

fn settings_path() -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir().context("failed to determine home directory")?;
    Ok(home.join(".navis").join("settings.json5"))
}

/// Loads settings from `~/.navis/settings.json5`, creating the file with
/// defaults if missing.
///
/// # Errors
///
/// Returns an error if the home directory cannot be resolved or the file is
/// malformed.
pub fn load_settings() -> anyhow::Result<Settings> {
    load_settings_at(&settings_path()?)
}

/// Persists `settings` to `~/.navis/settings.json5`.
///
/// # Errors
///
/// Returns an error if the home directory cannot be resolved or the atomic
/// write fails.
pub fn save_settings(settings: &Settings) -> anyhow::Result<()> {
    save_settings_at(&settings_path()?, settings)
}

pub(crate) fn load_settings_at(path: &Path) -> anyhow::Result<Settings> {
    if !path.exists() {
        let defaults = Settings::default();
        save_settings_at(path, &defaults)
            .with_context(|| format!("failed to seed defaults at {}", path.display()))?;
        return Ok(defaults);
    }

    read_json5(path).with_context(|| format!("failed to load settings from {}", path.display()))
}

pub(crate) fn save_settings_at(path: &Path, settings: &Settings) -> anyhow::Result<()> {
    write_json5(path, settings)
        .with_context(|| format!("failed to save settings to {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_returned_when_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json5");

        assert!(!path.exists());
        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(loaded, Settings::default());
        assert!(path.exists(), "missing file should be seeded with defaults");
    }

    #[test]
    fn valid_file_loads_to_struct() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json5");
        let raw = r#"{
            // user-chosen
            theme: "dark",
            language: "es",
            opacity: 0.85,
        }"#;
        std::fs::write(&path, raw).unwrap();

        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(
            loaded,
            Settings {
                theme: Theme::Dark,
                language: Language::Es,
                opacity: 0.85,
            }
        );
    }

    #[test]
    fn defaults_filled_when_fields_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json5");
        let raw = r#"{ theme: "dark" }"#;
        std::fs::write(&path, raw).unwrap();

        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(loaded.theme, Theme::Dark);
        assert_eq!(loaded.language, default_language());
        assert!((loaded.opacity - default_opacity()).abs() < f32::EPSILON);
    }

    #[test]
    fn unknown_fields_are_ignored() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json5");
        let raw = r#"{
            theme: "system",
            language: "en",
            opacity: 1.0,
            legacy_field: "from-an-older-navis",
            schema_version: 1,
        }"#;
        std::fs::write(&path, raw).unwrap();

        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(loaded, Settings::default());
    }

    #[test]
    fn roundtrip_preserves_values() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json5");
        let original = Settings::default();

        save_settings_at(&path, &original).unwrap();
        let loaded = load_settings_at(&path).unwrap();

        assert_eq!(loaded, original);
    }
}

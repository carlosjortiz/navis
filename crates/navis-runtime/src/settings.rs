use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail};
use serde::{Deserialize, Serialize};

use crate::io::{read_json5, write_json5};

pub const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Es,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub language: Language,
    pub opacity: f32,
    pub schema_version: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: Language::En,
            opacity: 1.0,
            schema_version: CURRENT_SCHEMA_VERSION,
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
/// Returns an error if the home directory cannot be resolved, the file is
/// malformed, or the persisted `schema_version` does not match the current
/// version.
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

    let settings: Settings = read_json5(path)
        .with_context(|| format!("failed to load settings from {}", path.display()))?;

    if settings.schema_version != CURRENT_SCHEMA_VERSION {
        bail!(
            "settings schema version mismatch in {}: file has {}, current is {}",
            path.display(),
            settings.schema_version,
            CURRENT_SCHEMA_VERSION
        );
    }

    Ok(settings)
}

pub(crate) fn save_settings_at(path: &Path, settings: &Settings) -> anyhow::Result<()> {
    write_json5(path, settings)
        .with_context(|| format!("failed to save settings to {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_have_current_schema_version() {
        assert_eq!(Settings::default().schema_version, CURRENT_SCHEMA_VERSION);
    }

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
            schema_version: 1,
        }"#;
        std::fs::write(&path, raw).unwrap();

        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(
            loaded,
            Settings {
                theme: Theme::Dark,
                language: Language::Es,
                opacity: 0.85,
                schema_version: 1,
            }
        );
    }

    #[test]
    fn schema_version_mismatch_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json5");
        let raw = r#"{
            theme: "system",
            language: "en",
            opacity: 1.0,
            schema_version: 99,
        }"#;
        std::fs::write(&path, raw).unwrap();

        let err = load_settings_at(&path).unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("schema version mismatch"), "got: {msg}");
        assert!(msg.contains("99"), "should mention file version: {msg}");
        assert!(msg.contains('1'), "should mention current version: {msg}");
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

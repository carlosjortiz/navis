use std::io::Write;
use std::path::Path;

use anyhow::{Context as _, bail};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tempfile::NamedTempFile;

mod surgical;

/// Reads a JSON5 file at `path` and deserializes it into `T`.
///
/// # Errors
///
/// Returns an error if the file cannot be read or if the content is not
/// valid JSON5 deserializable into `T`.
pub fn read_json5<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    json5::from_str(&raw)
        .with_context(|| format!("failed to parse {} as JSON5", path.display()))
}

/// Writes `value` to `path` as JSON5.
///
/// If `path` already exists, applies surgical edits to the existing text so
/// comments, key order, and whitespace are preserved wherever possible. If the
/// existing file is malformed JSON5 the call fails rather than silently
/// rewriting the whole file. Otherwise writes a fresh formatted document.
///
/// All writes go through a temp file in the same directory and `persist` to
/// land atomically.
///
/// # Errors
///
/// Returns an error if serialization fails, if the existing file cannot be
/// read or parsed, if surgical edits cannot be derived, or if the atomic
/// write fails.
pub fn write_json5<T: Serialize>(path: &Path, value: &T) -> anyhow::Result<()> {
    let new_value = serde_json::to_value(value).context("failed to serialize value to JSON")?;

    if path.exists() {
        let old_raw = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read existing {}", path.display()))?;
        let old_tree = navis_parser::parse(&old_raw)
            .with_context(|| format!("failed to parse existing {}", path.display()))?;
        if old_tree.root_node().has_error() {
            bail!(
                "existing file {} is malformed; refusing surgical edit",
                path.display()
            );
        }
        let old_value: serde_json::Value = json5::from_str(&old_raw)
            .with_context(|| format!("failed to parse {} as typed JSON5", path.display()))?;
        let new_text = surgical::apply(&old_raw, &old_tree, &old_value, &new_value)?;
        atomic_write(path, &new_text)
    } else {
        let text = serde_json::to_string_pretty(&new_value)
            .context("failed to format JSON output")?;
        atomic_write(path, &text)
    }
}

fn atomic_write(path: &Path, contents: &str) -> anyhow::Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)
        .with_context(|| format!("failed to create parent of {}", path.display()))?;

    let mut tmp = NamedTempFile::new_in(parent)
        .with_context(|| format!("failed to create temp file in {}", parent.display()))?;
    tmp.write_all(contents.as_bytes())
        .context("failed to write temp file")?;
    tmp.persist(path)
        .map_err(|e| anyhow::anyhow!("failed to persist atomic write: {}", e.error))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::io::Write;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Settings {
        theme: String,
        opacity: f64,
        retries: i32,
    }

    fn write_temp_with(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn reads_typed_struct_with_comments() {
        let f = write_temp_with(
            r#"{
            // user theme
            theme: "dark",
            opacity: 0.85,
            retries: 3,
        }"#,
        );
        let s: Settings = read_json5(f.path()).unwrap();
        assert_eq!(
            s,
            Settings {
                theme: "dark".into(),
                opacity: 0.85,
                retries: 3
            }
        );
    }

    #[test]
    fn errors_on_missing_file() {
        let err = read_json5::<Settings>(Path::new("/no/such/path/anywhere.json5")).unwrap_err();
        assert!(format!("{err:#}").contains("failed to read"));
    }

    #[test]
    fn errors_on_malformed_json5() {
        let f = write_temp_with(r#"{ key: }"#);
        let err = read_json5::<Settings>(f.path()).unwrap_err();
        assert!(format!("{err:#}").contains("failed to parse"));
    }

    #[test]
    fn fresh_write_then_read_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json5");
        let value = Settings {
            theme: "light".into(),
            opacity: 1.0,
            retries: 5,
        };
        write_json5(&path, &value).unwrap();
        let read: Settings = read_json5(&path).unwrap();
        assert_eq!(value, read);
    }

    #[test]
    fn update_existing_preserves_comments_in_raw_text() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("s.json5");
        let initial = r#"{
            // chosen by user
            theme: "light",
            opacity: 1.0,
            retries: 1,
        }"#;
        std::fs::write(&path, initial).unwrap();

        let updated = Settings {
            theme: "dark".into(),
            opacity: 1.0,
            retries: 1,
        };
        write_json5(&path, &updated).unwrap();

        let raw = std::fs::read_to_string(&path).unwrap();
        assert!(
            raw.contains("// chosen by user"),
            "comment should survive: {raw}"
        );
        assert!(
            raw.contains(r#""dark""#) || raw.contains("dark"),
            "theme should be updated: {raw}"
        );
    }

    #[test]
    fn round_trip_with_no_change_is_byte_stable() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("stable.json5");
        let initial = "{\n  // hi\n  theme: \"light\",\n  opacity: 1.0,\n  retries: 1,\n}";
        std::fs::write(&path, initial).unwrap();
        let value: Settings = read_json5(&path).unwrap();
        write_json5(&path, &value).unwrap();
        let raw = std::fs::read_to_string(&path).unwrap();
        assert_eq!(raw, initial);
    }
}

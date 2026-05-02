use std::path::Path;

use anyhow::Context as _;
use serde::de::DeserializeOwned;

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

    fn write_temp(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn reads_typed_struct_with_comments() {
        let f = write_temp(
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
        let f = write_temp(r#"{ key: }"#);
        let err = read_json5::<Settings>(f.path()).unwrap_err();
        assert!(format!("{err:#}").contains("failed to parse"));
    }
}

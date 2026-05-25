use std::path::{Path, PathBuf};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

use crate::io::{read_json5, write_json5};

/// Internal on-disk shape for a workspace — keeps `name` out of the file.
///
/// Disk I/O always goes through this struct so that `Workspace` (the public
/// IPC type) can serialize all its fields without side-effects on the file.
#[derive(Debug, Default, Serialize, Deserialize)]
struct WorkspaceFile {
    #[serde(default)]
    description: Option<String>,
}

/// A named container for related requests, stored as a subdirectory under
/// `~/.navis/workspaces/<name>/`.
///
/// The `name` field is NOT written to `workspace.json5` — it is derived from
/// the directory name at load time and the separation is enforced by using the
/// private `WorkspaceFile` struct for all disk reads and writes. Only
/// `description` (and any future fields) live on disk.
///
/// Forward-compatibility rules when evolving this struct:
/// - **Adding a field**: include `#[serde(default)]` so files that pre-date the
///   field still deserialize cleanly.
/// - **Renaming a field**: use `#[serde(alias = "old_name")]` so old files keep
///   working; the next save rewrites with the new name.
/// - **Removing a field**: just delete it. serde ignores unknown JSON fields by
///   default.
///
/// For non-trivial migrations (type-change, split, join) see navis-prd#37.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct Workspace {
    /// The directory name under `~/.navis/workspaces/`. Populated at load time.
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

fn workspaces_root() -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir().context("failed to determine home directory")?;
    Ok(home.join(".navis").join("workspaces"))
}

fn workspace_dir(root: &Path, name: &str) -> PathBuf {
    root.join(name)
}

fn workspace_file(root: &Path, name: &str) -> PathBuf {
    workspace_dir(root, name).join("workspace.json5")
}

// ---------------------------------------------------------------------------
// Public API — thin wrappers that resolve the real home dir
// ---------------------------------------------------------------------------

/// Returns every workspace found under `~/.navis/workspaces/`, sorted
/// alphabetically.
///
/// A missing workspaces directory is treated as an empty list, not an error.
///
/// # Errors
///
/// Returns an error if the home directory cannot be resolved or the workspaces
/// directory cannot be read.
pub fn list_workspaces() -> anyhow::Result<Vec<Workspace>> {
    list_workspaces_at(&workspaces_root()?)
}

/// Creates a new workspace directory and writes its metadata file.
///
/// # Errors
///
/// Returns an error if `name` is invalid, the workspace already exists, or the
/// filesystem write fails.
pub fn create_workspace(name: &str, description: Option<&str>) -> anyhow::Result<Workspace> {
    create_workspace_at(&workspaces_root()?, name, description)
}

/// Renames a workspace by moving its directory atomically.
///
/// The returned `Workspace` reflects the new name and preserves the existing
/// description.
///
/// # Errors
///
/// Returns an error if `new` is invalid, the source does not exist, the target
/// already exists, or the rename fails.
pub fn rename_workspace(old: &str, new: &str) -> anyhow::Result<Workspace> {
    rename_workspace_at(&workspaces_root()?, old, new)
}

/// Removes a workspace directory and all its contents.
///
/// Idempotent: returns `Ok(())` when the workspace does not exist.
///
/// # Errors
///
/// Returns an error if the directory exists but cannot be removed.
pub fn delete_workspace(name: &str) -> anyhow::Result<()> {
    delete_workspace_at(&workspaces_root()?, name)
}

/// Loads a single workspace by name.
///
/// A missing `workspace.json5` (or one missing the `description` field) yields
/// `description: None`. A syntactically malformed `workspace.json5` is treated
/// as corruption and surfaced as an error so the caller can inform the user.
///
/// # Errors
///
/// Returns an error if the workspace directory does not exist or if
/// `workspace.json5` exists but fails to parse.
pub fn load_workspace(name: &str) -> anyhow::Result<Workspace> {
    load_workspace_at(&workspaces_root()?, name)
}

/// Validates that `name` is acceptable as a workspace (and filesystem)
/// directory name.
///
/// Enforces Windows filesystem constraints regardless of the host OS so that
/// workspace names are portable across platforms.
///
/// # Errors
///
/// Returns a descriptive error for each class of invalid name.
pub fn validate_workspace_name(name: &str) -> anyhow::Result<()> {
    // Forbidden characters on Windows (and generally unsafe on filesystems).
    const FORBIDDEN: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*', '\0'];

    // Reserved Windows device names. Reject both bare names and names with any
    // extension (e.g. "CON.txt"), because Windows treats "CON.txt" the same as
    // "CON" when opening a path.
    const RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
        "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    if name.is_empty() || name.chars().all(char::is_whitespace) {
        anyhow::bail!("workspace name must not be empty or whitespace-only");
    }

    if name.chars().count() > 255 {
        anyhow::bail!("workspace name must not exceed 255 characters");
    }

    for ch in FORBIDDEN {
        if name.contains(*ch) {
            anyhow::bail!("workspace name contains forbidden character: {ch:?}");
        }
    }

    if name.starts_with(' ') || name.ends_with(' ') {
        anyhow::bail!("workspace name must not start or end with a space");
    }
    if name.starts_with('.') || name.ends_with('.') {
        anyhow::bail!("workspace name must not start or end with '.'");
    }

    // Strip any extension before comparing so "CON.txt" → "CON".
    let stem = name.split('.').next().unwrap_or(name);
    let stem_upper = stem.to_ascii_uppercase();
    for reserved in RESERVED {
        if stem_upper == *reserved {
            anyhow::bail!("workspace name {name:?} is a reserved Windows device name");
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Internal `*_at` forms — accept an explicit root for test injectability
// ---------------------------------------------------------------------------

pub(crate) fn list_workspaces_at(root: &Path) -> anyhow::Result<Vec<Workspace>> {
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut workspaces = Vec::new();

    let entries = std::fs::read_dir(root)
        .with_context(|| format!("failed to read workspaces directory {}", root.display()))?;

    for entry in entries {
        let entry =
            entry.with_context(|| format!("failed to read entry in {}", root.display()))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .with_context(|| {
                format!(
                    "workspace directory has a non-UTF-8 name: {}",
                    path.display()
                )
            })?
            .to_owned();

        let description = read_json5::<WorkspaceFile>(&workspace_file(root, &name))
            .ok()
            .and_then(|f| f.description);

        workspaces.push(Workspace { name, description });
    }

    workspaces.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(workspaces)
}

pub(crate) fn create_workspace_at(
    root: &Path,
    name: &str,
    description: Option<&str>,
) -> anyhow::Result<Workspace> {
    validate_workspace_name(name)?;

    let dir = workspace_dir(root, name);
    if dir.exists() {
        anyhow::bail!("workspace {name:?} already exists");
    }

    std::fs::create_dir_all(&dir)
        .with_context(|| format!("failed to create workspace directory {}", dir.display()))?;

    write_json5(
        &workspace_file(root, name),
        &WorkspaceFile {
            description: description.map(String::from),
        },
    )
    .with_context(|| format!("failed to write workspace.json5 for {name:?}"))?;

    Ok(Workspace {
        name: name.to_owned(),
        description: description.map(String::from),
    })
}

pub(crate) fn rename_workspace_at(root: &Path, old: &str, new: &str) -> anyhow::Result<Workspace> {
    validate_workspace_name(new)?;

    let src = workspace_dir(root, old);
    let dst = workspace_dir(root, new);

    if !src.exists() {
        anyhow::bail!("workspace {old:?} does not exist");
    }
    if dst.exists() {
        anyhow::bail!("workspace {new:?} already exists");
    }

    std::fs::rename(&src, &dst)
        .with_context(|| format!("failed to rename {old:?} to {new:?}"))?;

    // Re-read the description from the now-renamed directory.
    let description = read_json5::<WorkspaceFile>(&workspace_file(root, new))
        .ok()
        .and_then(|f| f.description);

    Ok(Workspace {
        name: new.to_owned(),
        description,
    })
}

pub(crate) fn delete_workspace_at(root: &Path, name: &str) -> anyhow::Result<()> {
    let dir = workspace_dir(root, name);
    if !dir.exists() {
        return Ok(());
    }

    std::fs::remove_dir_all(&dir)
        .with_context(|| format!("failed to remove workspace directory {}", dir.display()))
}

pub(crate) fn load_workspace_at(root: &Path, name: &str) -> anyhow::Result<Workspace> {
    let dir = workspace_dir(root, name);
    if !dir.is_dir() {
        anyhow::bail!("workspace {name:?} does not exist");
    }

    let file = workspace_file(root, name);
    let description = if file.exists() {
        read_json5::<WorkspaceFile>(&file)
            .with_context(|| format!("failed to parse workspace.json5 for {name:?}"))?
            .description
    } else {
        None
    };

    Ok(Workspace {
        name: name.to_owned(),
        description,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fake_home() -> TempDir {
        tempfile::tempdir().expect("failed to create temp dir")
    }

    // ------------------------------------------------------------------
    // list_workspaces
    // ------------------------------------------------------------------

    #[test]
    fn defaults_returned_when_workspace_json_missing() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        let dir = root.join("foo");
        std::fs::create_dir_all(&dir).unwrap();

        let list = list_workspaces_at(&root).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "foo");
        assert_eq!(list[0].description, None);
    }

    #[test]
    fn roundtrip_preserves_values() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        create_workspace_at(&root, "alpha", Some("my desc")).unwrap();
        let list = list_workspaces_at(&root).unwrap();

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "alpha");
        assert_eq!(list[0].description.as_deref(), Some("my desc"));

        // The file must contain "description" but NOT "name".
        let raw = std::fs::read_to_string(workspace_file(&root, "alpha")).unwrap();
        assert!(raw.contains("description"), "file should contain 'description': {raw}");
        assert!(!raw.contains("\"name\""), "file must not contain 'name' key: {raw}");
    }

    #[test]
    fn create_workspace_creates_dir_and_file() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        let ws = create_workspace_at(&root, "beta", Some("test")).unwrap();
        assert_eq!(ws.name, "beta");
        assert_eq!(ws.description.as_deref(), Some("test"));
        assert!(workspace_dir(&root, "beta").is_dir());
        assert!(workspace_file(&root, "beta").exists());
    }

    #[test]
    fn create_workspace_fails_when_already_exists() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        create_workspace_at(&root, "gamma", None).unwrap();
        let err = create_workspace_at(&root, "gamma", None).unwrap_err();
        assert!(
            format!("{err:#}").contains("already exists"),
            "unexpected error: {err:#}"
        );
    }

    #[test]
    fn list_workspaces_returns_empty_when_dir_missing() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        // root does not exist
        let list = list_workspaces_at(&root).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn list_workspaces_skips_files_only_dirs() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        std::fs::create_dir_all(&root).unwrap();

        // Create one proper workspace dir and one stray file.
        std::fs::create_dir_all(root.join("real-ws")).unwrap();
        std::fs::write(root.join("foo.txt"), b"stray").unwrap();

        let list = list_workspaces_at(&root).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "real-ws");
    }

    #[test]
    fn list_workspaces_returns_workspaces_in_alphabetical_order() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        for name in &["zebra", "apple", "mango"] {
            create_workspace_at(&root, name, None).unwrap();
        }

        let list = list_workspaces_at(&root).unwrap();
        let names: Vec<&str> = list.iter().map(|w| w.name.as_str()).collect();
        assert_eq!(names, vec!["apple", "mango", "zebra"]);
    }

    // ------------------------------------------------------------------
    // rename_workspace
    // ------------------------------------------------------------------

    #[test]
    fn rename_workspace_moves_dir_and_preserves_content() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        create_workspace_at(&root, "old-name", Some("preserved")).unwrap();
        let ws = rename_workspace_at(&root, "old-name", "new-name").unwrap();

        assert_eq!(ws.name, "new-name");
        assert_eq!(ws.description.as_deref(), Some("preserved"));
        assert!(!workspace_dir(&root, "old-name").exists());
        assert!(workspace_dir(&root, "new-name").is_dir());
    }

    #[test]
    fn rename_workspace_fails_if_target_exists() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        create_workspace_at(&root, "src", None).unwrap();
        create_workspace_at(&root, "dst", None).unwrap();

        let err = rename_workspace_at(&root, "src", "dst").unwrap_err();
        assert!(
            format!("{err:#}").contains("already exists"),
            "unexpected error: {err:#}"
        );
    }

    #[test]
    fn rename_workspace_fails_if_source_missing() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        std::fs::create_dir_all(&root).unwrap();

        let err = rename_workspace_at(&root, "ghost", "new").unwrap_err();
        assert!(
            format!("{err:#}").contains("does not exist"),
            "unexpected error: {err:#}"
        );
    }

    // ------------------------------------------------------------------
    // delete_workspace
    // ------------------------------------------------------------------

    #[test]
    fn delete_workspace_removes_dir_recursively() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        create_workspace_at(&root, "to-delete", Some("bye")).unwrap();
        // Drop a sub-file to confirm recursive deletion.
        let nested = workspace_dir(&root, "to-delete").join("nested").join("file.txt");
        std::fs::create_dir_all(nested.parent().unwrap()).unwrap();
        std::fs::write(&nested, b"content").unwrap();

        delete_workspace_at(&root, "to-delete").unwrap();
        assert!(!workspace_dir(&root, "to-delete").exists());
    }

    #[test]
    fn delete_workspace_idempotent_when_missing() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        std::fs::create_dir_all(&root).unwrap();

        // Should succeed even though the workspace was never created.
        delete_workspace_at(&root, "nonexistent").unwrap();
    }

    // ------------------------------------------------------------------
    // validate_workspace_name
    // ------------------------------------------------------------------

    #[test]
    fn validate_workspace_name_rejects_forbidden_chars() {
        let forbidden = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '\0'];
        for ch in forbidden {
            let name = format!("bad{ch}name");
            let result = validate_workspace_name(&name);
            assert!(
                result.is_err(),
                "expected error for char {ch:?}, but got Ok"
            );
            let msg = format!("{:#}", result.unwrap_err());
            assert!(
                msg.contains("forbidden character"),
                "unexpected message for char {ch:?}: {msg}"
            );
        }
    }

    #[test]
    fn validate_workspace_name_rejects_reserved_names() {
        let cases = [
            "CON", "con", "PRN", "prn.txt", "AUX", "AUX.log", "NUL", "COM1", "COM9", "LPT1",
            "LPT9",
        ];
        for name in cases {
            let result = validate_workspace_name(name);
            assert!(
                result.is_err(),
                "expected error for reserved name {name:?}, but got Ok"
            );
            let msg = format!("{:#}", result.unwrap_err());
            assert!(
                msg.contains("reserved"),
                "unexpected message for {name:?}: {msg}"
            );
        }
    }

    #[test]
    fn validate_workspace_name_rejects_leading_or_trailing_space_or_dot() {
        let cases = [" leading", "trailing ", ".leading-dot", "trailing-dot."];
        for name in cases {
            let result = validate_workspace_name(name);
            assert!(
                result.is_err(),
                "expected error for {name:?}, but got Ok"
            );
        }
    }

    #[test]
    fn validate_workspace_name_rejects_empty_or_whitespace_only() {
        for name in ["", "   ", "\t", "\n"] {
            let result = validate_workspace_name(name);
            assert!(
                result.is_err(),
                "expected error for whitespace-only name {name:?}, but got Ok"
            );
        }
    }

    // ------------------------------------------------------------------
    // load_workspace
    // ------------------------------------------------------------------

    #[test]
    fn load_workspace_returns_workspace_with_description() {
        let home = fake_home();
        let root = home.path().join("workspaces");

        create_workspace_at(&root, "delta", Some("hello")).unwrap();
        let ws = load_workspace_at(&root, "delta").unwrap();

        assert_eq!(ws.name, "delta");
        assert_eq!(ws.description.as_deref(), Some("hello"));
    }

    #[test]
    fn load_workspace_returns_none_description_when_json_file_missing() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        let dir = root.join("epsilon");
        std::fs::create_dir_all(&dir).unwrap();

        let ws = load_workspace_at(&root, "epsilon").unwrap();
        assert_eq!(ws.name, "epsilon");
        assert_eq!(ws.description, None);
    }

    #[test]
    fn load_workspace_fails_when_dir_missing() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        std::fs::create_dir_all(&root).unwrap();

        let err = load_workspace_at(&root, "ghost").unwrap_err();
        assert!(
            format!("{err:#}").contains("does not exist"),
            "unexpected error: {err:#}"
        );
    }

    #[test]
    fn load_workspace_fails_on_malformed_json5() {
        let home = fake_home();
        let root = home.path().join("workspaces");
        let dir = root.join("zeta");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(workspace_file(&root, "zeta"), b"{ not valid json5 ,,,").unwrap();

        let err = load_workspace_at(&root, "zeta").unwrap_err();
        assert!(
            format!("{err:#}").contains("workspace.json5"),
            "unexpected error: {err:#}"
        );
    }

    #[test]
    fn validate_workspace_name_rejects_over_255_chars() {
        let name = "a".repeat(256);
        let result = validate_workspace_name(&name);
        assert!(result.is_err(), "expected error for 256-char name");
        assert!(
            format!("{:#}", result.unwrap_err()).contains("255"),
            "error should mention the 255 limit"
        );
    }
}

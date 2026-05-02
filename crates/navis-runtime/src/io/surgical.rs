use std::ops::Range;

use anyhow::{Context as _, anyhow};
use serde_json::Value;
use tree_sitter::{Node, Tree};

/// Applies surgical edits to `source` so it reflects `new_value`, preserving
/// comments, key order, and whitespace from the original text wherever possible.
pub(super) fn apply(
    source: &str,
    tree: &Tree,
    old_value: &Value,
    new_value: &Value,
) -> anyhow::Result<String> {
    let mut edits: Vec<Edit> = Vec::new();
    diff_recursive(tree, source, &mut Vec::new(), old_value, new_value, &mut edits)?;

    // Sort in descending byte-range order so applying an edit doesn't shift
    // the offsets of edits that come before it in the source.
    edits.sort_by(|a, b| b.range.start.cmp(&a.range.start));

    let mut out = source.to_owned();
    for edit in edits {
        out.replace_range(edit.range, &edit.replacement);
    }
    Ok(out)
}

#[derive(Debug)]
struct Edit {
    range: Range<usize>,
    replacement: String,
}

/// Walks `old_value` and `new_value` in lockstep, emitting edits whenever a
/// leaf differs, a key was added, or a key was removed.
///
/// `path` accumulates the current key path for tree-sitter lookup.
fn diff_recursive(
    tree: &Tree,
    source: &str,
    path: &mut Vec<String>,
    old_value: &Value,
    new_value: &Value,
    edits: &mut Vec<Edit>,
) -> anyhow::Result<()> {
    match (old_value, new_value) {
        (Value::Object(old_map), Value::Object(new_map)) => {
            // Updates and removals: walk old keys.
            for (key, old_sub) in old_map {
                path.push(key.clone());
                if let Some(new_sub) = new_map.get(key) {
                    diff_recursive(tree, source, path, old_sub, new_sub, edits)?;
                } else {
                    let member_node = find_member_node(tree, source, path).ok_or_else(|| {
                        anyhow!("missing member node for path {path:?} during removal")
                    })?;
                    edits.push(Edit {
                        range: member_with_separator_range(member_node, source),
                        replacement: String::new(),
                    });
                }
                path.pop();
            }

            // Insertions: walk new keys absent from old.
            for (key, new_sub) in new_map {
                if !old_map.contains_key(key) {
                    path.push(key.clone());
                    let parent_path = &path[..path.len() - 1];
                    let parent_object_node =
                        find_object_node(tree, source, parent_path).ok_or_else(|| {
                            anyhow!(
                                "missing object node for path {parent_path:?} during insertion"
                            )
                        })?;
                    let insertion = build_insertion(parent_object_node, source, key, new_sub)?;
                    edits.push(insertion);
                    path.pop();
                }
            }
            Ok(())
        }
        (Value::Array(_), Value::Array(_)) if old_value == new_value => Ok(()),
        _ => {
            if old_value == new_value {
                return Ok(());
            }
            let value_node = find_value_node(tree, source, path)
                .ok_or_else(|| anyhow!("missing value node for path {path:?}"))?;
            let replacement = serde_json::to_string(new_value)
                .with_context(|| format!("failed to serialize replacement at {path:?}"))?;
            edits.push(Edit {
                range: value_node.byte_range(),
                replacement,
            });
            Ok(())
        }
    }
}

/// Returns the `member` node at the given path inside the object tree, or `None`
/// if any path segment doesn't resolve.
fn find_member_node<'tree>(
    tree: &'tree Tree,
    source: &str,
    path: &[String],
) -> Option<Node<'tree>> {
    if path.is_empty() {
        return None;
    }
    let mut current = root_object(tree)?;
    for (i, segment) in path.iter().enumerate() {
        let member = find_member_by_key(current, source, segment)?;
        if i == path.len() - 1 {
            return Some(member);
        }
        let value = member.child_by_field_name("value")?;
        if value.kind() != "object" {
            return None;
        }
        current = value;
    }
    None
}

fn find_value_node<'tree>(
    tree: &'tree Tree,
    source: &str,
    path: &[String],
) -> Option<Node<'tree>> {
    let member = find_member_node(tree, source, path)?;
    member.child_by_field_name("value")
}

fn find_object_node<'tree>(
    tree: &'tree Tree,
    source: &str,
    path: &[String],
) -> Option<Node<'tree>> {
    if path.is_empty() {
        return root_object(tree);
    }
    let value = find_value_node(tree, source, path)?;
    if value.kind() == "object" {
        Some(value)
    } else {
        None
    }
}

fn root_object(tree: &Tree) -> Option<Node<'_>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    root.children(&mut cursor).find(|c| c.kind() == "object")
}

fn find_member_by_key<'tree>(object: Node<'tree>, source: &str, key: &str) -> Option<Node<'tree>> {
    let mut cursor = object.walk();
    object.children(&mut cursor).find(|c| {
        if c.kind() != "member" {
            return false;
        }
        let Some(name_node) = c.child_by_field_name("name") else {
            return false;
        };
        member_key_text(name_node, source).as_deref() == Some(key)
    })
}

fn member_key_text(name_node: Node<'_>, source: &str) -> Option<String> {
    let raw = &source[name_node.byte_range()];
    match name_node.kind() {
        "identifier" => Some(raw.to_owned()),
        "string" => {
            // Strip outer quotes; safe because the tree is error-free at this point.
            raw.get(1..raw.len().saturating_sub(1)).map(str::to_owned)
        }
        _ => None,
    }
}

/// Range covering the member plus a trailing separator (comma + whitespace) so
/// the file stays well-formed after deletion. Falls back to just the member if
/// no trailing separator is found.
fn member_with_separator_range(member: Node<'_>, source: &str) -> Range<usize> {
    let start = member.start_byte();
    let mut end = member.end_byte();
    let bytes = source.as_bytes();
    while end < bytes.len() {
        let b = bytes[end];
        if b == b',' {
            end += 1;
            // Consume trailing whitespace up to and including one newline.
            while end < bytes.len() && (bytes[end] == b' ' || bytes[end] == b'\t') {
                end += 1;
            }
            if end < bytes.len() && bytes[end] == b'\n' {
                end += 1;
            }
            break;
        }
        if b == b'}' || !b.is_ascii_whitespace() {
            break;
        }
        end += 1;
    }
    start..end
}

/// Builds an insertion edit that adds `key: value` immediately before the
/// closing `}` of `object_node`, matching the indentation of the last member
/// if one exists.
fn build_insertion(
    object_node: Node<'_>,
    source: &str,
    key: &str,
    new_sub: &Value,
) -> anyhow::Result<Edit> {
    let mut cursor = object_node.walk();
    let close_brace = object_node
        .children(&mut cursor)
        .find(|c| c.kind() == "}")
        .ok_or_else(|| anyhow!("object node missing closing brace"))?;
    let insert_at = close_brace.start_byte();

    let line_start = source[..insert_at].rfind('\n').map_or(0, |n| n + 1);
    let close_indent = &source[line_start..insert_at];

    // Reuse the first member's indent level; fall back to close_indent + two spaces.
    let mut member_cursor = object_node.walk();
    let inner_indent = object_node
        .children(&mut member_cursor)
        .find(|c| c.kind() == "member")
        .and_then(|m| {
            let m_line_start = source[..m.start_byte()].rfind('\n').map(|n| n + 1)?;
            Some(source[m_line_start..m.start_byte()].to_owned())
        })
        .unwrap_or_else(|| format!("{close_indent}  "));

    let value_text = serde_json::to_string(new_sub)
        .with_context(|| format!("failed to serialize new value for key `{key}`"))?;
    let replacement = format!("{inner_indent}{key}: {value_text},\n{close_indent}");

    Ok(Edit {
        range: insert_at..insert_at,
        replacement,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use navis_parser::parse;

    fn run(source: &str, new: Value) -> String {
        let tree = parse(source).unwrap();
        let old: Value = json5::from_str(source).unwrap();
        apply(source, &tree, &old, &new).unwrap()
    }

    #[test]
    fn update_value_preserves_comments() {
        let src = "// hi\n{ a: 1 }";
        let new = serde_json::json!({ "a": 2 });
        let out = run(src, new);
        assert_eq!(out, "// hi\n{ a: 2 }");
    }

    #[test]
    fn update_preserves_trailing_comma() {
        let src = "{ a: 1, }";
        let new = serde_json::json!({ "a": 2 });
        let out = run(src, new);
        assert_eq!(out, "{ a: 2, }");
    }

    #[test]
    fn nested_update_preserves_outer() {
        let src = "{\n  // top\n  outer: {\n    inner: 1,\n  },\n}";
        let new = serde_json::json!({ "outer": { "inner": 2 } });
        let out = run(src, new);
        assert!(out.contains("// top"));
        assert!(out.contains("inner: 2"));
        assert!(!out.contains("inner: 1"));
    }

    #[test]
    fn no_changes_is_idempotent() {
        let src = "// k\n{ a: 1, b: 2 }";
        let same = serde_json::json!({ "a": 1, "b": 2 });
        let out = run(src, same);
        assert_eq!(out, src);
    }

    #[test]
    fn add_key_inserts_before_closing_brace() {
        let src = "{\n  a: 1,\n}";
        let new = serde_json::json!({ "a": 1, "b": 2 });
        let out = run(src, new);
        assert!(out.contains("a: 1"));
        assert!(out.contains("b: 2"));
    }

    #[test]
    fn remove_key_drops_member_and_separator() {
        let src = "{\n  a: 1,\n  b: 2,\n}";
        let new = serde_json::json!({ "a": 1 });
        let out = run(src, new);
        assert!(out.contains("a: 1"));
        assert!(!out.contains("b: 2"));
    }
}

use anyhow::Context as _;
use tree_sitter::{Parser, Tree};

/// Parses a JSON5 fragment and returns the tree-sitter `Tree`.
///
/// Tree-sitter is resilient: invalid input still produces an `Ok(Tree)` whose
/// root node reports `has_error() == true`. The `Err` of this function is
/// reserved for parser-level failures (timeout, cancellation, grammar load).
///
/// # Errors
///
/// Returns an error if the JSON5 grammar fails to load, or if tree-sitter
/// returns no tree (which only happens on explicit timeout or cancellation).
pub fn parse(input: &str) -> anyhow::Result<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_json5::language())
        .context("failed to load JSON5 grammar")?;
    parser
        .parse(input, None)
        .context("tree-sitter returned no tree (timeout or cancellation)")
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_JSON5: &str = r#"{
        // a comment
        name: "demo",
        version: 1,
        apis: [
            { slug: "users-api" }, // trailing comma OK
        ],
    }"#;

    const INVALID_JSON5: &str = r"{ key: }";

    #[test]
    fn parses_valid_json5_without_errors() {
        let tree = parse(VALID_JSON5).expect("parse should succeed");
        assert!(!tree.root_node().has_error(), "valid JSON5 should not contain ERROR nodes");
    }

    #[test]
    fn produces_tree_with_errors_on_invalid_input() {
        let tree = parse(INVALID_JSON5).expect("parse should succeed even for invalid syntax");
        assert!(tree.root_node().has_error(), "invalid JSON5 should produce ERROR nodes");
    }

    #[test]
    fn parser_loads_grammar_successfully() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_json5::language())
            .expect("grammar should load");
    }
}

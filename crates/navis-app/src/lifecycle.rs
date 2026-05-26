pub(crate) mod run_event;
pub(crate) mod single_instance;
pub(crate) mod window_event;

pub(crate) const WORKSPACE_LABEL_PREFIX: &str = "navis-";
pub(crate) const SELECTOR_LABEL: &str = "navis-ws-selector";
pub(crate) const SETTINGS_LABEL: &str = "navis-settings";

/// Returns the workspace name encoded in a window label, or `None` if the
/// label belongs to a non-workspace window (selector, settings, etc).
pub(crate) fn workspace_name_from_label(label: &str) -> Option<&str> {
    if label == SELECTOR_LABEL || label == SETTINGS_LABEL {
        return None;
    }
    label.strip_prefix(WORKSPACE_LABEL_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::workspace_name_from_label;

    #[test]
    fn extracts_workspace_name_from_prefixed_label() {
        assert_eq!(workspace_name_from_label("navis-production"), Some("production"));
        assert_eq!(workspace_name_from_label("navis-my workspace"), Some("my workspace"));
    }

    #[test]
    fn skips_non_workspace_labels() {
        assert_eq!(workspace_name_from_label("navis-ws-selector"), None);
        assert_eq!(workspace_name_from_label("navis-settings"), None);
        assert_eq!(workspace_name_from_label("other"), None);
    }
}

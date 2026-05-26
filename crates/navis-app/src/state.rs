use std::collections::HashSet;

use parking_lot::Mutex;

pub(crate) struct AppState {
    pub(crate) open_workspaces: Mutex<HashSet<String>>,
}

pub(crate) fn build() -> AppState {
    AppState {
        open_workspaces: Mutex::new(HashSet::new()),
    }
}

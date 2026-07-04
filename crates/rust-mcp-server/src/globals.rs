use std::{path::PathBuf, sync::OnceLock};

static WORKSPACE_ROOT: OnceLock<PathBuf> = OnceLock::new();
static DEFAULT_REGISTRY: OnceLock<String> = OnceLock::new();

pub fn set_workspace_root(root: impl Into<PathBuf>) {
    WORKSPACE_ROOT
        .set(root.into())
        .expect("Workspace root can only be set once");
}

/// Attempts to set the workspace root. Returns `true` if set successfully,
/// `false` if it was already set (e.g. via CLI argument).
pub fn try_set_workspace_root(root: impl Into<PathBuf>) -> bool {
    WORKSPACE_ROOT.set(root.into()).is_ok()
}

pub fn get_workspace_root() -> Option<&'static PathBuf> {
    WORKSPACE_ROOT.get()
}

pub fn set_default_registry(registry: String) {
    DEFAULT_REGISTRY
        .set(registry)
        .expect("Default registry can only be set once");
}

pub fn get_default_registry() -> Option<&'static str> {
    DEFAULT_REGISTRY.get().map(|s| s.as_str())
}
